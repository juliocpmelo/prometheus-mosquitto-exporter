use crate::constants;

use log::warn;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::fs;
use std::net::ToSocketAddrs;

#[derive(Serialize, Deserialize, Clone)]
pub struct Configuration {
    pub mqtt: Mqtt,
    pub service: Option<Service>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Service {
    pub listen: Option<String>,
    pub metrics_path: Option<String>,
    pub topic_listener: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Mqtt {
    pub auth: MQTTAuth,
    pub broker: String,
    pub ca_file: Option<String>,
    pub client_cert_file: Option<String>,
    pub client_key_file: Option<String>,
    pub client_id: Option<String>,
    pub insecure_ssl: Option<bool>,
    pub qos: Option<i32>,
    pub retry_interval: Option<u64>,
    pub timeout: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MQTTAuth {
    pub password: String,
    pub user: String,
}

impl std::fmt::Debug for Configuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Configuration")
            .field("mqtt", &self.mqtt)
            .field("service", &self.service)
            .finish()
    }
}

impl std::fmt::Debug for Service {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Service")
            .field("listen", &self.listen)
            .field("metrics_path", &self.metrics_path)
            .finish()
    }
}

impl std::fmt::Debug for Mqtt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Mqtt")
            .field("auth", &self.auth)
            .field("broker", &self.broker)
            .field("ca_file", &self.ca_file)
            .field("client_cert_file", &self.client_cert_file)
            .field("client_key_file", &self.client_key_file)
            .field("client_id", &self.client_id)
            .field("insecure_ssl", &self.insecure_ssl)
            .field("qos", &self.qos)
            .field("retry_interval", &self.retry_interval)
            .field("timeout", &self.timeout)
            .finish()
    }
}

impl std::fmt::Debug for MQTTAuth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MQTTAuth")
            .field("password", &self.password)
            .field("user", &self.user)
            .finish()
    }
}

pub fn parse_config_file(f: &str) -> Result<Configuration, Box<dyn Error>> {
    let raw = fs::read_to_string(f)?;
    let mut result: Configuration = match serde_yaml::from_str(raw.as_str()) {
        Ok(v) => v,
        Err(e) => return Err(Box::new(e)),
    };

    // Fill missing fields with default settings
    result.service = match result.service {
        Some(mut v) => {
            v.listen = match v.listen {
                Some(v) => Some(v),
                None => Some(constants::DEFAULT_LISTEN.to_string()),
            };
            v.metrics_path = match v.metrics_path {
                Some(v) => Some(v),
                None => Some(constants::DEFAULT_METRICS_PATH.to_string()),
            };
            v.topic_listener = match v.topic_listener {
                Some(v) => Some(v),
                None => Some(constants::DEFAULT_TOPIC_LISTENER.to_string()),
            };
            Some(v)
        }
        None => Some(Service {
            listen: Some(constants::DEFAULT_LISTEN.to_string()),
            metrics_path: Some(constants::DEFAULT_METRICS_PATH.to_string()),
            topic_listener: Some(constants::DEFAULT_TOPIC_LISTENER.to_string()),
        }),
    };

    result.mqtt.ca_file = match result.mqtt.ca_file {
        Some(v) => Some(v),
        None => Some(constants::DEFAULT_MQTT_CA_FILE.to_string()),
    };
    result.mqtt.insecure_ssl = match result.mqtt.insecure_ssl {
        Some(v) => Some(v),
        None => Some(false),
    };
    result.mqtt.client_id = match result.mqtt.client_id {
        Some(v) => Some(v),
        None => Some(constants::DEFAULT_MQTT_CLIENT_ID.to_string()),
    };
    result.mqtt.retry_interval = match result.mqtt.retry_interval {
        Some(v) => Some(v),
        None => Some(constants::DEFAULT_MQTT_RETRY_INTERVAL),
    };
    result.mqtt.timeout = match result.mqtt.timeout {
        Some(v) => Some(v),
        None => Some(constants::DEFAULT_MQTT_TIMEOUT),
    };
    result.mqtt.qos = match result.mqtt.qos {
        Some(v) => Some(v),
        None => Some(constants::DEFAULT_MQTT_QOS),
    };

    match validate_config(&result) {
        Ok(_) => {}
        Err(e) => bail!(e),
    }

    Ok(result)
}

fn validate_config(cfg: &Configuration) -> Result<(), Box<dyn Error>> {
    if cfg.mqtt.broker.is_empty() {
        bail!("MQTT broker is empty");
    }
    if !cfg.mqtt.broker.starts_with("tcp://") && !cfg.mqtt.broker.starts_with("ssl://") {
        bail!("invalid broker protocol (only tcp:// or ssl:// are supported)");
    }

    if let Some(v) = cfg.mqtt.qos {
        if v < 0 && v > 2 {
            bail!("invalid QoS value, only 0, 1 or 2 are valid QoS values");
        }
    }

    if let Some(v) = &cfg.mqtt.client_id {
        if v.is_empty() {
            bail!("MQTT client id can't be empty");
        }
        if v.len() > 23 {
            // MQTT 5 and 3.1.1 specification require the MQTT broker to accept at least 23 bytes
            // for the client id. It's up to the broker to accept more than 23 bytes.
            // see: https://docs.oasis-open.org/mqtt/mqtt/v5.0/os/mqtt-v5.0-os.html#_Toc3901059
            warn!("MQTT 5 and 3.1.1 specification set a (optional) limit of 23 UTF-8 chars for the client id");
        }
    }

    if let Some(svc) = &cfg.service {
        if let Some(v) = &svc.metrics_path {
            if v.is_empty() {
                bail!("path for metrics exposure can't be empty");
            }
        }
        if let Some(v) = &svc.metrics_path {
            if v == "/" {
                bail!("/ can't be uses as metrics path");
            }
        }
        if let Some(v) = &svc.listen {
            if v.is_empty() {
                bail!("listener address can't be empty");
            }
        }
    }
    Ok(())
}

pub fn socketaddr_from_listen(listen: String) -> Result<std::net::SocketAddr, Box<dyn Error>> {
    let sockaddrs = listen.to_socket_addrs()?;
    let addresses: Vec<_> = sockaddrs.collect();
    if addresses.is_empty() {
        bail!("can't resolve listener address");
    }
    Ok(addresses[0])
}

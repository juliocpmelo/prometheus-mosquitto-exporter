use crate::config;
use crate::constants;

use core::time::Duration;
use log::{error, info};
use std::error::Error;
use std::sync::mpsc;

pub fn start_mqtt_client(
    cfg: &config::Configuration,
    sender: mpsc::Sender<paho_mqtt::message::Message>,
) -> Result<(), Box<dyn Error>> {
    let client_id = match &cfg.mqtt.client_id {
        Some(v) => v.clone(),
        None => panic!("BUG: mqtt::start_mqtt_client: client_id is undefined"),
    };
    let broker = cfg.mqtt.broker.clone();
    let ca_file = match &cfg.mqtt.ca_file {
        Some(v) => v.clone(),
        None => panic!("BUG: mqtt::start_mqtt_client: ca_file is undefined"),
    };
    let check_ssl = match cfg.mqtt.insecure_ssl {
        Some(v) => !v,
        None => panic!("BUG: mqtt::start_mqtt_client: insecure_ssl is undefined"),
    };
    let connect_timeout = match cfg.mqtt.timeout {
        Some(v) => v,
        None => panic!("BUG: mqtt::start_mqtt_client: timeout is undefined"),
    };
    let retry_interval = match cfg.mqtt.retry_interval {
        Some(v) => v,
        None => panic!("BUG: mqtt::start_mqtt_client: retry_interval is undefined"),
    };
    let qos = match cfg.mqtt.qos {
        Some(v) => v,
        None => panic!("BUG: mqtt::start_mqtt_client: qos is undefined"),
    };

    let mqtt_opts = paho_mqtt::CreateOptionsBuilder::new()
        .server_uri(broker)
        .client_id(client_id)
        .persistence(None)
        .finalize();

    let mut ssl_opts = paho_mqtt::SslOptionsBuilder::new();
    ssl_opts.trust_store(ca_file)?;
    ssl_opts.verify(check_ssl);
    ssl_opts.enable_server_cert_auth(check_ssl);
    let ssl_opts = ssl_opts.finalize();

    let conn_opts = paho_mqtt::ConnectOptionsBuilder::new()
        .ssl_options(ssl_opts)
        .user_name(cfg.mqtt.auth.user.clone())
        .password(cfg.mqtt.auth.password.clone())
        .connect_timeout(Duration::from_secs(connect_timeout))
        .automatic_reconnect(Duration::from_secs(1), Duration::from_secs(retry_interval))
        .finalize();

    let mut mqtt_client = paho_mqtt::client::Client::new(mqtt_opts)?;
    let messages = mqtt_client.start_consuming();

    let connection = mqtt_client.connect(conn_opts)?;
    if let Some(response) = connection.connect_response() {
        info!("Connected to MQTT broker {}", response.server_uri);

        if !response.session_present {
            info!("Subscribing to {}", constants::MOSQUITTO_STATISTICS_TOPIC);
            mqtt_client.subscribe(constants::MOSQUITTO_STATISTICS_TOPIC, qos)?;
            info!(
                "Subscribed to {} on {}",
                constants::MOSQUITTO_STATISTICS_TOPIC,
                cfg.mqtt.broker
            );
        };
    };

    // Message receiving loop
    for msg in messages.iter().flatten() {
        info!("MQTT statistics received at {}", msg.topic());
        match sender.send(msg) {
            Ok(_) => {}
            Err(e) => {
                error!("Can't send received payload to data channel: {}", e);

                info!(
                    "Unsubscribing from topic {}",
                    constants::MOSQUITTO_STATISTICS_TOPIC
                );
                mqtt_client.unsubscribe(constants::MOSQUITTO_STATISTICS_TOPIC)?;

                info!("Disconnecting from MQTT broker {}", cfg.mqtt.broker);
                let disco_opts = paho_mqtt::DisconnectOptionsBuilder::new()
                    .timeout(Duration::from_secs(connect_timeout))
                    .finalize();
                mqtt_client.disconnect(disco_opts)?;
            }
        };
    }

    Ok(())
}
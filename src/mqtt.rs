use crate::config;
use crate::constants;

use core::time::Duration;
use log::{error, info, warn};
use std::sync::mpsc;

pub fn start_mqtt_client(cfg: &config::MQTT, sender: mpsc::Sender<paho_mqtt::message::Message>) {
    let client_id = match &cfg.client_id {
        Some(v) => v.clone(),
        None => panic!("BUG: mqtt::start_mqtt_client: client_id is undefined"),
    };
    let broker = cfg.broker.clone();
    let ca_file = match &cfg.ca_file {
        Some(v) => v.clone(),
        None => panic!("BUG: mqtt::start_mqtt_client: ca_file is undefined"),
    };
    let check_ssl = match cfg.insecure_ssl {
        Some(v) => !v,
        None => panic!("BUG: mqtt::start_mqtt_client: insecure_ssl is undefined"),
    };
    let connect_timeout = match cfg.timeout {
        Some(v) => v,
        None => panic!("BUG: mqtt::start_mqtt_client: timeout is undefined"),
    };
    let retry_interval = match cfg.retry_interval {
        Some(v) => v,
        None => panic!("BUG: mqtt::start_mqtt_client: retry_interval is undefined"),
    };
    let qos = match cfg.qos {
        Some(v) => v,
        None => panic!("BUG: mqtt::start_mqtt_client: qos is undefined"),
    };

    let mqtt_opts = paho_mqtt::CreateOptionsBuilder::new()
        .server_uri(broker)
        .client_id(client_id)
        .persistence(None)
        .finalize();

    let mut ssl_opts = paho_mqtt::SslOptionsBuilder::new();
    match ssl_opts.trust_store(ca_file) {
        Ok(_) => {}
        Err(e) => {
            error!("Can't add CA file to trust store: {}", e);
            return;
        }
    };
    ssl_opts.verify(check_ssl);
    ssl_opts.enable_server_cert_auth(check_ssl);
    let ssl_opts = ssl_opts.finalize();

    let conn_opts = paho_mqtt::ConnectOptionsBuilder::new()
        .ssl_options(ssl_opts)
        .user_name(cfg.auth.user.clone())
        .password(cfg.auth.password.clone())
        .connect_timeout(Duration::from_secs(connect_timeout))
        .automatic_reconnect(Duration::from_secs(1), Duration::from_secs(retry_interval))
        .finalize();

    let mut mqtt_client = match paho_mqtt::client::Client::new(mqtt_opts) {
        Ok(v) => v,
        Err(e) => {
            error!("Can't create new MQTT client structure: {}", e);
            return;
        }
    };
    let messages = mqtt_client.start_consuming();

    let connection = match mqtt_client.connect(conn_opts) {
        Ok(v) => v,
        Err(e) => {
            error!("Can't create MQTT connection: {}", e);
            return;
        }
    };
    if let Some(response) = connection.connect_response() {
        info!("Connected to MQTT broker {}", response.server_uri);

        if !response.session_present {
            info!(
                "Subscribing to topic {} on {}",
                constants::MOSQUITTO_STATISTICS_TOPIC,
                cfg.broker
            );
            match mqtt_client.subscribe(constants::MOSQUITTO_STATISTICS_TOPIC, qos) {
                Ok(_) => info!(
                    "Subscribed to topic {}",
                    constants::MOSQUITTO_STATISTICS_TOPIC
                ),
                Err(e) => error!(
                    "Can't subscribe to topic {}: {}",
                    constants::MOSQUITTO_STATISTICS_TOPIC,
                    e
                ),
            };
        };
    };

    // Message receiving loop
    for msg in messages.iter().flatten() {
        match sender.send(msg) {
            Ok(_) => {}
            Err(e) => {
                error!("Can't send received payload to data channel: {}", e);

                info!(
                    "Unsubscribing from topic {}",
                    constants::MOSQUITTO_STATISTICS_TOPIC
                );
                match mqtt_client.unsubscribe(constants::MOSQUITTO_STATISTICS_TOPIC) {
                    Ok(_) => info!(
                        "Unsubscribing from topic {}",
                        constants::MOSQUITTO_STATISTICS_TOPIC
                    ),
                    Err(e) => warn!(
                        "Can't unsubscripbe from topic {}: {}",
                        constants::MOSQUITTO_STATISTICS_TOPIC,
                        e
                    ),
                };

                info!("Disconnecting from MQTT broker {}", cfg.broker);
                let disco_opts = paho_mqtt::DisconnectOptionsBuilder::new()
                    .timeout(Duration::from_secs(connect_timeout))
                    .finalize();
                match mqtt_client.disconnect(disco_opts) {
                    Ok(_) => info!("Disconnecting from MQTT broker"),
                    Err(e) => warn!("Can't disconnect from MQTT broker: {}", e),
                };
            }
        };
    }
}

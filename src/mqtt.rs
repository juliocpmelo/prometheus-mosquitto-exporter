use crate::config;
use crate::constants;

use core::time::Duration;
use log::{error, info, warn};
use std::process;
use std::sync::mpsc;
use std::str;

pub fn start_mqtt_client(mqtt_cfg: &config::Mqtt, service_cfg: config::Service, sender: mpsc::Sender<paho_mqtt::message::Message>) {
    let client_id = match &mqtt_cfg.client_id {
        Some(v) => v.clone(),
        None => panic!("BUG: mqtt::start_mqtt_client: client_id is undefined"),
    };
    let broker = mqtt_cfg.broker.clone();
    let ca_file = match &mqtt_cfg.ca_file {
        Some(v) => v.clone(),
        None => panic!("BUG: mqtt::start_mqtt_client: ca_file is undefined"),
    };
    let client_cert_file = match &mqtt_cfg.client_cert_file {
        Some(v) => v.clone(),
        None => panic!("BUG: mqtt::start_mqtt_client: client_cert_file is undefined"),
    }; 
    let client_key_file = match &mqtt_cfg.client_key_file {
        Some(v) => v.clone(),
        None => panic!("BUG: mqtt::start_mqtt_client: client_key_file is undefined"),
    }; 
    let check_ssl = match mqtt_cfg.insecure_ssl {
        Some(v) => !v,
        None => panic!("BUG: mqtt::start_mqtt_client: insecure_ssl is undefined"),
    };
    let connect_timeout = match mqtt_cfg.timeout {
        Some(v) => v,
        None => panic!("BUG: mqtt::start_mqtt_client: timeout is undefined"),
    };
    let retry_interval = match mqtt_cfg.retry_interval {
        Some(v) => v,
        None => panic!("BUG: mqtt::start_mqtt_client: retry_interval is undefined"),
    };
    let qos = match mqtt_cfg.qos {
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
    if !client_cert_file.trim().is_empty() && !client_key_file.trim().is_empty() {
        match ssl_opts.key_store(client_cert_file) {
            Ok(_) => {}
            Err(e) => {
                error!("Can't add client's cert file: {}", e);
                return;
            }
        };
        match ssl_opts.private_key(client_key_file) {
            Ok(_) => {}
            Err(e) => {
                error!("Can't add client's cert file: {}", e);
                return;
            }
        };
    }
    else if !client_cert_file.trim().is_empty() || !client_key_file.trim().is_empty() {
        error!("BUG: mqtt::start_mqtt_client: when defined, both client_cert_file and client_key_file must be non empty");
        return;
    }

    ssl_opts.verify(check_ssl);
    ssl_opts.enable_server_cert_auth(check_ssl);
    let ssl_opts = ssl_opts.finalize();

    let conn_opts = paho_mqtt::ConnectOptionsBuilder::new()
        .ssl_options(ssl_opts)
        .user_name(mqtt_cfg.auth.user.clone())
        .password(mqtt_cfg.auth.password.clone())
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

    let topic_listener = service_cfg.topic_listener.as_ref().unwrap();

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
                mqtt_cfg.broker
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
            /*solarz device monitor topic*/
            match mqtt_client.subscribe(topic_listener.as_str(), qos) {
                Ok(_) => info!(
                    "Subscribed to topic {}",
                    topic_listener
                ),
                Err(e) => error!(
                    "Can't subscribe to topic {}: {}",
                    topic_listener,
                    e
                ),
            };
        };
    };

    // Message receiving loop
    let mut subscribed_topics : Vec<String> = Vec::new();
    for msg in messages.iter().flatten() {
        
        let topic = msg.topic();
        if topic == topic_listener {
            
            let topics_json = match str::from_utf8(msg.payload()) {
                Ok(v) => v,
                Err(e) => {
                    warn!(
                        "Can't convert payload from {} to a UTF-8 string: {}",
                        topic, e
                    );
                    continue; //ignore this message and skip to the next one
                }
            };
            let topics: Vec<String> = match serde_json::from_str(topics_json) {
                Ok(v) => v,
                Err(e) => {
                    warn!("Error in json parsing {} : {}", topics_json, e);
                    continue; //ignore this message and skip to the next one
                }
            };
        
            for t in topics.iter(){
                if subscribed_topics.contains(t) { //avoid resubscriptions
                    continue;
                }
                match mqtt_client.subscribe(t, qos) {
                    Ok(_) => {
                        info!("Subscribed to topic {}",t);
                        subscribed_topics.push(String::from(t));
                    },
                    Err(e) => error!(
                        "Can't subscribe to topic {}: {}",
                        t,
                        e
                    ),
                };
            }

            let mut unsubscribe : Vec<String> = Vec::new();
            for t in subscribed_topics.iter(){
                if !topics.contains(t) {
                    match mqtt_client.unsubscribe(t) {
                        Ok(_) => {
                            info!("unsubscribed from topic {}",t);
                            unsubscribe.push(String::from(t));
                        },
                        Err(e) => error!(
                            "Can't subscribe to topic {}: {}",
                            t,
                            e
                        ),
                    };
                }
            }
            subscribed_topics.retain(|element| !unsubscribe.contains(element));
            info!("Monitoring topics {:?}", subscribed_topics);

            

        };


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

                

                info!("Disconnecting from MQTT broker {}", mqtt_cfg.broker);
                let disco_opts = paho_mqtt::DisconnectOptionsBuilder::new()
                    .timeout(Duration::from_secs(connect_timeout))
                    .finalize();
                match mqtt_client.disconnect(disco_opts) {
                    Ok(_) => info!("Disconnecting from MQTT broker"),
                    Err(e) => warn!("Can't disconnect from MQTT broker: {}", e),
                };
                process::exit(1);
            }
        };
    }
}

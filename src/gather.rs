use crate::constants;

use log::{error, info, warn};
use prometheus::{Gauge, IntCounter, IntGauge, Registry};
use std::str;
use std::sync::mpsc;

lazy_static::lazy_static! {
    pub static ref REGISTRY: Registry = Registry::new();

    // Gauges - integer
    pub static ref CONNECTED_CLIENTS: IntGauge = IntGauge::new(constants::CONNECTED_CLIENTS_NAME, constants::CONNECTED_CLIENTS_DESC).unwrap();
    pub static ref EXPIRED_CLIENTS: IntGauge = IntGauge::new(constants::EXPIRED_CLIENTS_NAME, constants::EXPIRED_CLIENTS_DESC).unwrap();
    pub static ref MAXIMUM_CLIENTS: IntGauge = IntGauge::new(constants::MAXIMUM_CLIENTS_NAME, constants::MAXIMUM_CLIENTS_DESC).unwrap();
    pub static ref CURRENT_HEAP_SIZE: IntGauge = IntGauge::new(constants::CURRENT_HEAP_SIZE_NAME, constants::CURRENT_HEAP_SIZE_DESC).unwrap();
    pub static ref MAXIMUM_HEAP_SIZE: IntGauge = IntGauge::new(constants::MAXIMUM_HEAP_SIZE_NAME, constants::MAXIMUM_HEAP_SIZE_DESC).unwrap();
    pub static ref MESSAGES_INFLIGHT: IntGauge = IntGauge::new(constants::MESSAGES_INFLIGHT_NAME, constants::MESSAGES_INFLIGHT_DESC).unwrap();
    pub static ref STORE_MESSAGES_COUNT: IntGauge = IntGauge::new(constants::STORE_MESSAGES_COUNT_NAME, constants::STORE_MESSAGES_COUNT_DESC).unwrap();
    pub static ref STORE_MESSAGES_BYTES: IntGauge = IntGauge::new(constants::STORE_MESSAGES_BYTES_NAME, constants::STORE_MESSAGES_BYTES_DESC).unwrap();
    pub static ref SUBSCRIPTIONS: IntGauge = IntGauge::new(constants::SUBSCRIPTIONS_NAME, constants::SUBSCRIPTIONS_DESC).unwrap();

    // Gauges - float
    pub static ref CONNECTIONS_1MIN: Gauge = Gauge::new(constants::CONNECTIONS_1MIN_NAME, constants::CONNECTIONS_1MIN_DESC).unwrap();
    pub static ref CONNECTIONS_5MIN: Gauge = Gauge::new(constants::CONNECTIONS_5MIN_NAME, constants::CONNECTIONS_5MIN_DESC).unwrap();
    pub static ref CONNECTIONS_15MIN: Gauge = Gauge::new(constants::CONNECTIONS_15MIN_NAME, constants::CONNECTIONS_15MIN_DESC).unwrap();
    pub static ref BYTES_RECEIVE_1MIN: Gauge = Gauge::new(constants::BYTES_RECEIVE_1MIN_NAME, constants::BYTES_RECEIVE_1MIN_DESC).unwrap();
    pub static ref BYTES_RECEIVE_5MIN: Gauge = Gauge::new(constants::BYTES_RECEIVE_5MIN_NAME, constants::BYTES_RECEIVE_5MIN_DESC).unwrap();
    pub static ref BYTES_RECEIVE_15MIN: Gauge = Gauge::new(constants::BYTES_RECEIVE_15MIN_NAME, constants::BYTES_RECEIVE_15MIN_DESC).unwrap();
    pub static ref BYTES_TRANSMIT_1MIN: Gauge = Gauge::new(constants::BYTES_TRANSMIT_1MIN_NAME, constants::BYTES_TRANSMIT_1MIN_DESC).unwrap();
    pub static ref BYTES_TRANSMIT_5MIN: Gauge = Gauge::new(constants::BYTES_TRANSMIT_5MIN_NAME, constants::BYTES_TRANSMIT_5MIN_DESC).unwrap();
    pub static ref BYTES_TRANSMIT_15MIN: Gauge = Gauge::new(constants::BYTES_TRANSMIT_15MIN_NAME, constants::BYTES_TRANSMIT_15MIN_DESC).unwrap();
    pub static ref MESSAGES_RECEIVE_1MIN: Gauge = Gauge::new(constants::MESSAGES_RECEIVE_1MIN_NAME, constants::MESSAGES_RECEIVE_1MIN_DESC).unwrap();
    pub static ref MESSAGES_RECEIVE_5MIN: Gauge = Gauge::new(constants::MESSAGES_RECEIVE_5MIN_NAME, constants::MESSAGES_RECEIVE_5MIN_DESC).unwrap();
    pub static ref MESSAGES_RECEIVE_15MIN: Gauge = Gauge::new(constants::MESSAGES_RECEIVE_15MIN_NAME, constants::MESSAGES_RECEIVE_15MIN_DESC).unwrap();
    pub static ref MESSAGES_TRANSMIT_1MIN: Gauge = Gauge::new(constants::MESSAGES_TRANSMIT_1MIN_NAME, constants::MESSAGES_TRANSMIT_1MIN_DESC).unwrap();
    pub static ref MESSAGES_TRANSMIT_5MIN: Gauge = Gauge::new(constants::MESSAGES_TRANSMIT_5MIN_NAME, constants::MESSAGES_TRANSMIT_5MIN_DESC).unwrap();
    pub static ref MESSAGES_TRANSMIT_15MIN: Gauge = Gauge::new(constants::MESSAGES_TRANSMIT_15MIN_NAME, constants::MESSAGES_TRANSMIT_15MIN_DESC).unwrap();
    pub static ref PUBLISH_DROPPED_1MIN: Gauge = Gauge::new(constants::PUBLISH_DROPPED_1MIN_NAME, constants::PUBLISH_DROPPED_1MIN_DESC).unwrap();
    pub static ref PUBLISH_DROPPED_5MIN: Gauge = Gauge::new(constants::PUBLISH_DROPPED_5MIN_NAME, constants::PUBLISH_DROPPED_5MIN_DESC).unwrap();
    pub static ref PUBLISH_DROPPED_15MIN: Gauge = Gauge::new(constants::PUBLISH_DROPPED_15MIN_NAME, constants::PUBLISH_DROPPED_15MIN_DESC).unwrap();
    pub static ref PUBLISH_RECEIVE_1MIN: Gauge = Gauge::new(constants::PUBLISH_RECEIVE_1MIN_NAME, constants::PUBLISH_RECEIVE_1MIN_DESC).unwrap();
    pub static ref PUBLISH_RECEIVE_5MIN: Gauge = Gauge::new(constants::PUBLISH_RECEIVE_5MIN_NAME, constants::PUBLISH_RECEIVE_5MIN_DESC).unwrap();
    pub static ref PUBLISH_RECEIVE_15MIN: Gauge = Gauge::new(constants::PUBLISH_RECEIVE_15MIN_NAME, constants::PUBLISH_RECEIVE_15MIN_DESC).unwrap();
    pub static ref PUBLISH_TRANSMIT_1MIN: Gauge = Gauge::new(constants::PUBLISH_TRANSMIT_1MIN_NAME, constants::PUBLISH_TRANSMIT_1MIN_DESC).unwrap();
    pub static ref PUBLISH_TRANSMIT_5MIN: Gauge = Gauge::new(constants::PUBLISH_TRANSMIT_5MIN_NAME, constants::PUBLISH_TRANSMIT_5MIN_DESC).unwrap();
    pub static ref PUBLISH_TRANSMIT_15MIN: Gauge = Gauge::new(constants::PUBLISH_TRANSMIT_15MIN_NAME, constants::PUBLISH_TRANSMIT_15MIN_DESC).unwrap();
    pub static ref SOCKETS_1MIN: Gauge = Gauge::new(constants::SOCKETS_1MIN_NAME, constants::SOCKETS_1MIN_DESC).unwrap();
    pub static ref SOCKETS_5MIN: Gauge = Gauge::new(constants::SOCKETS_5MIN_NAME, constants::SOCKETS_5MIN_DESC).unwrap();
    pub static ref SOCKETS_15MIN: Gauge = Gauge::new(constants::SOCKETS_15MIN_NAME, constants::SOCKETS_15MIN_DESC).unwrap();

    // Counters
    pub static ref RECEIVE_BYTES: IntCounter = IntCounter::new(constants::RECEIVE_BYTES_NAME, constants::RECEIVE_BYTES_DESC).unwrap();
    pub static ref TRANSMIT_BYTES: IntCounter = IntCounter::new(constants::TRANSMIT_BYTES_NAME, constants::TRANSMIT_BYTES_DESC).unwrap();
    pub static ref DISCONNECTED_CLIENTS: IntCounter = IntCounter::new(constants::DISCONNECTED_CLIENTS_NAME, constants::DISCONNECTED_CLIENTS_DESC).unwrap();
    pub static ref TOTAL_CLIENTS: IntCounter = IntCounter::new(constants::TOTAL_CLIENTS_NAME, constants::TOTAL_CLIENTS_DESC).unwrap();
    pub static ref MESSAGES_RECEIVE: IntCounter = IntCounter::new(constants::MESSAGES_RECEIVE_NAME, constants::MESSAGES_RECEIVE_DESC).unwrap();
    pub static ref MESSAGES_TRANSMIT: IntCounter = IntCounter::new(constants::MESSAGES_TRANSMIT_NAME, constants::MESSAGES_TRANSMIT_DESC).unwrap();
    pub static ref PUBLISH_DROPPED: IntCounter = IntCounter::new(constants::PUBLISH_DROPPED_NAME, constants::PUBLISH_DROPPED_DESC).unwrap();
    pub static ref PUBLISH_RECEIVE: IntCounter = IntCounter::new(constants::PUBLISH_RECEIVE_NAME, constants::PUBLISH_RECEIVE_DESC).unwrap();
    pub static ref PUBLISH_TRANSMIT: IntCounter = IntCounter::new(constants::PUBLISH_TRANSMIT_NAME, constants::PUBLISH_TRANSMIT_DESC).unwrap();
    pub static ref PUBLISH_BYTES_RECEIVE: IntCounter = IntCounter::new(constants::PUBLISH_BYTES_RECEIVE_NAME, constants::PUBLISH_BYTES_RECEIVE_DESC).unwrap();
    pub static ref PUBLISH_BYTES_TRANSMIT: IntCounter = IntCounter::new(constants::PUBLISH_BYTES_TRANSMIT_NAME, constants::PUBLISH_BYTES_TRANSMIT_DESC).unwrap();
    pub static ref MESSAGES_RETAINED: IntCounter = IntCounter::new(constants::MESSAGES_RETAINED_NAME, constants::MESSAGES_RETAINED_DESC).unwrap();
}

pub fn run(receiver: mpsc::Receiver<paho_mqtt::message::Message>, teardown: mpsc::Receiver<bool>) {
    register_metrics();

    loop {
        if teardown.try_recv().is_ok() {
            info!("Stopping data collector thread");
            return;
        };

        let raw_msg = receiver.recv();
        let msg = match raw_msg {
            Ok(v) => v,
            Err(e) => {
                error!("Can't receive MQTT message: {}", e);
                return;
            }
        };
        let topic = msg.topic();

        let raw_data = msg.payload();
        let payload = match str::from_utf8(raw_data) {
            Ok(v) => v,
            Err(e) => {
                warn!(
                    "Can't convert payload from {} to a UTF-8 string: {}",
                    topic, e
                );
                continue;
            }
        };
        process_data(topic, payload)
    }
}

fn process_data(topic: &str, data: &str) {
    match topic {
        constants::MOSQUITTO_UPTIME_TOPIC => {}
        constants::MOSQUITTO_VERSION_TOPIC => {}
        constants::BYTES_RECEIVE_1MIN_TOPIC => {
            match data.parse::<f64>() {
                Ok(v) => BYTES_RECEIVE_1MIN.set(v),
                Err(e) => warn!(
                    "Can't convert data from {} to floating number: {}",
                    topic, e
                ),
            };
        }
        constants::BYTES_RECEIVE_5MIN_TOPIC => {
            match data.parse::<f64>() {
                Ok(v) => BYTES_RECEIVE_5MIN.set(v),
                Err(e) => warn!(
                    "Can't convert data from {} to floating number: {}",
                    topic, e
                ),
            };
        }
        constants::BYTES_RECEIVE_15MIN_TOPIC => {
            match data.parse::<f64>() {
                Ok(v) => BYTES_RECEIVE_15MIN.set(v),
                Err(e) => warn!(
                    "Can't convert data from {} to floating number: {}",
                    topic, e
                ),
            };
        }
        constants::BYTES_TRANSMIT_1MIN_TOPIC => {
            match data.parse::<f64>() {
                Ok(v) => BYTES_TRANSMIT_1MIN.set(v),
                Err(e) => warn!(
                    "Can't convert data from {} to floating number: {}",
                    topic, e
                ),
            };
        }
        constants::BYTES_TRANSMIT_5MIN_TOPIC => {
            match data.parse::<f64>() {
                Ok(v) => BYTES_TRANSMIT_5MIN.set(v),
                Err(e) => warn!(
                    "Can't convert data from {} to floating number: {}",
                    topic, e
                ),
            };
        }
        constants::BYTES_TRANSMIT_15MIN_TOPIC => {
            match data.parse::<f64>() {
                Ok(v) => BYTES_TRANSMIT_15MIN.set(v),
                Err(e) => warn!(
                    "Can't convert data from {} to floating number: {}",
                    topic, e
                ),
            };
        }
        constants::CONNECTED_CLIENTS_TOPIC => {
            match data.parse::<i64>() {
                Ok(v) => CONNECTED_CLIENTS.set(v),
                Err(e) => warn!("Can't convert data from {} to signed integer: {}", topic, e),
            };
        }
        constants::CONNECTED_CLIENTS_TOPIC_DEPRECATED => {
            match data.parse::<i64>() {
                Ok(v) => CONNECTED_CLIENTS.set(v),
                Err(e) => warn!("Can't convert data from {} to signed integer: {}", topic, e),
            };
        }
        constants::CONNECTIONS_1MIN_TOPIC => {
            match data.parse::<f64>() {
                Ok(v) => CONNECTIONS_1MIN.set(v),
                Err(e) => warn!(
                    "Can't convert data from {} to floating number: {}",
                    topic, e
                ),
            };
        }
        constants::CONNECTIONS_5MIN_TOPIC => {
            match data.parse::<f64>() {
                Ok(v) => CONNECTIONS_5MIN.set(v),
                Err(e) => warn!(
                    "Can't convert data from {} to floating number: {}",
                    topic, e
                ),
            };
        }
        constants::CONNECTIONS_15MIN_TOPIC => {
            match data.parse::<f64>() {
                Ok(v) => CONNECTIONS_15MIN.set(v),
                Err(e) => warn!(
                    "Can't convert data from {} to floating number: {}",
                    topic, e
                ),
            };
        }
        constants::CURRENT_HEAP_SIZE_TOPIC => {
            match data.parse::<i64>() {
                Ok(v) => CURRENT_HEAP_SIZE.set(v),
                Err(e) => warn!("Can't convert data from {} to signed integer: {}", topic, e),
            };
        }
        constants::DISCONNECTED_CLIENTS_TOPIC => {
            match data.parse::<u64>() {
                Ok(v) => {
                    let old_val = DISCONNECTED_CLIENTS.get();
                    if old_val > v {
                        DISCONNECTED_CLIENTS.reset();
                    } else {
                        DISCONNECTED_CLIENTS.inc_by(v - old_val);
                    }
                }
                Err(e) => warn!("Can't convert data from {} to signed integer: {}", topic, e),
            };
        }
        constants::DISCONNECTED_CLIENTS_TOPIC_DEPRECATED => {
            match data.parse::<u64>() {
                Ok(v) => {
                    let old_val = DISCONNECTED_CLIENTS.get();
                    if old_val > v {
                        DISCONNECTED_CLIENTS.reset();
                    } else {
                        DISCONNECTED_CLIENTS.inc_by(v - old_val);
                    }
                }
                Err(e) => warn!("Can't convert data from {} to signed integer: {}", topic, e),
            };
        }
        constants::EXPIRED_CLIENTS_TOPIC => {
            match data.parse::<i64>() {
                Ok(v) => EXPIRED_CLIENTS.set(v),
                Err(e) => warn!("Can't convert data from {} to signed integer: {}", topic, e),
            };
        }
        constants::MAXIMUM_CLIENTS_TOPIC => {
            match data.parse::<i64>() {
                Ok(v) => MAXIMUM_CLIENTS.set(v),
                Err(e) => warn!("Can't convert data from {} to signed integer: {}", topic, e),
            };
        }
        constants::MAXIMUM_HEAP_SIZE_TOPIC => {
            match data.parse::<i64>() {
                Ok(v) => MAXIMUM_HEAP_SIZE.set(v),
                Err(e) => warn!("Can't convert data from {} to signed integer: {}", topic, e),
            };
        }
        constants::MESSAGES_INFLIGHT_TOPIC => {
            match data.parse::<i64>() {
                Ok(v) => MESSAGES_INFLIGHT.set(v),
                Err(e) => warn!("Can't convert data from {} to signed integer: {}", topic, e),
            };
        }
        constants::MESSAGES_RECEIVE_1MIN_TOPIC => {
            match data.parse::<f64>() {
                Ok(v) => MESSAGES_RECEIVE_1MIN.set(v),
                Err(e) => warn!(
                    "Can't convert data from {} to floating number: {}",
                    topic, e
                ),
            };
        }
        constants::MESSAGES_RECEIVE_5MIN_TOPIC => {
            match data.parse::<f64>() {
                Ok(v) => MESSAGES_RECEIVE_5MIN.set(v),
                Err(e) => warn!(
                    "Can't convert data from {} to floating number: {}",
                    topic, e
                ),
            };
        }
        constants::MESSAGES_RECEIVE_15MIN_TOPIC => {
            match data.parse::<f64>() {
                Ok(v) => MESSAGES_RECEIVE_15MIN.set(v),
                Err(e) => warn!(
                    "Can't convert data from {} to floating number: {}",
                    topic, e
                ),
            };
        }
        constants::MESSAGES_RECEIVE_TOPIC => {
            match data.parse::<u64>() {
                Ok(v) => {
                    let old_val = MESSAGES_RECEIVE.get();
                    if old_val > v {
                        MESSAGES_RECEIVE.reset();
                    } else {
                        MESSAGES_RECEIVE.inc_by(v - old_val);
                    }
                }
                Err(e) => warn!("Can't convert data from {} to signed integer: {}", topic, e),
            };
        }
        constants::MESSAGES_RETAINED_TOPIC => {
            match data.parse::<u64>() {
                Ok(v) => {
                    let old_val = MESSAGES_RETAINED.get();
                    if old_val > v {
                        MESSAGES_RETAINED.reset();
                    } else {
                        MESSAGES_RETAINED.inc_by(v - old_val);
                    }
                }
                Err(e) => warn!("Can't convert data from {} to signed integer: {}", topic, e),
            };
        }
        constants::MESSAGES_TRANSMIT_1MIN_TOPIC => {
            match data.parse::<f64>() {
                Ok(v) => MESSAGES_TRANSMIT_1MIN.set(v),
                Err(e) => warn!(
                    "Can't convert data from {} to floating number: {}",
                    topic, e
                ),
            };
        }
        constants::MESSAGES_TRANSMIT_5MIN_TOPIC => {
            match data.parse::<f64>() {
                Ok(v) => MESSAGES_TRANSMIT_5MIN.set(v),
                Err(e) => warn!(
                    "Can't convert data from {} to floating number: {}",
                    topic, e
                ),
            };
        }
        constants::MESSAGES_TRANSMIT_15MIN_TOPIC => {
            match data.parse::<f64>() {
                Ok(v) => MESSAGES_TRANSMIT_15MIN.set(v),
                Err(e) => warn!(
                    "Can't convert data from {} to floating number: {}",
                    topic, e
                ),
            };
        }
        constants::MESSAGES_TRANSMIT_TOPIC => {
            match data.parse::<u64>() {
                Ok(v) => {
                    let old_val = MESSAGES_TRANSMIT.get();
                    if old_val > v {
                        MESSAGES_TRANSMIT.reset();
                    } else {
                        MESSAGES_TRANSMIT.inc_by(v - old_val);
                    }
                }
                Err(e) => warn!("Can't convert data from {} to signed integer: {}", topic, e),
            };
        }
        constants::PUBLISH_DROPPED_1MIN_TOPIC => {
            match data.parse::<f64>() {
                Ok(v) => PUBLISH_DROPPED_1MIN.set(v),
                Err(e) => warn!(
                    "Can't convert data from {} to floating number: {}",
                    topic, e
                ),
            };
        }
        constants::PUBLISH_DROPPED_5MIN_TOPIC => {
            match data.parse::<f64>() {
                Ok(v) => PUBLISH_DROPPED_5MIN.set(v),
                Err(e) => warn!(
                    "Can't convert data from {} to floating number: {}",
                    topic, e
                ),
            };
        }
        constants::PUBLISH_DROPPED_15MIN_TOPIC => {
            match data.parse::<f64>() {
                Ok(v) => PUBLISH_DROPPED_15MIN.set(v),
                Err(e) => warn!(
                    "Can't convert data from {} to floating number: {}",
                    topic, e
                ),
            };
        }
        constants::PUBLISH_DROPPED_TOPIC => {
            match data.parse::<u64>() {
                Ok(v) => {
                    let old_val = PUBLISH_DROPPED.get();
                    if old_val > v {
                        PUBLISH_DROPPED.reset();
                    } else {
                        PUBLISH_DROPPED.inc_by(v - old_val);
                    }
                }
                Err(e) => warn!("Can't convert data from {} to signed integer: {}", topic, e),
            };
        }
        constants::PUBLISH_RECEIVE_1MIN_TOPIC => {
            match data.parse::<f64>() {
                Ok(v) => PUBLISH_RECEIVE_1MIN.set(v),
                Err(e) => warn!(
                    "Can't convert data from {} to floating number: {}",
                    topic, e
                ),
            };
        }
        constants::PUBLISH_RECEIVE_5MIN_TOPIC => {
            match data.parse::<f64>() {
                Ok(v) => PUBLISH_RECEIVE_5MIN.set(v),
                Err(e) => warn!(
                    "Can't convert data from {} to floating number: {}",
                    topic, e
                ),
            };
        }
        constants::PUBLISH_RECEIVE_15MIN_TOPIC => {
            match data.parse::<f64>() {
                Ok(v) => PUBLISH_RECEIVE_15MIN.set(v),
                Err(e) => warn!(
                    "Can't convert data from {} to floating number: {}",
                    topic, e
                ),
            };
        }
        constants::PUBLISH_BYTES_RECEIVE_TOPIC => {
            match data.parse::<u64>() {
                Ok(v) => {
                    let old_val = PUBLISH_BYTES_RECEIVE.get();
                    if old_val > v {
                        PUBLISH_BYTES_RECEIVE.reset();
                    } else {
                        PUBLISH_BYTES_RECEIVE.inc_by(v - old_val);
                    }
                }
                Err(e) => warn!("Can't convert data from {} to signed integer: {}", topic, e),
            };
        }
        constants::PUBLISH_RECEIVE_TOPIC => {
            match data.parse::<u64>() {
                Ok(v) => {
                    let old_val = PUBLISH_RECEIVE.get();
                    if old_val > v {
                        PUBLISH_RECEIVE.reset();
                    } else {
                        PUBLISH_RECEIVE.inc_by(v - old_val);
                    }
                }
                Err(e) => warn!("Can't convert data from {} to signed integer: {}", topic, e),
            };
        }
        constants::PUBLISH_TRANSMIT_1MIN_TOPIC => {
            match data.parse::<f64>() {
                Ok(v) => PUBLISH_TRANSMIT_1MIN.set(v),
                Err(e) => warn!(
                    "Can't convert data from {} to floating number: {}",
                    topic, e
                ),
            };
        }
        constants::PUBLISH_TRANSMIT_5MIN_TOPIC => {
            match data.parse::<f64>() {
                Ok(v) => PUBLISH_TRANSMIT_5MIN.set(v),
                Err(e) => warn!(
                    "Can't convert data from {} to floating number: {}",
                    topic, e
                ),
            };
        }
        constants::PUBLISH_TRANSMIT_15MIN_TOPIC => {
            match data.parse::<f64>() {
                Ok(v) => PUBLISH_TRANSMIT_15MIN.set(v),
                Err(e) => warn!(
                    "Can't convert data from {} to floating number: {}",
                    topic, e
                ),
            };
        }
        constants::PUBLISH_BYTES_TRANSMIT_TOPIC => {
            match data.parse::<u64>() {
                Ok(v) => {
                    let old_val = PUBLISH_BYTES_TRANSMIT.get();
                    if old_val > v {
                        PUBLISH_BYTES_TRANSMIT.reset();
                    } else {
                        PUBLISH_BYTES_TRANSMIT.inc_by(v - old_val);
                    }
                }
                Err(e) => warn!("Can't convert data from {} to signed integer: {}", topic, e),
            };
        }
        constants::PUBLISH_TRANSMIT_TOPIC => {
            match data.parse::<u64>() {
                Ok(v) => {
                    let old_val = PUBLISH_TRANSMIT.get();
                    if old_val > v {
                        PUBLISH_TRANSMIT.reset();
                    } else {
                        PUBLISH_TRANSMIT.inc_by(v - old_val);
                    }
                }
                Err(e) => warn!("Can't convert data from {} to signed integer: {}", topic, e),
            };
        }
        constants::RECEIVE_BYTES_TOPIC => {
            match data.parse::<u64>() {
                Ok(v) => {
                    let old_val = RECEIVE_BYTES.get();
                    if old_val > v {
                        RECEIVE_BYTES.reset();
                    } else {
                        RECEIVE_BYTES.inc_by(v - old_val);
                    }
                }
                Err(e) => warn!("Can't convert data from {} to signed integer: {}", topic, e),
            };
        }
        constants::SOCKETS_1MIN_TOPIC => {
            match data.parse::<f64>() {
                Ok(v) => SOCKETS_1MIN.set(v),
                Err(e) => warn!(
                    "Can't convert data from {} to floating number: {}",
                    topic, e
                ),
            };
        }
        constants::SOCKETS_5MIN_TOPIC => {
            match data.parse::<f64>() {
                Ok(v) => SOCKETS_5MIN.set(v),
                Err(e) => warn!(
                    "Can't convert data from {} to floating number: {}",
                    topic, e
                ),
            };
        }
        constants::SOCKETS_15MIN_TOPIC => {
            match data.parse::<f64>() {
                Ok(v) => SOCKETS_15MIN.set(v),
                Err(e) => warn!(
                    "Can't convert data from {} to floating number: {}",
                    topic, e
                ),
            };
        }
        constants::STORE_MESSAGES_BYTES_TOPIC => {
            match data.parse::<i64>() {
                Ok(v) => STORE_MESSAGES_BYTES.set(v),
                Err(e) => warn!("Can't convert data from {} to signed integer: {}", topic, e),
            };
        }
        constants::STORE_MESSAGES_COUNT_TOPIC => {
            match data.parse::<i64>() {
                Ok(v) => STORE_MESSAGES_COUNT.set(v),
                Err(e) => warn!("Can't convert data from {} to signed integer: {}", topic, e),
            };
        }
        constants::STORE_MESSAGES_COUNT_TOPIC_DEPRECATED => {
            match data.parse::<i64>() {
                Ok(v) => STORE_MESSAGES_COUNT.set(v),
                Err(e) => warn!("Can't convert data from {} to signed integer: {}", topic, e),
            };
        }
        constants::SUBSCRIPTIONS_TOPIC => {
            match data.parse::<i64>() {
                Ok(v) => SUBSCRIPTIONS.set(v),
                Err(e) => warn!("Can't convert data from {} to signed integer: {}", topic, e),
            };
        }
        constants::TOTAL_CLIENTS_TOPIC => {
            match data.parse::<u64>() {
                Ok(v) => {
                    let old_val = TOTAL_CLIENTS.get();
                    if old_val > v {
                        TOTAL_CLIENTS.reset();
                    } else {
                        TOTAL_CLIENTS.inc_by(v - old_val);
                    }
                }
                Err(e) => warn!("Can't convert data from {} to signed integer: {}", topic, e),
            };
        }
        constants::TRANSMIT_BYTES_TOPIC => {
            match data.parse::<u64>() {
                Ok(v) => {
                    let old_val = TRANSMIT_BYTES.get();
                    if old_val > v {
                        TRANSMIT_BYTES.reset();
                    } else {
                        TRANSMIT_BYTES.inc_by(v - old_val);
                    }
                }
                Err(e) => warn!("Can't convert data from {} to signed integer: {}", topic, e),
            };
        }
        _ => {
            info!("Unhandled topic {} -> {}", topic, data);
        }
    };
}

fn register_metrics() {
    REGISTRY
        .register(Box::new(CONNECTED_CLIENTS.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(EXPIRED_CLIENTS.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(MAXIMUM_CLIENTS.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(CURRENT_HEAP_SIZE.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(MAXIMUM_HEAP_SIZE.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(MESSAGES_INFLIGHT.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(STORE_MESSAGES_COUNT.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(STORE_MESSAGES_BYTES.clone()))
        .unwrap();
    REGISTRY.register(Box::new(SUBSCRIPTIONS.clone())).unwrap();
    REGISTRY
        .register(Box::new(CONNECTIONS_1MIN.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(CONNECTIONS_5MIN.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(CONNECTIONS_15MIN.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(BYTES_RECEIVE_1MIN.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(BYTES_RECEIVE_5MIN.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(BYTES_RECEIVE_15MIN.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(BYTES_TRANSMIT_1MIN.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(BYTES_TRANSMIT_5MIN.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(BYTES_TRANSMIT_15MIN.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(MESSAGES_RECEIVE_1MIN.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(MESSAGES_RECEIVE_5MIN.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(MESSAGES_RECEIVE_15MIN.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(MESSAGES_TRANSMIT_1MIN.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(MESSAGES_TRANSMIT_5MIN.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(MESSAGES_TRANSMIT_15MIN.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(PUBLISH_DROPPED_1MIN.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(PUBLISH_DROPPED_5MIN.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(PUBLISH_DROPPED_15MIN.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(PUBLISH_RECEIVE_1MIN.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(PUBLISH_RECEIVE_5MIN.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(PUBLISH_RECEIVE_15MIN.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(PUBLISH_TRANSMIT_1MIN.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(PUBLISH_TRANSMIT_5MIN.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(PUBLISH_TRANSMIT_15MIN.clone()))
        .unwrap();
    REGISTRY.register(Box::new(SOCKETS_1MIN.clone())).unwrap();
    REGISTRY.register(Box::new(SOCKETS_5MIN.clone())).unwrap();
    REGISTRY.register(Box::new(SOCKETS_15MIN.clone())).unwrap();
    REGISTRY.register(Box::new(RECEIVE_BYTES.clone())).unwrap();
    REGISTRY.register(Box::new(TRANSMIT_BYTES.clone())).unwrap();
    REGISTRY
        .register(Box::new(DISCONNECTED_CLIENTS.clone()))
        .unwrap();
    REGISTRY.register(Box::new(TOTAL_CLIENTS.clone())).unwrap();
    REGISTRY
        .register(Box::new(MESSAGES_RECEIVE.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(MESSAGES_TRANSMIT.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(PUBLISH_DROPPED.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(PUBLISH_RECEIVE.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(PUBLISH_TRANSMIT.clone()))
        .unwrap();
    REGISTRY
        .register(Box::new(MESSAGES_RETAINED.clone()))
        .unwrap();
}

pub fn serve_metrics() -> String {
    let encoder = prometheus::TextEncoder::new();
    let mut buffer = String::new();

    if let Err(e) = encoder.encode_utf8(&REGISTRY.gather(), &mut buffer) {
        error!("Can't encode metrics as UTF8 string: {}", e);
    }

    buffer
}
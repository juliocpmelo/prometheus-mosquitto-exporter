use crate::constants;

use log::{error, info, warn};
use prometheus::{Counter, Gauge, IntGauge, Registry};
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
    pub static ref RECEIVE_BYTES: Counter = Counter::new(constants::RECEIVE_BYTES_NAME, constants::RECEIVE_BYTES_DESC).unwrap();
    pub static ref TRANSMIT_BYTES: Counter = Counter::new(constants::TRANSMIT_BYTES_NAME, constants::TRANSMIT_BYTES_DESC).unwrap();
    pub static ref DISCONNECTED_CLIENTS: Counter = Counter::new(constants::DISCONNECTED_CLIENTS_NAME, constants::DISCONNECTED_CLIENTS_DESC).unwrap();
    pub static ref TOTAL_CLIENTS: Counter = Counter::new(constants::TOTAL_CLIENTS_NAME, constants::TOTAL_CLIENTS_DESC).unwrap();
    pub static ref MESSAGES_RECEIVE: Counter = Counter::new(constants::MESSAGES_RECEIVE_NAME, constants::MESSAGES_RECEIVE_DESC).unwrap();
    pub static ref MESSAGES_TRANSMIT: Counter = Counter::new(constants::MESSAGES_TRANSMIT_NAME, constants::MESSAGES_TRANSMIT_DESC).unwrap();
    pub static ref PUBLISH_DROPPED: Counter = Counter::new(constants::PUBLISH_DROPPED_NAME, constants::PUBLISH_DROPPED_DESC).unwrap();
    pub static ref PUBLISH_RECEIVE: Counter = Counter::new(constants::PUBLISH_RECEIVE_NAME, constants::PUBLISH_RECEIVE_DESC).unwrap();
    pub static ref PUBLISH_TRANSMIT: Counter = Counter::new(constants::PUBLISH_TRANSMIT_NAME, constants::PUBLISH_TRANSMIT_DESC).unwrap();
    pub static ref MESSAGES_RETAINED: Counter = Counter::new(constants::MESSAGES_RETAINED_NAME, constants::MESSAGES_RETAINED_DESC).unwrap();
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

        // $SYS/broker/version and $SYS/broker/uptime are strings and can't be converted, so don't bother ...
        if topic == constants::MOSQUITTO_UPTIME_TOPIC || topic == constants::MOSQUITTO_VERSION_TOPIC
        {
            continue;
        }

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
    // TODO: Define topic in constants and use match statement

    // Integer gauges
    if topic == "$SYS/broker/clients/connected" || topic == "$SYS/broker/clients/active" {
        let int_value = match data.parse::<i64>() {
            Ok(v) => v,
            Err(e) => {
                warn!("Can't convert data from {} to signed integer: {}", topic, e);
                return;
            }
        };
        CONNECTED_CLIENTS.set(int_value);
    };

    // Float gauges
    // Counters
}

fn register_metrics() {
    REGISTRY
        .register(Box::new(CONNECTED_CLIENTS.clone()))
        .unwrap();
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

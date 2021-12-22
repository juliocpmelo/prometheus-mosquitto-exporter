pub const DEFAULT_LISTEN: &str = "localhost:9883";
pub const DEFAULT_METRICS_PATH: &str = "/metrics";
pub const DEFAULT_MQTT_CA_FILE: &str = "/etc/ssl/certs/ca-certificates.crt";
pub const DEFAULT_MQTT_CLIENT_ID: &str = "mosquitto-exporter";
pub const DEFAULT_MQTT_QOS: i32 = 0;
pub const DEFAULT_MQTT_RETRY_INTERVAL: u64 = 300;
pub const DEFAULT_MQTT_TIMEOUT: u64 = 15;

pub const NAME: &str = "prometheus-mosquitto-exporter";
pub const VERSION: &str = "0.1.0-20211222";

pub const MOSQUITTO_STATISTICS_TOPIC: &str = "$SYS/#";
pub const MOSQUITTO_UPTIME_TOPIC: &str = "$SYS/broker/uptime";
pub const MOSQUITTO_VERSION_TOPIC: &str = "$SYS/broker/version";

pub const CONNECTED_CLIENTS_NAME: &str = "mosquitto_clients_connected";
pub const CONNECTED_CLIENTS_DESC: &str = "The number of currently connected clients";

pub const RECEIVE_BYTES_NAME: &str = "mosquitto_receive_bytes_total";
pub const RECEIVE_BYTES_DESC: &str = "The total number of bytes received since the broker started";

pub const TRANSMIT_BYTES_NAME: &str = "mosquitto_transmit_bytes_total";
pub const TRANSMIT_BYTES_DESC: &str =
    "The total number of bytes transmitted since the broker started";

pub const EXPIRED_CLIENTS_NAME: &str = "mosquitto_clients_expired";
pub const EXPIRED_CLIENTS_DESC: &str = "The number of disconnected persistent clients that have been expired and removed through the persistent_client_expiration option";

pub const DISCONNECTED_CLIENTS_NAME: &str = "mosquitto_clients_disconnected_total";
pub const DISCONNECTED_CLIENTS_DESC: &str = "The total number of persistent clients (with clean session disabled) that are registered at the broker but are currently disconnected";

pub const MAXIMUM_CLIENTS_NAME: &str = "mosquitto_clients_maximum";
pub const MAXIMUM_CLIENTS_DESC: &str =
    "The maximum number of clients that have been connected to the broker at the same time";

pub const TOTAL_CLIENTS_NAME: &str = "mosquitto_clients_total";
pub const TOTAL_CLIENTS_DESC: &str = "The total number of active and inactive clients currently connected and registered on the broker";

pub const CURRENT_HEAP_SIZE_NAME: &str = "mosquitto_current_heap_size_bytes";
pub const CURRENT_HEAP_SIZE_DESC: &str = "The current size of the heap memory in use by mosquitto";

pub const MAXIMUM_HEAP_SIZE_NAME: &str = "mosquitto_maximum_heap_size_bytes";
pub const MAXIMUM_HEAP_SIZE_DESC: &str = "The maximum size of the heap memory in use by mosquitto";

pub const CONNECTIONS_1MIN_NAME: &str = "mosquitto_1min_avg_load_connections";
pub const CONNECTIONS_1MIN_DESC: &str = "The moving average of the number of CONNECT packets received by the broker over 1 minute time interval";

pub const CONNECTIONS_5MIN_NAME: &str = "mosquitto_5min_avg_load_connections";
pub const CONNECTIONS_5MIN_DESC: &str = "The moving average of the number of CONNECT packets received by the broker over 5 minutes time interval";

pub const CONNECTIONS_15MIN_NAME: &str = "mosquitto_15min_avg_load_connections";
pub const CONNECTIONS_15MIN_DESC: &str = "The moving average of the number of CONNECT packets received by the broker over 15 minutes time interval";

pub const BYTES_RECEIVE_1MIN_NAME: &str = "mosquitto_1min_avg_receive_bytes";
pub const BYTES_RECEIVE_1MIN_DESC: &str =
    "The moving average of the number of bytes received by the broker over 1 minute time interval";

pub const BYTES_RECEIVE_5MIN_NAME: &str = "mosquitto_5min_avg_receive_bytes";
pub const BYTES_RECEIVE_5MIN_DESC: &str =
    "The moving average of the number of bytes received by the broker over 5 minutes time interval";

pub const BYTES_RECEIVE_15MIN_NAME: &str = "mosquitto_15min_avg_receive_bytes";
pub const BYTES_RECEIVE_15MIN_DESC: &str = "The moving average of the number of bytes received by the broker over 15 minutes time interval";

pub const BYTES_TRANSMIT_1MIN_NAME: &str = "mosquitto_1min_avg_transmit_bytes";
pub const BYTES_TRANSMIT_1MIN_DESC: &str = "The moving average of the number of bytes transmitted by the broker over 1 minute time interval";

pub const BYTES_TRANSMIT_5MIN_NAME: &str = "mosquitto_5min_avg_transmit_bytes";
pub const BYTES_TRANSMIT_5MIN_DESC: &str = "The moving average of the number of bytes transmitted by the broker over 5 minutes time interval";

pub const BYTES_TRANSMIT_15MIN_NAME: &str = "mosquitto_15min_avg_transmit_bytes";
pub const BYTES_TRANSMIT_15MIN_DESC: &str = "The moving average of the number of bytes transmitted by the broker over 15 minutes time interval";

pub const MESSAGES_RECEIVE_1MIN_NAME: &str = "mosquitto_1min_avg_receive_messages";
pub const MESSAGES_RECEIVE_1MIN_DESC: &str = "The moving average of the number of all types of MQTT messages received by the broker over 1 minute time interval";

pub const MESSAGES_RECEIVE_5MIN_NAME: &str = "mosquitto_5min_avg_receive_messages";
pub const MESSAGES_RECEIVE_5MIN_DESC: &str = "The moving average of the number of all types of MQTT messages received by the broker over 5 minutes time interval";

pub const MESSAGES_RECEIVE_15MIN_NAME: &str = "mosquitto_15min_avg_receive_messages";
pub const MESSAGES_RECEIVE_15MIN_DESC: &str = "The moving average of the number of all types of MQTT messages received by the broker over 15 minutes time interval";

pub const MESSAGES_TRANSMIT_1MIN_NAME: &str = "mosquitto_1min_avg_transmit_messages";
pub const MESSAGES_TRANSMIT_1MIN_DESC: &str = "The moving average of the number of all types of MQTT messages transmitted by the broker over 1 minute time interval";

pub const MESSAGES_TRANSMIT_5MIN_NAME: &str = "mosquitto_5min_avg_transmit_messages";
pub const MESSAGES_TRANSMIT_5MIN_DESC: &str = "The moving average of the number of all types of MQTT messages transmitted by the broker over 5 minutes time interval";

pub const MESSAGES_TRANSMIT_15MIN_NAME: &str = "mosquitto_15min_avg_transmit_messages";
pub const MESSAGES_TRANSMIT_15MIN_DESC: &str = "The moving average of the number of all types of MQTT messages transmitted by the broker over 15 minutes time interval";

pub const PUBLISH_DROPPED_1MIN_NAME: &str = "mosquitto_1min_avg_publish_drop_messages";
pub const PUBLISH_DROPPED_1MIN_DESC: &str = "The moving average of the number of publish messages dropped by the broker over 1 minute time interval";

pub const PUBLISH_DROPPED_5MIN_NAME: &str = "mosquitto_5min_avg_publish_drop_messages";
pub const PUBLISH_DROPPED_5MIN_DESC: &str = "The moving average of the number of publish messages dropped by the broker over 5 minutes time interval";

pub const PUBLISH_DROPPED_15MIN_NAME: &str = "mosquitto_15min_avg_publish_drop_messages";
pub const PUBLISH_DROPPED_15MIN_DESC: &str = "The moving average of the number of publish messages dropped by the broker over 15 minutes time interval";

pub const PUBLISH_RECEIVE_1MIN_NAME: &str = "mosquitto_1min_avg_publish_receive_messages";
pub const PUBLISH_RECEIVE_1MIN_DESC: &str = "The moving average of the number of publish messages received by the broker over 1 minute time interval";

pub const PUBLISH_RECEIVE_5MIN_NAME: &str = "mosquitto_5min_avg_publish_receive_messages";
pub const PUBLISH_RECEIVE_5MIN_DESC: &str = "The moving average of the number of publish messages received by the broker over 5 minutes time interval";

pub const PUBLISH_RECEIVE_15MIN_NAME: &str = "mosquitto_15min_avg_publish_receive_messages";
pub const PUBLISH_RECEIVE_15MIN_DESC: &str = "The moving average of the number of publish messages received by the broker over 15 minutes time interval";

pub const PUBLISH_TRANSMIT_1MIN_NAME: &str = "mosquitto_1min_avg_publish_transmit_messages";
pub const PUBLISH_TRANSMIT_1MIN_DESC: &str = "The moving average of the number of publish messages transmitted by the broker over 1 minute time interval";

pub const PUBLISH_TRANSMIT_5MIN_NAME: &str = "mosquitto_5min_avg_publish_transmit_messages";
pub const PUBLISH_TRANSMIT_5MIN_DESC: &str = "The moving average of the number of publish messages transmitted by the broker over 5 minutes time interval";

pub const PUBLISH_TRANSMIT_15MIN_NAME: &str = "mosquitto_15min_avg_publish_transmit_messages";
pub const PUBLISH_TRANSMIT_15MIN_DESC: &str = "The moving average of the number of publish messages transmitted by the broker over 15 minutes time interval";

pub const SOCKETS_1MIN_NAME: &str = "mosquitto_1min_avg_sockets";
pub const SOCKETS_1MIN_DESC: &str = "The moving average of the number of socket connections opened to the broker over 1 minute time interval";

pub const SOCKETS_5MIN_NAME: &str = "mosquitto_5min_avg_sockets";
pub const SOCKETS_5MIN_DESC: &str = "The moving average of the number of socket connections opened to the broker over 5 minutes time interval";

pub const SOCKETS_15MIN_NAME: &str = "mosquitto_15min_avg_sockets";
pub const SOCKETS_15MIN_DESC: &str = "The moving average of the number of socket connections opened to the broker over 15 minutes time interval";

pub const MESSAGES_INFLIGHT_NAME: &str = "mosquitto_inflight_messages";
pub const MESSAGES_INFLIGHT_DESC: &str =
    "The number of messages with QoS>0 that are awaiting acknowledgments";

pub const MESSAGES_RECEIVE_NAME: &str = "mosquitto_receive_messages_total";
pub const MESSAGES_RECEIVE_DESC: &str =
    "The total number of messages of any type received since the broker started";

pub const MESSAGES_TRANSMIT_NAME: &str = "mosquitto_transmit_messages_total";
pub const MESSAGES_TRANSMIT_DESC: &str =
    "The total number of messages of any type transmitted since the broker started";

pub const PUBLISH_DROPPED_NAME: &str = "mosquitto_drop_messages_total";
pub const PUBLISH_DROPPED_DESC: &str =
    "The total number of publish messages that have been dropped due to inflight/queuing limits";

pub const PUBLISH_RECEIVE_NAME: &str = "mosquitto_receive_messages_total";
pub const PUBLISH_RECEIVE_DESC: &str =
    "The total number of PUBLISH messages received since the broker started";

pub const PUBLISH_TRANSMIT_NAME: &str = "mosquitto_transmit_messages_total";
pub const PUBLISH_TRANSMIT_DESC: &str =
    "The total number of PUBLISH messages transmitted since the broker started";

pub const MESSAGES_RETAINED_NAME: &str = "mosquitto_retain_messages_total";
pub const MESSAGES_RETAINED_DESC: &str =
    "The total number of retained messages active on the broker";

pub const STORE_MESSAGES_COUNT_NAME: &str = "mosquitto_store_messages";
pub const STORE_MESSAGES_COUNT_DESC: &str = "The number of messages currently held in the message store including retained messages and messages queued for durable clients";

pub const STORE_MESSAGES_BYTES_NAME: &str = "mosquitto_store_bytes";
pub const STORE_MESSAGES_BYTES_DESC: &str = "The number of bytes currently held by message payloads in the message store inclusing retained messages and messages queued for durable clients";

pub const SUBSCRIPTIONS_NAME: &str = "mosquitto_active_subscriptions";
pub const SUBSCRIPTIONS_DESC: &str = "The total number of subscriptions active on the broker";

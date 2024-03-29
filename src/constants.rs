pub const DEFAULT_LISTEN: &str = "localhost:9883";
pub const DEFAULT_METRICS_PATH: &str = "/metrics";
pub const DEFAULT_MQTT_CA_FILE: &str = "/etc/ssl/certs/ca-certificates.crt";
pub const DEFAULT_MQTT_CLIENT_ID: &str = "mosquitto-exporter";
pub const DEFAULT_MQTT_QOS: i32 = 0;
pub const DEFAULT_MQTT_RETRY_INTERVAL: u64 = 300;
pub const DEFAULT_MQTT_TIMEOUT: u64 = 15;

pub const DEFAULT_TOPIC_LISTENER: &str = "prometheus-mqtt-exporter/listen";

pub const NAME: &str = "prometheus-mosquitto-exporter";
pub const VERSION: &str = "1.1.0";

pub const BYTES_RECEIVE_15MIN_DESC: &str = "The moving average of the number of bytes received by the broker over 15 minutes time interval";
pub const BYTES_RECEIVE_15MIN_NAME: &str = "mosquitto_15min_avg_receive_bytes";
pub const BYTES_RECEIVE_15MIN_TOPIC: &str = "$SYS/broker/load/bytes/received/15min";
pub const BYTES_RECEIVE_1MIN_DESC: &str =
    "The moving average of the number of bytes received by the broker over 1 minute time interval";
pub const BYTES_RECEIVE_1MIN_NAME: &str = "mosquitto_1min_avg_receive_bytes";
pub const BYTES_RECEIVE_1MIN_TOPIC: &str = "$SYS/broker/load/bytes/received/1min";
pub const BYTES_RECEIVE_5MIN_DESC: &str =
    "The moving average of the number of bytes received by the broker over 5 minutes time interval";
pub const BYTES_RECEIVE_5MIN_NAME: &str = "mosquitto_5min_avg_receive_bytes";
pub const BYTES_RECEIVE_5MIN_TOPIC: &str = "$SYS/broker/load/bytes/received/5min";
pub const BYTES_TRANSMIT_15MIN_DESC: &str = "The moving average of the number of bytes transmitted by the broker over 15 minutes time interval";
pub const BYTES_TRANSMIT_15MIN_NAME: &str = "mosquitto_15min_avg_transmit_bytes";
pub const BYTES_TRANSMIT_15MIN_TOPIC: &str = "$SYS/broker/load/bytes/sent/15min";
pub const BYTES_TRANSMIT_1MIN_DESC: &str = "The moving average of the number of bytes transmitted by the broker over 1 minute time interval";
pub const BYTES_TRANSMIT_1MIN_NAME: &str = "mosquitto_1min_avg_transmit_bytes";
pub const BYTES_TRANSMIT_1MIN_TOPIC: &str = "$SYS/broker/load/bytes/sent/1min";
pub const BYTES_TRANSMIT_5MIN_DESC: &str = "The moving average of the number of bytes transmitted by the broker over 5 minutes time interval";
pub const BYTES_TRANSMIT_5MIN_NAME: &str = "mosquitto_5min_avg_transmit_bytes";
pub const BYTES_TRANSMIT_5MIN_TOPIC: &str = "$SYS/broker/load/bytes/sent/5min";
pub const CONNECTED_CLIENTS_DESC: &str = "The number of currently connected clients";
pub const CONNECTED_CLIENTS_NAME: &str = "mosquitto_clients_connected";
pub const CONNECTED_CLIENTS_TOPIC: &str = "$SYS/broker/clients/connected";
pub const CONNECTED_CLIENTS_TOPIC_DEPRECATED: &str = "$SYS/broker/clients/active";
pub const CONNECTIONS_15MIN_DESC: &str = "The moving average of the number of CONNECT packets received by the broker over 15 minutes time interval";
pub const CONNECTIONS_15MIN_NAME: &str = "mosquitto_15min_avg_load_connections";
pub const CONNECTIONS_15MIN_TOPIC: &str = "$SYS/broker/load/connections/15min";
pub const CONNECTIONS_1MIN_DESC: &str = "The moving average of the number of CONNECT packets received by the broker over 1 minute time interval";
pub const CONNECTIONS_1MIN_NAME: &str = "mosquitto_1min_avg_load_connections";
pub const CONNECTIONS_1MIN_TOPIC: &str = "$SYS/broker/load/connections/1min";
pub const CONNECTIONS_5MIN_DESC: &str = "The moving average of the number of CONNECT packets received by the broker over 5 minutes time interval";
pub const CONNECTIONS_5MIN_NAME: &str = "mosquitto_5min_avg_load_connections";
pub const CONNECTIONS_5MIN_TOPIC: &str = "$SYS/broker/load/connections/5min";
pub const CURRENT_HEAP_SIZE_DESC: &str = "The current size of the heap memory in use by mosquitto";
pub const CURRENT_HEAP_SIZE_NAME: &str = "mosquitto_current_heap_size_bytes";
pub const CURRENT_HEAP_SIZE_TOPIC: &str = "$SYS/broker/heap/current";
pub const DISCONNECTED_CLIENTS_DESC: &str = "The total number of persistent clients (with clean session disabled) that are registered at the broker but are currently disconnected";
pub const DISCONNECTED_CLIENTS_NAME: &str = "mosquitto_clients_disconnected_total";
pub const DISCONNECTED_CLIENTS_TOPIC: &str = "$SYS/broker/clients/disconnected";
pub const DISCONNECTED_CLIENTS_TOPIC_DEPRECATED: &str = "$SYS/broker/clients/inactive";
pub const EXPIRED_CLIENTS_DESC: &str = "The number of disconnected persistent clients that have been expired and removed through the persistent_client_expiration option";
pub const EXPIRED_CLIENTS_NAME: &str = "mosquitto_clients_expired";
pub const EXPIRED_CLIENTS_TOPIC: &str = "$SYS/broker/clients/expired";
pub const MAXIMUM_CLIENTS_DESC: &str =
    "The maximum number of clients that have been connected to the broker at the same time";
pub const MAXIMUM_CLIENTS_NAME: &str = "mosquitto_clients_maximum";
pub const MAXIMUM_CLIENTS_TOPIC: &str = "$SYS/broker/clients/maximum";
pub const MAXIMUM_HEAP_SIZE_DESC: &str = "The maximum size of the heap memory in use by mosquitto";
pub const MAXIMUM_HEAP_SIZE_NAME: &str = "mosquitto_maximum_heap_size_bytes";
pub const MAXIMUM_HEAP_SIZE_TOPIC: &str = "$SYS/broker/heap/maximum";
pub const MESSAGES_INFLIGHT_DESC: &str =
    "The number of messages with QoS>0 that are awaiting acknowledgments";
pub const MESSAGES_INFLIGHT_NAME: &str = "mosquitto_inflight_messages";
pub const MESSAGES_INFLIGHT_TOPIC: &str = "$SYS/broker/messages/inflight";
pub const MESSAGES_RECEIVE_15MIN_DESC: &str = "The moving average of the number of all types of MQTT messages received by the broker over 15 minutes time interval";
pub const MESSAGES_RECEIVE_15MIN_NAME: &str = "mosquitto_15min_avg_receive_messages";
pub const MESSAGES_RECEIVE_15MIN_TOPIC: &str = "$SYS/broker/load/messages/received/15min";
pub const MESSAGES_RECEIVE_1MIN_DESC: &str = "The moving average of the number of all types of MQTT messages received by the broker over 1 minute time interval";
pub const MESSAGES_RECEIVE_1MIN_NAME: &str = "mosquitto_1min_avg_receive_messages";
pub const MESSAGES_RECEIVE_1MIN_TOPIC: &str = "$SYS/broker/load/messages/received/1min";
pub const MESSAGES_RECEIVE_5MIN_DESC: &str = "The moving average of the number of all types of MQTT messages received by the broker over 5 minutes time interval";
pub const MESSAGES_RECEIVE_5MIN_NAME: &str = "mosquitto_5min_avg_receive_messages";
pub const MESSAGES_RECEIVE_5MIN_TOPIC: &str = "$SYS/broker/load/messages/received/5min";
pub const MESSAGES_RECEIVE_DESC: &str =
    "The total number of messages of any type received since the broker started";
pub const MESSAGES_RECEIVE_NAME: &str = "mosquitto_receive_messages_total";
pub const MESSAGES_RECEIVE_TOPIC: &str = "$SYS/broker/messages/received";
pub const MESSAGES_RETAINED_DESC: &str =
    "The total number of retained messages active on the broker";
pub const MESSAGES_RETAINED_NAME: &str = "mosquitto_retain_messages_total";
pub const MESSAGES_RETAINED_TOPIC: &str = "$SYS/broker/retained messages/count";
pub const MESSAGES_TRANSMIT_15MIN_DESC: &str = "The moving average of the number of all types of MQTT messages transmitted by the broker over 15 minutes time interval";
pub const MESSAGES_TRANSMIT_15MIN_NAME: &str = "mosquitto_15min_avg_transmit_messages";
pub const MESSAGES_TRANSMIT_15MIN_TOPIC: &str = "$SYS/broker/load/messages/sent/15min";
pub const MESSAGES_TRANSMIT_1MIN_DESC: &str = "The moving average of the number of all types of MQTT messages transmitted by the broker over 1 minute time interval";
pub const MESSAGES_TRANSMIT_1MIN_NAME: &str = "mosquitto_1min_avg_transmit_messages";
pub const MESSAGES_TRANSMIT_1MIN_TOPIC: &str = "$SYS/broker/load/messages/sent/1min";
pub const MESSAGES_TRANSMIT_5MIN_DESC: &str = "The moving average of the number of all types of MQTT messages transmitted by the broker over 5 minutes time interval";
pub const MESSAGES_TRANSMIT_5MIN_NAME: &str = "mosquitto_5min_avg_transmit_messages";
pub const MESSAGES_TRANSMIT_5MIN_TOPIC: &str = "$SYS/broker/load/messages/sent/5min";
pub const MESSAGES_TRANSMIT_DESC: &str =
    "The total number of messages of any type transmitted since the broker started";
pub const MESSAGES_TRANSMIT_NAME: &str = "mosquitto_transmit_messages_total";
pub const MESSAGES_TRANSMIT_TOPIC: &str = "$SYS/broker/messages/sent";
pub const MOSQUITTO_STATISTICS_TOPIC: &str = "$SYS/#";
pub const MOSQUITTO_UPTIME_TOPIC: &str = "$SYS/broker/uptime";
pub const MOSQUITTO_VERSION_TOPIC: &str = "$SYS/broker/version";
pub const PUBLISH_BYTES_RECEIVE_DESC: &str =
    "The total number of PUBLISH bytes received since the broker started";
pub const PUBLISH_BYTES_RECEIVE_NAME: &str = "mosquitto_publish_receive_bytes_total";
pub const PUBLISH_BYTES_RECEIVE_TOPIC: &str = "$SYS/broker/publish/bytes/received";
pub const PUBLISH_BYTES_TRANSMIT_DESC: &str =
    "The total number of PUBLISH bytes transmitted since the broker started";
pub const PUBLISH_BYTES_TRANSMIT_NAME: &str = "mosquitto_publish_transmit_bytes_total";
pub const PUBLISH_BYTES_TRANSMIT_TOPIC: &str = "$SYS/broker/publish/bytes/sent";
pub const PUBLISH_DROPPED_15MIN_DESC: &str = "The moving average of the number of publish messages dropped by the broker over 15 minutes time interval";
pub const PUBLISH_DROPPED_15MIN_NAME: &str = "mosquitto_15min_avg_publish_drop_messages";
pub const PUBLISH_DROPPED_15MIN_TOPIC: &str = "$SYS/broker/load/publish/dropped/15min";
pub const PUBLISH_DROPPED_1MIN_DESC: &str = "The moving average of the number of publish messages dropped by the broker over 1 minute time interval";
pub const PUBLISH_DROPPED_1MIN_NAME: &str = "mosquitto_1min_avg_publish_drop_messages";
pub const PUBLISH_DROPPED_1MIN_TOPIC: &str = "$SYS/broker/load/publish/dropped/1min";
pub const PUBLISH_DROPPED_5MIN_DESC: &str = "The moving average of the number of publish messages dropped by the broker over 5 minutes time interval";
pub const PUBLISH_DROPPED_5MIN_NAME: &str = "mosquitto_5min_avg_publish_drop_messages";
pub const PUBLISH_DROPPED_5MIN_TOPIC: &str = "$SYS/broker/load/publish/dropped/5min";
pub const PUBLISH_DROPPED_DESC: &str =
    "The total number of publish messages that have been dropped due to inflight/queuing limits";
pub const PUBLISH_DROPPED_NAME: &str = "mosquitto_publish_drop_messages_total";
pub const PUBLISH_DROPPED_TOPIC: &str = "$SYS/broker/publish/messages/dropped";
pub const PUBLISH_RECEIVE_15MIN_DESC: &str = "The moving average of the number of publish messages received by the broker over 15 minutes time interval";
pub const PUBLISH_RECEIVE_15MIN_NAME: &str = "mosquitto_15min_avg_publish_receive_messages";
pub const PUBLISH_RECEIVE_15MIN_TOPIC: &str = "$SYS/broker/load/publish/received/15min";
pub const PUBLISH_RECEIVE_1MIN_DESC: &str = "The moving average of the number of publish messages received by the broker over 1 minute time interval";
pub const PUBLISH_RECEIVE_1MIN_NAME: &str = "mosquitto_1min_avg_publish_receive_messages";
pub const PUBLISH_RECEIVE_1MIN_TOPIC: &str = "$SYS/broker/load/publish/received/1min";
pub const PUBLISH_RECEIVE_5MIN_DESC: &str = "The moving average of the number of publish messages received by the broker over 5 minutes time interval";
pub const PUBLISH_RECEIVE_5MIN_NAME: &str = "mosquitto_5min_avg_publish_receive_messages";
pub const PUBLISH_RECEIVE_5MIN_TOPIC: &str = "$SYS/broker/load/publish/received/5min";
pub const PUBLISH_RECEIVE_DESC: &str =
    "The total number of PUBLISH messages received since the broker started";
pub const PUBLISH_RECEIVE_NAME: &str = "mosquitto_publish_receive_messages_total";
pub const PUBLISH_RECEIVE_TOPIC: &str = "$SYS/broker/publish/messages/received";
pub const PUBLISH_TRANSMIT_15MIN_DESC: &str = "The moving average of the number of publish messages transmitted by the broker over 15 minutes time interval";
pub const PUBLISH_TRANSMIT_15MIN_NAME: &str = "mosquitto_15min_avg_publish_transmit_messages";
pub const PUBLISH_TRANSMIT_15MIN_TOPIC: &str = "$SYS/broker/load/publish/sent/15min";
pub const PUBLISH_TRANSMIT_1MIN_DESC: &str = "The moving average of the number of publish messages transmitted by the broker over 1 minute time interval";
pub const PUBLISH_TRANSMIT_1MIN_NAME: &str = "mosquitto_1min_avg_publish_transmit_messages";
pub const PUBLISH_TRANSMIT_1MIN_TOPIC: &str = "$SYS/broker/load/publish/sent/1min";
pub const PUBLISH_TRANSMIT_5MIN_DESC: &str = "The moving average of the number of publish messages transmitted by the broker over 5 minutes time interval";
pub const PUBLISH_TRANSMIT_5MIN_NAME: &str = "mosquitto_5min_avg_publish_transmit_messages";
pub const PUBLISH_TRANSMIT_5MIN_TOPIC: &str = "$SYS/broker/load/publish/sent/5min";
pub const PUBLISH_TRANSMIT_DESC: &str =
    "The total number of PUBLISH messages transmitted since the broker started";
pub const PUBLISH_TRANSMIT_NAME: &str = "mosquitto_publish_transmit_messages_total";
pub const PUBLISH_TRANSMIT_TOPIC: &str = "$SYS/broker/publish/messages/sent";
pub const RECEIVE_BYTES_DESC: &str = "The total number of bytes received since the broker started";
pub const RECEIVE_BYTES_NAME: &str = "mosquitto_receive_bytes_total";
pub const RECEIVE_BYTES_TOPIC: &str = "$SYS/broker/bytes/received";
pub const SOCKETS_15MIN_DESC: &str = "The moving average of the number of socket connections opened to the broker over 15 minutes time interval";
pub const SOCKETS_15MIN_NAME: &str = "mosquitto_15min_avg_sockets";
pub const SOCKETS_15MIN_TOPIC: &str = "$SYS/broker/load/sockets/15min";
pub const SOCKETS_1MIN_DESC: &str = "The moving average of the number of socket connections opened to the broker over 1 minute time interval";
pub const SOCKETS_1MIN_NAME: &str = "mosquitto_1min_avg_sockets";
pub const SOCKETS_1MIN_TOPIC: &str = "$SYS/broker/load/sockets/1min";
pub const SOCKETS_5MIN_DESC: &str = "The moving average of the number of socket connections opened to the broker over 5 minutes time interval";
pub const SOCKETS_5MIN_NAME: &str = "mosquitto_5min_avg_sockets";
pub const SOCKETS_5MIN_TOPIC: &str = "$SYS/broker/load/sockets/5min";
pub const STORE_MESSAGES_BYTES_DESC: &str = "The number of bytes currently held by message payloads in the message store inclusing retained messages and messages queued for durable clients";
pub const STORE_MESSAGES_BYTES_NAME: &str = "mosquitto_store_bytes";
pub const STORE_MESSAGES_BYTES_TOPIC: &str = "$SYS/broker/store/messages/bytes";
pub const STORE_MESSAGES_COUNT_DESC: &str = "The number of messages currently held in the message store including retained messages and messages queued for durable clients";
pub const STORE_MESSAGES_COUNT_NAME: &str = "mosquitto_store_messages";
pub const STORE_MESSAGES_COUNT_TOPIC: &str = "$SYS/broker/store/messages/count";
pub const STORE_MESSAGES_COUNT_TOPIC_DEPRECATED: &str = "$SYS/broker/messages/stored";
pub const SUBSCRIPTIONS_DESC: &str = "The total number of subscriptions active on the broker";
pub const SUBSCRIPTIONS_NAME: &str = "mosquitto_active_subscriptions";
pub const SUBSCRIPTIONS_TOPIC: &str = "$SYS/broker/subscriptions/count";
pub const TOTAL_CLIENTS_DESC: &str = "The total number of active and inactive clients currently connected and registered on the broker";
pub const TOTAL_CLIENTS_NAME: &str = "mosquitto_clients_total";
pub const TOTAL_CLIENTS_TOPIC: &str = "$SYS/broker/clients/total";
pub const TRANSMIT_BYTES_DESC: &str =
    "The total number of bytes transmitted since the broker started";
pub const TRANSMIT_BYTES_NAME: &str = "mosquitto_transmit_bytes_total";
pub const TRANSMIT_BYTES_TOPIC: &str = "$SYS/broker/bytes/sent";

pub const MONITORED_TOPICS_RECEIVED_PUBLISHES_NAME: &str = "monitoredtopics_received_publishes_total";
pub const MONITORED_TOPICS_RECEIVED_PUBLISHES_DESC: &str =  "The counter of the number of publishes received on specific topics configured on {topic_listener}";

#[allow(non_snake_case)] //kept here just to follow the coding style
pub fn MONITORED_TOPIC_RECEIVED_PUBLISHES_NAME(topic: &String) -> String {
    let base_name : String = String::from("monitoredtopics_{topic}_publishes_total");
    let mut name = base_name.replace("{topic}", topic.as_str());
    name = name.replace("/", "_");
    name = name.replace("-", "_");
    return name;
} 

pub const MONITORED_TOPIC_RECEIVED_PUBLISHES_DESC: &str = "The counter of the number of publishes received on topic {topic}";

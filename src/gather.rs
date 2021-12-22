use crate::constants;

use log::{error, info, warn};
use std::str;
use std::sync::mpsc;

pub fn run(receiver: mpsc::Receiver<paho_mqtt::message::Message>, teardown: mpsc::Receiver<bool>) {
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
        let value = match payload.parse::<f64>() {
            Ok(v) => v,
            Err(e) => {
                warn!("Can't convert data from {} to float: {}", topic, e);
                continue;
            }
        };

        println!("{} -> {}", topic, value);
    }
}

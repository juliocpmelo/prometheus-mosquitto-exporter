#[macro_use]
extern crate simple_error;

mod config;
mod constants;
mod logging;
mod mqtt;
mod usage;

use getopts::Options;
use log::error;
use std::sync::mpsc;
use std::{env, process};

fn main() {
    let argv: Vec<String> = env::args().collect();
    let mut options = Options::new();
    let mut log_level = log::LevelFilter::Info;

    options.optopt("c", "config", "Path to configuration file", "config_file");
    options.optflag("h", "help", "Show help text");
    options.optflag("q", "quiet", "Log only warn and error messages");
    options.optflag("V", "version", "Show version information");

    let opts = match options.parse(&argv[1..]) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: Can't parse command line arguments ({})", e);
            println!();
            usage::show_usage();
            process::exit(1);
        }
    };

    if opts.opt_present("h") {
        usage::show_usage();
        process::exit(0);
    }

    if opts.opt_present("V") {
        usage::show_version();
        process::exit(0);
    }

    if opts.opt_present("q") {
        log_level = log::LevelFilter::Warn;
    }

    let config_file = match opts.opt_str("c") {
        Some(v) => v,
        None => {
            eprintln!("Error: Missing configuration file");
            println!();
            usage::show_usage();
            process::exit(1);
        }
    };

    match logging::init(log_level) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: Initialisation of logging failed: {}", e);
            process::exit(1);
        }
    };

    let config = match config::parse_config_file(&config_file) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: Can't parse configuration file: {}", e);
            process::exit(1);
        }
    };

    let (send, receive) = mpsc::channel::<paho_mqtt::message::Message>();

    // TODO: Start consuming thread and pass it the receive end of the channel
    match mqtt::start_mqtt_client(&config, send) {
        Ok(_) => {}
        Err(e) => error!("MQTT client failed: {}", e),
    };
}

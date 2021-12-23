#[macro_use]
extern crate simple_error;
extern crate lazy_static;

mod config;
mod constants;
mod gather;
mod logging;
mod mqtt;
mod usage;

use getopts::Options;
use std::sync::mpsc;
use std::thread;
use std::{env, process};
use warp::Filter;

#[tokio::main]
async fn main() {
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

    // data channel MQTT -> data handler
    let (send, receive) = mpsc::channel::<paho_mqtt::message::Message>();

    // signal teardown for data handler thread
    let (td_send_svc, td_recv_svc) = mpsc::channel::<bool>();

    let gather_thread_id = thread::spawn(move || {
        gather::run(receive, td_recv_svc);
    });

    let mqtt_cfg = config.mqtt;
    let mqtt_thread_id = thread::spawn(move || {
        mqtt::start_mqtt_client(&mqtt_cfg, send);
    });

    // TODO: process server. metrics_path here
    let metrics = warp::path!("metrics")
        .and(warp::get())
        .map(gather::serve_metrics);

    // TODO: process server.listen here
    warp::serve(metrics).run(([127, 0, 0, 1], 6883)).await;

    // TODO: How to shutdown MQTT loop gracefully ?
    #[allow(unused_must_use)]
    {
        mqtt_thread_id.join();
    }

    // Shutdown data processing thread
    #[allow(unused_must_use)]
    {
        td_send_svc.send(true);
        gather_thread_id.join();
    }
}

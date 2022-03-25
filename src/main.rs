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
use log::error;
use std::sync::mpsc;
use std::{env, process, thread};
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
    let svc_cfg = match config.service {
        Some(v) => v,
        None => panic!("BUG: main: config.service is undefined"),
    };

    // TODO: process server. metrics_path here
    let metrics_path = match svc_cfg.metrics_path {
        Some(v) => v,
        None => panic!("BUG: config.service.metrics_path is undefined"),
    };
    let listen = match &svc_cfg.listen {
        Some(v) => v,
        None => panic!("BUG: main: config.service.listen is undefined"),
    };
    

    // data channel MQTT -> data handler
    let (send, receive) = mpsc::channel::<paho_mqtt::message::Message>();

    // signal teardown for data handler thread
    let (td_send_svc, td_recv_svc) = mpsc::channel::<bool>();

    let gather_thread_id = thread::spawn(move || {
        //TODO: add a communication channel between gather and mqtt_thread
        gather::run(receive, td_recv_svc, &svc_cfg.topic_listener.unwrap());
    });

    // start MQTT thread
    let mqtt_cfg = config.mqtt;
    let mqtt_thread_id = thread::spawn(move || {
        mqtt::start_mqtt_client(&mqtt_cfg, &svc_cfg, send);
    });

    

    
    let url_path = metrics_path.as_str().trim_start_matches('/').to_string();
    let root_html = format!("<html>\n<head><title>Mosquitto exporter</title></head>\n<body>\n<h1>Mosquitto exporter</h1>\n<p><a href=\"/{}\">Metrics</a></p>\n</body>\n</html>\n", url_path);

    
    let socketaddr = match config::socketaddr_from_listen(listen.to_string()) {
        Ok(v) => v,
        Err(e) => {
            error!("{}", e);
            process::exit(1);
        }
    };

    let prometheus_route = warp::path(url_path)
        .and(warp::get())
        .map(gather::serve_metrics);

    let root_route = warp::path::end()
        .and(warp::get())
        .map(move || warp::reply::html(root_html.to_string()));

    let route = root_route.or(prometheus_route);

    // XXX: async rust with tokio might provide a better solution enable graceful shutdown
    //      e.g. https://docs.rs/warp/latest/warp/struct.Server.html#method.bind_with_graceful_shutdown
    warp::serve(route).run(socketaddr).await;

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

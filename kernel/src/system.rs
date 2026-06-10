use crate::config::server_config;
use chrono::Local;
use std::{env, process};

pub fn show() {
    show_logo();

    let config = server_config();

    #[cfg(target_os = "windows")]
    let system_name = env::var("OS").unwrap().to_string();
    #[cfg(not(target_os = "windows"))]
    let system_name = std::env::consts::OS;

    let socket_url = format!("{}:{}{}", config.host, config.port, config.ws_path);

    println!("{:>5}: {}", "系统架构", env::consts::ARCH);
    println!("{:>5}: {}", "操作系统", system_name);
    println!("{:>2}: {}", "CPU核心数", num_cpus::get());
    println!("{:>5}: {}", "服务进程", process::id());
    println!("{:>7}: http://{}:{}", "API服务", config.host, config.port);
    if config.ws_open {
        println!("{:>7}: ws://{}", "WS服务", socket_url);
    }
    println!(
        "{:>5}: {}",
        "启动时间",
        Local::now().format("%Y-%m-%d %H:%M:%S")
    );

    println!()
}

fn show_logo() {
    let logo = r#"

██████  ███████ ███    ██  ██████  ███████ ██    ██
██   ██ ██      ████   ██ ██       ██       ██  ██
██████  █████   ██ ██  ██ ██   ███ ███████   ████
██      ██      ██  ██ ██ ██    ██      ██    ██
██      ███████ ██   ████  ██████  ███████    ██

    "#;
    println!("{}", logo);
}

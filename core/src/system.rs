use common::config::server_config;
use std::{env, process};

pub fn show() {
    show_logo();

    let config = server_config();

    println!("{:>2} Axum [v0.8.8] 服务启动成功!!!", "🎉🎉🎉");
    println!();

    #[cfg(target_os = "windows")]
    let system_name = env::var("OS").unwrap().to_string();
    #[cfg(not(target_os = "windows"))]
    let system_name = std::env::consts::OS;

    let socket_url = format!("{}:{}{}", config.host, config.port, config.ws_path);
    let start_time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");

    println!("{:>2}: {}", "系统架构", env::consts::ARCH);
    println!("{:>2}: {}", "操作系统", system_name);
    println!("{:>2}: {}", "服务进程", process::id());
    println!("{:>6}: http://{}:{}", "API服务", config.host, config.port);
    if config.ws_open {
        println!("{:>6}: ws://{}", "WS服务", socket_url);
    }
    println!("{:>2}: {}", "启动时间", start_time);

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

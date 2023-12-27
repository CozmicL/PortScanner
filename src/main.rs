use std::net::TcpListener;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::Read;
use regex::Regex;

fn text_file_scan(ip: &str, path: &str) {
    let path_buf = PathBuf::from(path);
    let file = OpenOptions::new().read(true).open(&path_buf).expect("Error opening file");
    let mut buffer = String::new();
    file.take(1000).read_to_string(&mut buffer).expect("Error reading file");

    for port in buffer.lines() {
        if let Ok(listener) = TcpListener::bind(format!("{}:{}", ip, port)) {
            println!("Port {} is open", port);
            drop(listener);
        } else {
            println!("Port {} is not open", port);
        }
    }
}

fn range_scan(ip: &str, start_port: i32, end_port: i32) {
    for port in start_port..=end_port {
        if let Ok(listener) = TcpListener::bind(format!("{}:{}", ip, port)) {
            println!("Port {} is open", port);
            drop(listener);
        } else {
            println!("Port {} is not open", port);
        }
    }
}

fn parse_port(arg: &str) -> i32 {
    arg.parse().expect("Invalid port number")
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    match args.len() {
        3 => {
            let ip = &args[1];
            let re = Regex::new(r"^\d+\.\d+\.\d+\.\d+$").expect("Error creating regex");
            if !re.is_match(ip) {
                panic!("Please specify a valid IP address: {}", ip);
            }
            let path = &args[2];

            println!("Starting Text File Scan");
            text_file_scan(ip, path);
        }
        4 => {
            let ip = &args[1];
            let re = Regex::new(r"^\d+\.\d+\.\d+\.\d+$").expect("Error creating regex");
            if !re.is_match(ip) {
                panic!("Please specify a valid IP address: {}", ip);
            }

            let start_port = parse_port(&args[2]);
            let end_port = parse_port(&args[3]);

            println!("Starting Range Scan");
            range_scan(ip, start_port, end_port);
        }
        _ => {
            panic!("Please specify util");
        }
    }
}
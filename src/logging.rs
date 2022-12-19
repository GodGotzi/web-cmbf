extern crate chrono;

use std::fmt;
use std::fmt::Formatter;
use actix_web::HttpRequest;
use chrono::Local;

#[allow(dead_code)]
pub enum LogLevel {
    INFO,
    WARN,
    ERR
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::INFO => write!(f, "INFO"),
            LogLevel::WARN => write!(f, "WARN"),
            LogLevel::ERR => write!(f, "ERR")
        }
    }
}

pub fn log_info(msg: &str, level: LogLevel) {
    let now = Local::now();

    let time_str = now.format("[%Y-%m-%d] [%H:%M:%S]");

    println!("{} {} Message -> {}", time_str, level, msg);
}

pub fn log_request(req: &HttpRequest) {
    log_info(format!("Request -> [ip: {}]", req.peer_addr().unwrap()).as_str(),
             LogLevel::INFO);
}

pub fn log_response(resp: &str) {
    log_info(format!("Response -> [filepath: {}]", resp).as_str(), LogLevel::INFO);
}

extern crate chrono;

use actix_web::HttpRequest;
use chrono::Local;

pub fn log_info(msg: &str) {
    let now = Local::now();

    let time_str = now.format("[%Y-%m-%d] [%H:%M:%S]");

    println!("{} Message -> {}", time_str, msg);
}

pub fn log_request(req: &HttpRequest) {
    log_info(format!("Request -> [ip: {}]", req.peer_addr().unwrap()).as_str());
}

pub fn log_response(resp: &str) {
    log_info(format!("Response -> [filepath: {}]", resp).as_str());
}

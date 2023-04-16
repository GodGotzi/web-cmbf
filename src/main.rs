extern crate core;

mod language;
mod properties;
mod logging;

use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};
use std::path::PathBuf;
use actix_web::http::header::ACCEPT_LANGUAGE;
use actix_web::{web, App, HttpServer};
use lazy_static::lazy_static;

//#[allow(unused_imports)]
//use openssl::ssl::{SslAcceptor, SslMethod, SslFiletype};

use crate::language::{get_language, get_path, LanguageProperties};
use crate::properties::Properties;
use crate::logging::{log_info, log_request, log_response, LogLevel};

lazy_static! {
    static ref PROPERTIES: Properties = Properties::new("frontend/env.properties");
    static ref LANGUAGE_PROPERTIES: LanguageProperties<'static> = LanguageProperties::new("frontend/env.properties", &PROPERTIES);
}

async fn index(req: HttpRequest) -> Result<NamedFile> {
    log_request(&req);

    let language_str = match req.headers().get(ACCEPT_LANGUAGE) {
        Some(header) => {
            let mut language_split = get_language(header).split(";");
            let language_str = language_split.next().unwrap();

            language_str
        },
        None => {
            log_info("Invalid Language", LogLevel::WARN);
            "en"
        } 
    };

    let str: String = get_path(req.path(), language_str, &LANGUAGE_PROPERTIES);

    log_response(str.as_str());
    Ok(NamedFile::open(PathBuf::from(str))?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log_info("Starting Webserver...", LogLevel::INFO);
    log_info("Waiting for Requests", LogLevel::INFO);

    /*
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();
    */

    HttpServer::new(|| App::new().route("/{filename:.*}", web::get().to(index)))
        .workers(40)
        .bind(("5.132.159.90", 33333))? //172.20.10.2:3000
        .run()
        .await
}

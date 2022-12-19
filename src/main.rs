extern crate core;

mod language;
mod properties;
mod logging;

use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};
use std::path::PathBuf;
use actix_web::http::header::ACCEPT_LANGUAGE;
use actix_web::{web, App, HttpServer};
use crate::language::{get_language, get_path, LanguageProperties};
use crate::properties::Properties;
use lazy_static::lazy_static;
use crate::logging::{log_info, log_request, log_response};

lazy_static! {
    static ref PROPERTIES: Properties = Properties::new("frontend/env.properties");
    static ref LANGUAGE_PROPERTIES: LanguageProperties<'static> = LanguageProperties::new("frontend/env.properties", &PROPERTIES);
}

async fn index(req: HttpRequest) -> Result<NamedFile> {
    log_request(&req);

    let language_value = req.headers().get(ACCEPT_LANGUAGE).unwrap();
    let language_str = get_language(language_value);
    let str: String = get_path(req.path(), language_str, &LANGUAGE_PROPERTIES);

    log_response(str.as_str());
    Ok(NamedFile::open(PathBuf::from(str))?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log_info("Starting Webserver...");
    log_info("Waiting for Requests");

    HttpServer::new(|| App::new().route("/{filename:.*}", web::get().to(index)))
        .workers(20)
        .bind(("192.168.1.185", 3000))?
        .run()
        .await
}
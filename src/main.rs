use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};
use std::path::PathBuf;

async fn index(req: HttpRequest) -> Result<NamedFile> {
    let str = match req.path() {
         "/" => String::from("frontend/home.html"),
         _ => format!("frontend{}", req.path()),
    };

    println!("Computing Path -> {}", str);
    Ok(NamedFile::open(PathBuf::from(str))?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| App::new().route("/{filename:.*}", web::get().to(index)))
        .workers(10)
        .bind(("127.0.0.1", 7000))?
        .run()
        .await
}
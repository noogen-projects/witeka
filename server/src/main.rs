#[macro_use]
extern crate diesel;

use crate::settings::Settings;
use actix_files as fs;
use actix_web::{App, HttpServer};
use dao::user::ConnectionsPool;
use diesel::r2d2::{ConnectionManager, Pool};
use http::api;
use log::info;

mod core;
mod dao;
mod http;
mod settings;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let settings = Settings::new().expect("Error read settings");
    env_logger::builder().filter_level(settings.log_level).init();

    let manager = ConnectionManager::new(&settings.database_url);
    let pool: ConnectionsPool = Pool::new(manager).expect("Error creating connection pool");

    info!("Application started on {}", settings.app_address);
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(api::v1::user)
            .service(api::v1::group)
            .service(api::v1::space)
            .service(fs::Files::new("/", "./static/").index_file("index.html"))
    })
    .bind(&settings.app_address)?
    .run()
    .await
}

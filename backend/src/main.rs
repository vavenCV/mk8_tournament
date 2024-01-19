#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

extern crate r2d2_diesel;
extern crate serde_json;
// ...
mod db;
mod services;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web::JsonConfig, App, HttpServer};
    let conn_pool = db::establish_connection();
    HttpServer::new(move || {
        App::new()
            .data(conn_pool.clone())
            .data(JsonConfig::default().limit(4096))
            .configure(services::user::init_routes)
    })
    .bind("0.0.0.0:5000")?
    .run()
    .await
}

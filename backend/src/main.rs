use actix_web::dev::Server;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

extern crate r2d2_diesel;
extern crate serde_json;
// ...
mod db;
mod services;
mod utils;

// mod auth;
mod tests;

fn main() -> std::io::Result<()> {
    use actix_cors::Cors;
    use actix_web::{web::JsonConfig, App, HttpServer};
    let conn_pool = db::establish_connection();

    let mut sys = actix_rt::System::new("server");

    let srv: Server = HttpServer::new(move || {
        App::new()
            .data(conn_pool.clone())
            .data(JsonConfig::default().limit(4096))
            .wrap(Cors::permissive())
            .configure(services::players::init_routes)
            .configure(services::teams::init_routes)
            .configure(services::faceoffs::init_routes)
            .configure(services::races::init_routes)
    })
    .bind("0.0.0.0:5000")?
    .run();

    println!("🍄🍄 Mario Kart 🍄🍄 Server started on localhost:5000");

    sys.block_on(srv)
}

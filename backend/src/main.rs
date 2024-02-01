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

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web::JsonConfig, App, HttpServer};
    use actix_cors::Cors;
    let conn_pool = db::establish_connection();
    HttpServer::new(move || {
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
    .run()
    .await
}

#[cfg(test)]
mod main_tests {
    use std::thread;

    use actix_web::dev::{Body, JsonBody};
    use actix_web::http::header;
    use actix_web::test::{self, read_body_json};
    use actix_web::web;

    use crate::db::establish_connection;
    use crate::db::model::player::Player;
    use crate::services::teams::TeamForm;
    use crate::{main, services};

    const SERVER_URL: &str = "http://localhost:5000";

    // #[test]
    // fn create_env() {

    //     // let resp = reqwest::blocking::get(format!("{SERVER_URL}/players")).unwrap().json::<Vec<Player>>().unwrap();
    //     // dbg!(resp);
    //     let mut conn = establish_connection();

    //     let mut resp = services::teams::create(
    //         actix_web::web::Json(TeamForm {
    //             player_names: [
    //                 "p1".to_string(),
    //                 "p2".to_string(),
    //                 "p3".to_string(),
    //                 "p4".to_string(),
    //             ],
    //         }),
    //         web::Data::new(conn),
    //     );

    // assert!(resp.status().is_success());

    // let result: Person = test::read_body_json(resp).await;
    
    //     // let mut resp = services::players::index(web::Data::new(conn));
    //     // // let body = to_bytes(races.into_body()).await.unwrap();
    //     let body = resp.take_body();
    //     let body = body.as_ref().unwrap();
    //     // let json = serde_json::from_slice(body).unwrap();
    //     // // match body.as_ref().unwrap() {
    //     // //     Body::Message(msg) => println!("{:?}", msg),
    //     // //     _ => {}
    //     // // };
    //     dbg!(body);
    //     let v = read_body_json(resp).await;
    //     // assert!(resp.status().is_success());
    //     // // assert_eq!(
    //     // //     &Body::from(json!({"name":"Test"})), // or serde.....
    //     // //     body
    //     // // );
    // }
}

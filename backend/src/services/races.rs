// src/services/user.rs
use crate::db::{model::player::Player, DbPool};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct RaceForm {
    name: Option<String>,
}
pub fn create(race_form: web::Json<RaceForm>, pool: web::Data<DbPool>) -> HttpResponse {
    let mut conn = pool.get().unwrap();
    match Player::create(&race_form.name.clone().unwrap(), None, &mut conn) {
        Some(user) => HttpResponse::Ok().json(user),
        _ => HttpResponse::InternalServerError().json("Could not create user"),
    }
}
pub fn index(pool: web::Data<DbPool>) -> HttpResponse {
    let mut conn = pool.get().unwrap();
    HttpResponse::Ok().json(Player::list(&mut conn))
}
pub fn get(id: web::Path<i32>, pool: web::Data<DbPool>) -> HttpResponse {
    let conn = pool.get().unwrap();
    match Player::by_id(&id, &conn) {
        Some(user) => HttpResponse::Ok().json(user),
        _ => HttpResponse::NotFound().json("Not Found"),
    }
}
pub fn update(id: web::Path<i32>, race_form: web::Json<RaceForm>, pool: web::Data<DbPool>) {}
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    /*
     * index: curl -i -X GET -H "Content-Type: application/json" http://localhost:5000/users
     * get: curl -i -X GET -H "Content-Type: application/json" http://localhost:5000/users/<id>
     * post: curl -i -X POST -H "Content-Type: application/json" -d '{"email":"xxx"}' http://localhost:5000/users
     */

    cfg.service(
        web::resource("/races")
            .route(web::post().to(create))
            .route(web::get().to(index))
            .route(web::put().to(index)),
    )
    .service(web::scope("/races").route("/{id}", web::get().to(get)));
}

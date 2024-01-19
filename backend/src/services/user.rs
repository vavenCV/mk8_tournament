// src/services/user.rs
use crate::db::{model::Player, DbPool};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct PlayerForm {
    name: Option<String>,
}
pub fn create(player_form: web::Json<PlayerForm>, pool: web::Data<DbPool>) -> HttpResponse {
    let mut conn = pool.get().unwrap();
    match Player::create(player_form.name.as_deref(), &mut conn) {
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
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    /*
     * index: curl -i -X GET -H "Content-Type: application/json" http://localhost:5000/users
     * get: curl -i -X GET -H "Content-Type: application/json" http://localhost:5000/users/<id>
     * post: curl -i -X POST -H "Content-Type: application/json" -d '{"email":"xxx", "phone": "yyy"}' http://localhost:5000/users
     */

    cfg.service(
        web::resource("/users")
            .route(web::post().to(create))
            .route(web::get().to(index)),
    )
    .service(web::scope("/users").route("/{id}", web::get().to(get)));
}

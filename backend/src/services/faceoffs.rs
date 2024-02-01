// src/services/user.rs
use crate::db::{
    model::{faceoff::Faceoff},
    DbPool,
};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct FaceoffForm {
    pub race_number: i32,
    pub team_ids: Vec<i32>,
}
#[derive(Serialize, Deserialize)]
pub struct FaceoffTeamUpdate {
    pub team_ids: Vec<i32>,
}
pub fn create(faceoff_form: web::Json<FaceoffForm>, pool: web::Data<DbPool>) -> HttpResponse {
    let mut conn = pool.get().unwrap();
    match Faceoff::create(
        faceoff_form.race_number,
        faceoff_form.team_ids.clone(),
        &mut conn,
    ) {
        Some(user) => HttpResponse::Ok().json(user),
        _ => HttpResponse::InternalServerError().json("Could not create faceoff"),
    }
}
pub fn index(pool: web::Data<DbPool>) -> HttpResponse {
    let mut conn = pool.get().unwrap();
    HttpResponse::Ok().json(Faceoff::list(&mut conn))
}
pub fn get(id: web::Path<i32>, pool: web::Data<DbPool>) -> HttpResponse {
    let conn = pool.get().unwrap();
    match Faceoff::by_id(&id, &conn) {
        Some(user) => HttpResponse::Ok().json(user),
        _ => HttpResponse::NotFound().json("Not Found"),
    }
}
pub fn generate(id: web::Path<i32>, pool: web::Data<DbPool>) -> HttpResponse {
    let mut conn = pool.get().unwrap();
    match Faceoff::generate_races(&id, &mut conn) {
        Ok(faceoff) => HttpResponse::Ok().json(faceoff),
        _ => HttpResponse::NotFound().json("Not Found"),
    }
}
pub fn update(id: web::Path<i32>, faceoff_form: web::Json<FaceoffTeamUpdate>, pool: web::Data<DbPool>) -> HttpResponse {
    let mut conn = pool.get().unwrap();
    match Faceoff::set_team_ids(&id, &faceoff_form.team_ids, &mut conn) {
        Ok(faceoff) => HttpResponse::Ok().json(faceoff),
        _ => HttpResponse::NotFound().json("Not Found"),
    }
}
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    /*
     * index: curl -i -X GET -H "Content-Type: application/json" http://localhost:5000/faceoffs
     * get: curl -i -X GET -H "Content-Type: application/json" http://localhost:5000/faceoffs/<id>
     * post: curl -i -X POST -H "Content-Type: application/json" -d '{"race_number":6, "team_ids": [X, Y, Z]}' http://localhost:5000/faceoffs
     */

    cfg.service(
        web::resource("/faceoffs")
            .route(web::post().to(create))
            .route(web::get().to(index)),
    )
    .service(
        web::scope("/faceoffs")
        
        .route("/{id}/generate", web::post().to(generate))
            .route("/{id}", web::get().to(get))
            .route("/{id}", web::put().to(update)),
    );
}

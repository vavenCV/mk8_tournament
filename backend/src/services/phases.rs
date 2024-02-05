// src/services/user.rs
use crate::db::{model::phase::Phase, DbPool};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct PhaseForm {
    pub phase_number: i32,
    pub faceoff_count: i32,
}
#[derive(Serialize, Deserialize)]
pub struct PhaseTeamUpdate {
    pub team_ids: Vec<i32>,
}
pub fn create(phase_form: web::Json<PhaseForm>, pool: web::Data<DbPool>) -> HttpResponse {
    let mut conn = pool.get().unwrap();
    match Phase::create(phase_form.phase_number, phase_form.faceoff_count, &mut conn) {
        Some(user) => HttpResponse::Ok().json(user),
        _ => HttpResponse::InternalServerError().json("Could not create phase"),
    }
}
pub fn index(pool: web::Data<DbPool>) -> HttpResponse {
    let mut conn = pool.get().unwrap();
    HttpResponse::Ok().json(Phase::list(&mut conn))
}
pub fn get(id: web::Path<i32>, pool: web::Data<DbPool>) -> HttpResponse {
    let conn = pool.get().unwrap();
    match Phase::by_id(&id, &conn) {
        Some(user) => HttpResponse::Ok().json(user),
        _ => HttpResponse::NotFound().json("Not Found"),
    }
}
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    /*
     * index: curl -i -X GET -H "Content-Type: application/json" http://localhost:5000/phases
     * get: curl -i -X GET -H "Content-Type: application/json" http://localhost:5000/phases/<id>
     * post: curl -i -X POST -H "Content-Type: application/json" -d '{"race_number":6, "team_ids": [X, Y, Z]}' http://localhost:5000/phases
     */

    cfg.service(
        web::resource("/phases")
            .route(web::post().to(create))
            .route(web::get().to(index)),
    )
    .service(web::scope("/phases").route("/{id}", web::get().to(get)));
}

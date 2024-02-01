use std::ops::Deref;

// src/services/user.rs
use crate::db::{model::team::Team, DbPool};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TeamForm {
    pub team_name: String,
    pub player_names: [String; 4],
}
pub fn create(team_form: web::Json<TeamForm>, pool: web::Data<DbPool>) -> HttpResponse {
    let mut conn = pool.get().unwrap();

    let mut str_names: [&str; 4] = ["", "", "", ""];
    for (index, name) in team_form.player_names.iter().enumerate() {
        str_names[index] = &name;
    }

    match Team::create(team_form.team_name.deref().to_owned(), str_names, &mut conn) {
        Some(user) => HttpResponse::Ok().json(user),
        _ => HttpResponse::InternalServerError().json("Could not create team"),
    }
}
pub fn index(pool: web::Data<DbPool>) -> HttpResponse {
    let mut conn = pool.get().unwrap();
    HttpResponse::Ok().json(Team::list(&mut conn))
}
pub fn get(id: web::Path<i32>, pool: web::Data<DbPool>) -> HttpResponse {
    let conn = pool.get().unwrap();
    match Team::by_id(&id, &conn) {
        Some(user) => HttpResponse::Ok().json(user),
        _ => HttpResponse::NotFound().json("Not Found"),
    }
}
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    /*
     * index: curl -i -X GET -H "Content-Type: application/json" http://localhost:5000/teams
     * get: curl -i -X GET -H "Content-Type: application/json" http://localhost:5000/teams/<id>
     * post: curl -i -X POST -H "Content-Type: application/json" -d '{"player_names":["xxx", "xxx", "xxx", "xxx"]}' http://localhost:5000/teams
     */

    cfg.service(
        web::resource("/teams")
            .route(web::post().to(create))
            .route(web::get().to(index)),
    )
    .service(web::scope("/teams").route("/{id}", web::get().to(get)));
}

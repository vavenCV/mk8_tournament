use std::ops::Deref;

// src/services/user.rs
use crate::db::{
    model::{player::Player, race::Race, race_point::RacePoints},
    DbPool,
};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RaceStatus {
    pub id: i32,
    pub is_ended: bool,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RacePointForm {
    pub player_id: i32,
    pub points: u8,
}
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct RaceForm {
    pub race_points: Vec<RacePointForm>,
}
pub fn index(pool: web::Data<DbPool>) -> HttpResponse {
    let mut conn = pool.get().unwrap();
    HttpResponse::Ok().json(Race::list(&mut conn))
}
pub fn get(id: web::Path<i32>, pool: web::Data<DbPool>) -> HttpResponse {
    let conn = pool.get().unwrap();
    match Race::by_id(&id, &conn) {
        Some(race) => HttpResponse::Ok().json(race),
        _ => HttpResponse::NotFound().json("Not Found"),
    }
}
pub fn status(id: web::Path<i32>, pool: web::Data<DbPool>) -> HttpResponse {
    let mut conn = pool.get().unwrap();
    match Race::are_all_points_entered(&id, &mut conn) {
        Ok(is_ended) => {
            HttpResponse::Ok().json(RaceStatus{ id: *id, is_ended: is_ended })
        },
        _ => HttpResponse::NotFound().json("Not Found"),
    }
}
pub fn update(
    id: web::Path<i32>,
    race_form: web::Json<RaceForm>,
    pool: web::Data<DbPool>,
) -> HttpResponse {
    let mut conn = pool.get().unwrap();

    let non_null_race_points = race_form
        .race_points
        .iter()
        .filter(|rp| Player::by_id(&rp.player_id, &conn).is_some())
        .collect::<Vec<&RacePointForm>>();

    let race_point_ids = non_null_race_points
        .iter()
        .map(|rp| {
            RacePoints::create(rp.player_id, *id.deref(), rp.points, &mut conn)
                .unwrap()
                .id
        })
        .collect::<Vec<i32>>();

    Race::set_racepoint_ids(*id, &race_point_ids, &mut conn);
    get(id, pool)
}
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    /*
     * index: curl -i -X GET -H "Content-Type: application/json" http://localhost:5000/races
     * get: curl -i -X GET -H "Content-Type: application/json" http://localhost:5000/races/<id>
     */

    cfg.service(web::resource("/races").route(web::get().to(index)))
        .service(
            web::scope("/races")
                .route("/{id}", web::put().to(update))
                .route("/{id}", web::get().to(get))
                .route("/{id}/status", web::get().to(status)),
        );
}
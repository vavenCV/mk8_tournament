// src/services/user.rs
use crate::db::{
    model::{player::Player, race::Race, race_point::RacePoints},
    DbPool,
};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerPointsResp {
    pub total_points: u32,
}
#[derive(Serialize, Deserialize)]
pub struct PlayerForm {
    pub team_id: i32,
    pub name: String,
}
pub fn create(player_form: web::Json<PlayerForm>, pool: web::Data<DbPool>) -> HttpResponse {
    let mut conn = pool.get().unwrap();
    match Player::create(&player_form.name.clone(), player_form.team_id, &mut conn) {
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
pub fn get_total_points(id: web::Path<i32>, pool: web::Data<DbPool>) -> HttpResponse {
    let conn = pool.get().unwrap();

    match RacePoints::by_player_id(&id, &conn) {
        Some(race_points) => {
            let total_points: u32 = race_points.iter().map(|r| r.points as u32).sum();
            HttpResponse::Ok().json(PlayerPointsResp { total_points })
        }
        _ => HttpResponse::NotFound().json("Not Found"),
    }
}
pub fn get_total_points_in_faceoff(
    info: web::Path<(i32, i32)>,
    pool: web::Data<DbPool>,
) -> HttpResponse {
    let conn = pool.get().unwrap();
    let (player_id, faceoff_id) = info.into_inner();
    match RacePoints::by_player_id(&player_id, &conn) {
        Some(race_points) => {
            let total_points: u32 = race_points
                .iter()
                .filter(|r| {
                    Race::by_id(&r.race_id, &conn)
                        .unwrap()
                        .faceoff_id
                        .eq(&Some(faceoff_id))
                })
                .map(|r| r.points as u32)
                .sum();
            HttpResponse::Ok().json(PlayerPointsResp { total_points })
        }
        _ => HttpResponse::NotFound().json("Not Found"),
    }
}
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    /*
     * index: curl -i -X GET -H "Content-Type: application/json" http://localhost:5000/players
     * get: curl -i -X GET -H "Content-Type: application/json" http://localhost:5000/players/<id>
     * post: curl -i -X POST -H "Content-Type: application/json" -d '{"name":"xxx"}' http://localhost:5000/players
     */

    cfg.service(
        web::resource("/players")
            .route(web::post().to(create))
            .route(web::get().to(index)),
    )
    .service(
        web::scope("/players/{id}")
            .route("", web::get().to(get))
            .route("/total_points", web::get().to(get_total_points))
            .route(
                "/total_points_in_faceoff/{faceoff_id}",
                web::get().to(get_total_points_in_faceoff),
            ),
    );
}

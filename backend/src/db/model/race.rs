use std::collections::HashMap;

use crate::db::schema::races::dsl::races as race_dsl;
use crate::{db::schema::races, utils};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "races"]

pub struct Race {
    pub id: i32,
    pub team_ids: Option<String>,
    pub faceoff_id: Option<i32>,
    pub race_point_ids: Option<String>,
}

impl Race {
    pub fn list(conn: &mut SqliteConnection) -> Vec<Self> {
        race_dsl.load::<Race>(conn).expect("Error loading races")
    }
    pub fn by_id(id: &i32, conn: &SqliteConnection) -> Option<Self> {
        if let Ok(record) = race_dsl.find(id).get_result::<Race>(conn) {
            Some(record)
        } else {
            None
        }
    }

    /* Get every Race in a faceoff (Circuit 1, 2 ,3 etc) */
    pub fn by_faceoff_id(query_faceoff_id: &i32, conn: &SqliteConnection) -> Option<Vec<Race>> {
        use crate::db::schema::races::dsl::faceoff_id;

        if let Ok(record) = race_dsl
            .filter(faceoff_id.eq(query_faceoff_id))
            .load::<Race>(conn)
        {
            Some(record)
        } else {
            None
        }
    }

    /* Get every Race in a faceoff (Circuit 1, 2 ,3 etc) */
    pub fn by_team_id(query_team_id: &i32, conn: &SqliteConnection) -> Option<Vec<Race>> {
        use crate::db::schema::races::dsl::team_ids;

        if let Ok(record) = race_dsl
            .filter(team_ids.like(format!("%{}%", query_team_id)))
            .load::<Race>(conn)
        {
            Some(record)
        } else {
            None
        }
    }

    pub fn by_racepoint_id(query_racepoint_id: &i32, conn: &SqliteConnection) -> Option<Race> {
        use crate::db::schema::races::dsl::race_point_ids;

        if let Ok(record) = race_dsl
            .filter(race_point_ids.like(format!("%{}%", query_racepoint_id)))
            .first::<Race>(conn)
        {
            Some(record)
        } else {
            None
        }
    }

    pub fn by_faceoff_and_racepoint_ids(
        query_faceoff_id: &i32,
        query_race_point_ids: &Vec<i32>,
        conn: &SqliteConnection,
    ) -> Option<Self> {
        use crate::db::schema::races::dsl::faceoff_id;
        use crate::db::schema::races::dsl::race_point_ids;

        let str_race_point_ids = utils::ids::ids_to_string(Some(query_race_point_ids.to_vec()));

        if let Ok(record) = race_dsl
            .filter(faceoff_id.eq(query_faceoff_id))
            .filter(race_point_ids.eq(str_race_point_ids))
            .first::<Race>(conn)
        {
            Some(record)
        } else {
            None
        }
    }

    pub fn create(
        team_ids: Vec<i32>,
        faceoff_id: Option<i32>,
        race_point_ids: Option<Vec<i32>>,
        conn: &mut SqliteConnection,
    ) -> Option<Self> {
        let new_id = Uuid::new_v4().as_u128() as i32;

        if faceoff_id.is_some() && race_point_ids.is_some() {
            if let Some(player) = Self::by_faceoff_and_racepoint_ids(
                &faceoff_id.unwrap(),
                &race_point_ids.clone().unwrap(),
                conn,
            ) {
                return Some(player);
            }
        }
        let new_race = Self::new_race_struct(&new_id, team_ids, faceoff_id, race_point_ids);

        diesel::insert_into(race_dsl)
            .values(&new_race)
            .execute(conn)
            .expect("Error saving new new_race");
        Self::by_id(&new_id, conn)
    }

    fn new_race_struct(
        id: &i32,
        team_ids: Vec<i32>,
        faceoff_id: Option<i32>,
        race_point_ids: Option<Vec<i32>>,
    ) -> Self {
        Race {
            id: *id,
            team_ids: Some(utils::ids::ids_to_string(Some(team_ids))),
            faceoff_id: faceoff_id,
            race_point_ids: Some(utils::ids::ids_to_string(race_point_ids)),
        }
    }

    pub fn set_faceoff_id(query_id: i32, new_faceoff_id: i32, conn: &mut SqliteConnection) {
        use crate::db::schema::races::dsl::faceoff_id;
        use crate::db::schema::races::dsl::id;

        let updated_row = diesel::update(race_dsl.filter(id.eq(query_id)))
            .set(faceoff_id.eq(new_faceoff_id))
            .execute(conn);
    }

    pub fn set_racepoint_ids(query_id: i32, racepoint_ids: &[i32], conn: &mut SqliteConnection) {
        use crate::db::schema::races::dsl::id;
        use crate::db::schema::races::dsl::race_point_ids;

        let str_racepoint_ids = utils::ids::ids_to_string(Some(racepoint_ids.to_vec()));

        println!("{:?}", str_racepoint_ids);
        let updated_row = diesel::update(race_dsl.filter(id.eq(query_id)))
            .set(race_point_ids.eq(str_racepoint_ids))
            .execute(conn);
    }
}
#[cfg(test)]
mod player_test {
    use crate::{
        db::{
            establish_connection,
            model::{player::Player, race::Race, race_point::RacePoints, team::Team},
        },
        utils,
    };
    #[test]
    fn create_race() {
        let mut conn = establish_connection().get().unwrap();

        let player_name = "[GRE] p1";
        let points = 15;

        let player = Player::create(player_name, None, &mut conn).unwrap();

        let team = Team::create(["P1", "P2", "P3", "P4"], &mut conn).unwrap();

        let mut race = Race::create(vec![team.id], None, None, &mut conn).unwrap();

        let race_points = RacePoints::create(player.id, race.id, points, &mut conn).unwrap();

        Race::set_racepoint_ids(race.id, &[race_points.id], &mut conn);
        race = Race::by_id(&race.id, &conn).unwrap();

        assert_eq!(race_points.points as u8, points);
        assert_eq!(
            utils::ids::string_to_ids(race.race_point_ids.unwrap())
                .unwrap()
                .first()
                .unwrap(),
            &race_points.id
        );
    }
}
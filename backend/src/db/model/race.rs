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
    pub faceoff_id: Option<i32>,
    pub race_point_ids: String,
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

    pub fn by_racepoint_id(query_racepoint_id: &i32, conn: &SqliteConnection) -> Option<Vec<Race>> {
        use crate::db::schema::races::dsl::race_point_ids;

        // let str_race_point_ids = utils::ids::ids_to_string(Some(query_race_point_ids.to_vec()));

        if let Ok(record) = race_dsl.filter(race_point_ids.eq("")).load::<Race>(conn) {
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
        let new_race = Self::new_race_struct(&new_id, faceoff_id, race_point_ids);

        diesel::insert_into(race_dsl)
            .values(&new_race)
            .execute(conn)
            .expect("Error saving new new_race");
        Self::by_id(&new_id, conn)
    }

    fn new_race_struct(
        id: &i32,
        faceoff_id: Option<i32>,
        race_point_ids: Option<Vec<i32>>,
    ) -> Self {
        Race {
            id: *id,
            faceoff_id: faceoff_id,
            race_point_ids: utils::ids::ids_to_string(race_point_ids),
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
            model::{player::Player, race::Race, race_point::RacePoints},
        },
        utils,
    };
    #[test]
    fn create_race() {
        let mut conn = establish_connection().get().unwrap();

        let player_name = "[GRE] p1";
        let points = 15;

        let player = Player::create(player_name, None, &mut conn).unwrap();
        let mut race = Race::create(None, None, &mut conn).unwrap();

        let race_points =
            RacePoints::create(Some(player.id), Some(race.id), Some(points), &mut conn).unwrap();
        println!("{:?}", race);
        println!("{:?}", race_points);

        Race::set_racepoint_ids(race.id, &[race_points.id], &mut conn);
        race = Race::by_id(&race.id, &conn).unwrap();

        println!("{:?}", race);

        assert_eq!(race_points.points, Some(points));
        assert_eq!(
            utils::ids::string_to_ids(race.race_point_ids)
                .unwrap()
                .first()
                .unwrap(),
            &race_points.id
        );
    }
    // #[test]
    // fn create_player_with_existing_name() {
    //     let mut conn = establish_connection().get().unwrap();
    //     let name = Some("[GRE] p1");
    //     let player = Race::create(name, &mut conn).unwrap();
    //     let existing_player = Race::create(name, &mut conn).unwrap();
    //     assert_eq!(player.id, existing_player.id);
    // }
    // #[test]
    // fn list_players() {
    //     let mut conn = establish_connection().get().unwrap();
    //     let name = Some("[GRE] p1");
    //     let player = Race::create(name, &mut conn).unwrap();
    //     let existing_players = Race::list(&mut conn);
    //     assert_eq!(1, existing_players.len());
    //     assert_eq!(player.id, existing_players[0].id);
    // }
    // #[test]
    // fn get_player_by_name() {
    //     let mut conn = establish_connection().get().unwrap();
    //     let name = Some("[GRE] p1");
    //     let player = Race::create(name, &mut conn).unwrap();
    //     let existing_player = Race::by_name(&name.unwrap(), &conn).unwrap();
    //     assert_eq!(player.id, existing_player.id);
    // }
    // #[test]
    // fn get_player_by_id() {
    //     let mut conn = establish_connection().get().unwrap();
    //     let name = Some("[GRE] p1");
    //     let player = Race::create(name, &mut conn).unwrap();
    //     let existing_player = Race::by_id(&player.id, &conn).unwrap();
    //     assert_eq!(player.id, existing_player.id);
    // }
}

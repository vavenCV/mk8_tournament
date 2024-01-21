use std::collections::HashMap;

use crate::db::schema::race;
use crate::db::schema::race::dsl::race as race_dsl;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "race"]

pub struct Race {
    pub id: i32,
    pub faceoff_id: Option<i32>,
    pub race_point_ids: Option<String>,
}

impl Race {
    pub fn list(conn: &mut SqliteConnection) -> Vec<Self> {
        race_dsl
            .load::<Race>(conn)
            .expect("Error loading races")
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
        use crate::db::schema::race::dsl::faceoff_id;

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
        use crate::db::schema::race::dsl::race_point_ids;

        if let Ok(record) = race_dsl
            .filter(race_point_ids.contains(query_racepoint_id))
            .load::<Race>(conn)
        {
            Some(record)
        } else {
            None
        }
    }

    pub fn by_points(query_points: &i32, conn: &SqliteConnection) -> Option<Vec<Race>> {
        use crate::db::schema::race::dsl::points;

        if let Ok(record) = race_dsl
            .filter(points.eq(query_points))
            .load::<Race>(conn)
        {
            Some(record)
        } else {
            None
        }
    }

    pub fn by_player_and_race_id(
        query_player_id: &i32,
        query_race_id: &i32,
        conn: &SqliteConnection,
    ) -> Option<Self> {
        use crate::db::schema::race::dsl::player_id;
        use crate::db::schema::race::dsl::race_id;

        if let Ok(record) = race_dsl
            .filter(race_id.eq(query_race_id))
            .filter(player_id.eq(query_player_id))
            .first::<Race>(conn)
        {
            Some(record)
        } else {
            None
        }
    }

    pub fn create(
        player_id: Option<i32>,
        race_id: Option<i32>,
        points: Option<i32>,
        conn: &mut SqliteConnection,
    ) -> Option<Self> {
        let new_id = Uuid::new_v4().as_u128() as i32;

        if race_id.is_some() && player_id.is_some() {
            if let Some(player) =
                Self::by_player_and_race_id(&player_id.unwrap(), &race_id.unwrap(), conn)
            {
                return Some(player);
            }
        }
        let new_race = Self::new_player_struct(&new_id, race_id, player_id, points);

        diesel::insert_into(race_dsl)
            .values(&new_race)
            .execute(conn)
            .expect("Error saving new new_race");
        Self::by_id(&new_id, conn)
    }
    fn new_player_struct(
        id: &i32,
        race_id: Option<i32>,
        player_id: Option<i32>,
        points: Option<i32>,
    ) -> Self {
        Race {
            id: *id,
            race_id: race_id,
            player_id: player_id,
            points: points,
        }
    }
}
#[cfg(test)]
mod player_test {
    use crate::db::{
        establish_connection,
        model::{player::Player, race_point::{Race, self}},
    };
    #[test]
    fn create_race() {
        let mut conn = establish_connection().get().unwrap();

        let player_name = Some("[GRE] p1");
        let player = Player::create(player_name, &mut conn).unwrap();
        let race = Race::create(player_name, &mut conn).unwrap();

        let race = Race::create(player.id, race_id, points, conn)

        assert_eq!(player.name.unwrap().as_str(), name.unwrap());
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

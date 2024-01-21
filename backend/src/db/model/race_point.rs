use std::collections::HashMap;

use crate::db::schema::race_points;
use crate::db::schema::race_points::dsl::race_points as race_points_dsl;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "race_points"]

pub struct RacePoints {
    pub id: i32,
    pub race_id: Option<i32>,
    pub player_id: Option<i32>,
    pub points: Option<i32>,
}

impl RacePoints {
    pub fn list(conn: &mut SqliteConnection) -> Vec<Self> {
        race_points_dsl
            .load::<RacePoints>(conn)
            .expect("Error loading race_points")
    }
    pub fn by_id(id: &i32, conn: &SqliteConnection) -> Option<Self> {
        if let Ok(record) = race_points_dsl.find(id).get_result::<RacePoints>(conn) {
            Some(record)
        } else {
            None
        }
    }

    pub fn by_player_id(query_player_id: &i32, conn: &SqliteConnection) -> Option<Vec<RacePoints>> {
        use crate::db::schema::race_points::dsl::player_id;

        if let Ok(record) = race_points_dsl
            .filter(player_id.eq(query_player_id))
            .load::<RacePoints>(conn)
        {
            Some(record)
        } else {
            None
        }
    }

    pub fn by_race_id(query_race_id: &i32, conn: &SqliteConnection) -> Option<Vec<RacePoints>> {
        use crate::db::schema::race_points::dsl::race_id;

        if let Ok(record) = race_points_dsl
            .filter(race_id.eq(query_race_id))
            .load::<RacePoints>(conn)
        {
            Some(record)
        } else {
            None
        }
    }

    pub fn by_points(query_points: &i32, conn: &SqliteConnection) -> Option<Vec<RacePoints>> {
        use crate::db::schema::race_points::dsl::points;

        if let Ok(record) = race_points_dsl
            .filter(points.eq(query_points))
            .load::<RacePoints>(conn)
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
        use crate::db::schema::race_points::dsl::player_id;
        use crate::db::schema::race_points::dsl::race_id;

        if let Ok(record) = race_points_dsl
            .filter(race_id.eq(query_race_id))
            .filter(player_id.eq(query_player_id))
            .first::<RacePoints>(conn)
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
        let new_race_points = Self::new_player_struct(&new_id, race_id, player_id, points);

        diesel::insert_into(race_points_dsl)
            .values(&new_race_points)
            .execute(conn)
            .expect("Error saving new new_race_points");
        Self::by_id(&new_id, conn)
    }
    fn new_player_struct(
        id: &i32,
        race_id: Option<i32>,
        player_id: Option<i32>,
        points: Option<i32>,
    ) -> Self {
        RacePoints {
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
        model::{player::Player, race_point::{RacePoints, self}},
    };
    #[test]
    fn create_race_points() {
        let mut conn = establish_connection().get().unwrap();

        let player_name = Some("[GRE] p1");
        let player = Player::create(player_name, &mut conn).unwrap();
        let race = Race::create(player_name, &mut conn).unwrap();

        let race_points = RacePoints::create(player.id, race_id, points, conn)

        assert_eq!(player.name.unwrap().as_str(), name.unwrap());
    }
    // #[test]
    // fn create_player_with_existing_name() {
    //     let mut conn = establish_connection().get().unwrap();
    //     let name = Some("[GRE] p1");
    //     let player = RacePoints::create(name, &mut conn).unwrap();
    //     let existing_player = RacePoints::create(name, &mut conn).unwrap();
    //     assert_eq!(player.id, existing_player.id);
    // }
    // #[test]
    // fn list_players() {
    //     let mut conn = establish_connection().get().unwrap();
    //     let name = Some("[GRE] p1");
    //     let player = RacePoints::create(name, &mut conn).unwrap();
    //     let existing_players = RacePoints::list(&mut conn);
    //     assert_eq!(1, existing_players.len());
    //     assert_eq!(player.id, existing_players[0].id);
    // }
    // #[test]
    // fn get_player_by_name() {
    //     let mut conn = establish_connection().get().unwrap();
    //     let name = Some("[GRE] p1");
    //     let player = RacePoints::create(name, &mut conn).unwrap();
    //     let existing_player = RacePoints::by_name(&name.unwrap(), &conn).unwrap();
    //     assert_eq!(player.id, existing_player.id);
    // }
    // #[test]
    // fn get_player_by_id() {
    //     let mut conn = establish_connection().get().unwrap();
    //     let name = Some("[GRE] p1");
    //     let player = RacePoints::create(name, &mut conn).unwrap();
    //     let existing_player = RacePoints::by_id(&player.id, &conn).unwrap();
    //     assert_eq!(player.id, existing_player.id);
    // }
}

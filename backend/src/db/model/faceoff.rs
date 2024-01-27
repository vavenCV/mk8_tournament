use std::collections::HashMap;

use crate::db::schema::faceoffs::dsl::faceoffs as faceoff_dsl;
use crate::{db::schema::faceoffs, utils};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "faceoffs"]

pub struct Faceoff {
    pub id: i32,
    pub race_ids: String,
    pub player_ids: String,
}

impl Faceoff {
    pub fn list(conn: &mut SqliteConnection) -> Vec<Self> {
        faceoff_dsl
            .load::<Faceoff>(conn)
            .expect("Error loading faceoffs")
    }
    pub fn by_id(id: &i32, conn: &SqliteConnection) -> Option<Self> {
        if let Ok(record) = faceoff_dsl.find(id).get_result::<Faceoff>(conn) {
            Some(record)
        } else {
            None
        }
    }

    pub fn by_race_id(query_race_id: i32, conn: &mut SqliteConnection) -> Option<Self> {
        use crate::db::schema::faceoffs::dsl::race_ids;

        if let Ok(record) = faceoff_dsl
            .filter(race_ids.like(format!("%{}%", query_race_id)))
            .first::<Self>(conn)
        {
            Some(record)
        } else {
            None
        }
    }

    pub fn by_player_id(query_player_id: i32, conn: &mut SqliteConnection) -> Option<Vec<Self>> {
        use crate::db::schema::faceoffs::dsl::player_ids;

        if let Ok(record) = faceoff_dsl
            .filter(player_ids.like(format!("%{}%", query_player_id)))
            .load::<Self>(conn)
        {
            Some(record)
        } else {
            None
        }
    }

    pub fn by_players_and_races_ids(
        query_players_id: &Vec<i32>,
        query_races_id: &Vec<i32>,
        conn: &SqliteConnection,
    ) -> Option<Self> {
        use crate::db::schema::faceoffs::dsl::player_ids;
        use crate::db::schema::faceoffs::dsl::race_ids;

        if let Ok(record) = faceoff_dsl
            .filter(player_ids.eq(utils::ids::ids_to_string(Some(query_players_id.to_vec()))))
            .filter(race_ids.eq(utils::ids::ids_to_string(Some(query_races_id.to_vec()))))
            .first::<Self>(conn)
        {
            Some(record)
        } else {
            None
        }
    }

    pub fn create(
        race_ids: Option<Vec<i32>>,
        player_ids: Option<Vec<i32>>,
        conn: &mut SqliteConnection,
    ) -> Option<Self> {
        let new_id = Uuid::new_v4().as_u128() as i32;

        if race_ids.is_some() && player_ids.is_some() {
            if let Some(player) = Self::by_players_and_races_ids(
                &player_ids.clone().unwrap(),
                &race_ids.clone().unwrap(),
                conn,
            ) {
                return Some(player);
            }
        }
        let new_race = Self::new_faceoff_struct(&new_id, race_ids, player_ids);

        diesel::insert_into(faceoff_dsl)
            .values(&new_race)
            .execute(conn)
            .expect("Error saving new faceoff");
        Self::by_id(&new_id, conn)
    }

    fn new_faceoff_struct(
        id: &i32,
        race_ids: Option<Vec<i32>>,
        player_ids: Option<Vec<i32>>,
    ) -> Self {
        Faceoff {
            id: *id,
            race_ids: utils::ids::ids_to_string(race_ids),
            player_ids: utils::ids::ids_to_string(player_ids),
        }
    }

    pub fn set_race_ids(query_id: i32, new_race_ids: &[i32], conn: &mut SqliteConnection) {
        use crate::db::schema::faceoffs::dsl::id;
        use crate::db::schema::faceoffs::dsl::race_ids;

        let str_race_ids = utils::ids::ids_to_string(Some(new_race_ids.to_vec()));

        let updated_row = diesel::update(faceoff_dsl.filter(id.eq(query_id)))
            .set(race_ids.eq(str_race_ids))
            .execute(conn);
    }

    pub fn set_player_ids(query_id: i32, new_player_ids: &[i32], conn: &mut SqliteConnection) {
        use crate::db::schema::faceoffs::dsl::id;
        use crate::db::schema::faceoffs::dsl::player_ids;

        let str_player_ids = utils::ids::ids_to_string(Some(new_player_ids.to_vec()));

        let updated_row = diesel::update(faceoff_dsl.filter(id.eq(query_id)))
            .set(player_ids.eq(str_player_ids))
            .execute(conn);
    }
}
#[cfg(test)]
mod faceoff_test {
    use crate::{
        db::{
            establish_connection,
            model::{faceoff::Faceoff, player::Player, race::Race, race_point::RacePoints},
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

        Race::set_racepoint_ids(race.id, &[race_points.id], &mut conn);
        race = Race::by_id(&race.id, &conn).unwrap();

        let faceoff = Faceoff::create(
            Some([race.id].to_vec()),
            Some([player.id].to_vec()),
            &mut conn,
        )
        .unwrap();

        assert_eq!(race_points.points, Some(points));
        assert_eq!(
            utils::ids::string_to_ids(race.race_point_ids)
                .unwrap()
                .first()
                .unwrap(),
            &race_points.id
        );
        assert_eq!(
            faceoff.id,
            Faceoff::by_player_id(player.id, &mut conn)
                .unwrap()
                .first()
                .unwrap()
                .id
        );
    }
}

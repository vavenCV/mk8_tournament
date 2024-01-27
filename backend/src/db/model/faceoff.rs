use std::collections::HashMap;

use crate::db::schema::faceoffs::dsl::faceoffs as faceoff_dsl;
use crate::{db::schema::faceoffs, utils};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::race::Race;
#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "faceoffs"]

pub struct Faceoff {
    pub id: i32,
    pub race_ids: String,
    pub team_ids: String,
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

    pub fn by_team_id(query_team_id: i32, conn: &mut SqliteConnection) -> Option<Vec<Self>> {
        use crate::db::schema::faceoffs::dsl::team_ids;

        if let Ok(record) = faceoff_dsl
            .filter(team_ids.like(format!("%{}%", query_team_id)))
            .load::<Self>(conn)
        {
            Some(record)
        } else {
            None
        }
    }

    pub fn by_teams_and_races_ids(
        query_players_id: &Vec<i32>,
        query_races_id: &Vec<i32>,
        conn: &SqliteConnection,
    ) -> Option<Self> {
        use crate::db::schema::faceoffs::dsl::race_ids;
        use crate::db::schema::faceoffs::dsl::team_ids;

        if let Ok(record) = faceoff_dsl
            .filter(team_ids.eq(utils::ids::ids_to_string(Some(query_players_id.to_vec()))))
            .filter(race_ids.eq(utils::ids::ids_to_string(Some(query_races_id.to_vec()))))
            .first::<Self>(conn)
        {
            Some(record)
        } else {
            None
        }
    }

    pub fn create(
        race_number: i32,
        team_ids: Vec<i32>,
        conn: &mut SqliteConnection,
    ) -> Option<Self> {
        let new_id = Uuid::new_v4().as_u128() as i32;

        let mut race_ids: Vec<i32> = vec![];

        for _race in 0..race_number {
            race_ids.push(
                Race::create(team_ids.clone(), Some(new_id), None, conn)
                    .unwrap()
                    .id,
            );
        }

        let new_race = Self::new_faceoff_struct(&new_id, Some(race_ids), Some(team_ids));

        diesel::insert_into(faceoff_dsl)
            .values(&new_race)
            .execute(conn)
            .expect("Error saving new faceoff");
        Self::by_id(&new_id, conn)
    }

    fn new_faceoff_struct(
        id: &i32,
        race_ids: Option<Vec<i32>>,
        team_ids: Option<Vec<i32>>,
    ) -> Self {
        Faceoff {
            id: *id,
            race_ids: utils::ids::ids_to_string(race_ids),
            team_ids: utils::ids::ids_to_string(team_ids),
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

    pub fn set_team_ids(query_id: i32, new_team_ids: &[i32], conn: &mut SqliteConnection) {
        use crate::db::schema::faceoffs::dsl::id;
        use crate::db::schema::faceoffs::dsl::team_ids;

        let str_team_ids = utils::ids::ids_to_string(Some(new_team_ids.to_vec()));

        let updated_row = diesel::update(faceoff_dsl.filter(id.eq(query_id)))
            .set(team_ids.eq(str_team_ids))
            .execute(conn);
    }
}
#[cfg(test)]
mod faceoff_test {
    use crate::{
        db::{
            establish_connection,
            model::{
                faceoff::Faceoff, player::Player, race::Race, race_point::RacePoints, team::Team,
            },
            schema::teams,
        },
        utils,
    };
    #[test]
    fn create_faceoff() {
        let mut conn = establish_connection().get().unwrap();

        let teams = vec![
            Team::create(["P1", "P2", "P3", "P4"], &mut conn).unwrap(),
            Team::create(["P5", "P6", "P7", "P8"], &mut conn).unwrap(),
        ];
        let team_ids = teams.iter().map(|team| team.id).collect::<Vec<i32>>();

        let faceoff = Faceoff::create(6, team_ids, &mut conn).unwrap();

        for race_id in utils::ids::string_to_ids(faceoff.race_ids).unwrap() {
            println!("{:?}", Race::by_id(&race_id, &conn).unwrap());
        }
    }
}

use super::player::Player;
use crate::db::schema::teams;
use crate::db::schema::teams::dsl::teams as team_dsl;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "teams"]
pub struct Team {
    pub id: i32,
    pub player_ids: String,
}
impl Team {
    pub fn list(conn: &mut SqliteConnection) -> Vec<Self> {
        team_dsl.load::<Team>(conn).expect("Error loading users")
    }
    pub fn by_id(id: &i32, conn: &SqliteConnection) -> Option<Self> {
        if let Ok(record) = team_dsl.find(id).get_result::<Team>(conn) {
            Some(record)
        } else {
            None
        }
    }
    pub fn by_player_ids(name_str: &str, conn: &SqliteConnection) -> Option<Self> {
        use crate::db::schema::teams::dsl::player_ids;
        if let Ok(record) = team_dsl.filter(player_ids.eq(name_str)).first::<Team>(conn) {
            Some(record)
        } else {
            None
        }
    }

    pub fn create(player_names: [&str; 4], conn: &mut SqliteConnection) -> Option<Self> {
        let new_id = Uuid::new_v4().as_u128() as i32;

        let mut player_ids_vec: Vec<String> = vec![];

        for name in player_names {
            let players: Vec<Player> = player_names
                .iter()
                .map(|name| Player::create(name, Some(new_id), conn).unwrap())
                .collect();

            player_ids_vec.push(match Player::by_name(name, conn) {
                Some(player) => player.id.to_string(),
                None => return None,
            });
        }

        let player_ids = player_ids_vec.join(";");

        if let Some(player) = Self::by_player_ids(&player_ids, conn) {
            return Some(player);
        }

        let players: Vec<Player> = player_ids_vec
            .iter()
            .map(|id_str| Player::by_id(&id_str.parse::<i32>().unwrap(), conn).unwrap())
            .collect();
        let new_team = Self::new_player_struct(&new_id, players);

        diesel::insert_into(team_dsl)
            .values(&new_team)
            .execute(conn)
            .expect("Error saving new player");
        Self::by_id(&new_id, conn)
    }
    fn new_player_struct(id: &i32, players: Vec<Player>) -> Self {
        let player_ids = players
            .iter()
            .map(|player| player.id.to_string())
            .collect::<Vec<String>>()
            .join(";");

        Team {
            id: *id,
            player_ids: player_ids,
        }
    }
}
#[cfg(test)]
mod team_test {
    use std::{thread, time};

    use crate::db::{
        establish_connection,
        model::{player::Player, team::Team},
    };
    #[test]
    fn create_team_with_player_ids() {
        let mut conn = establish_connection().get().unwrap();
        let player_names = ["[GRE] p1", "[GRE] p2", "[GRE] p3", "[GRE] p4"];
        let team = Team::create(player_names, &mut conn).unwrap();
        let player_ids = player_names
            .iter()
            .map(|name| Player::by_name(name, &conn).unwrap().id.to_string())
            .collect::<Vec<String>>();
        let player_ids_str = player_ids.join(";");
        assert_eq!(team.player_ids.as_str(), player_ids_str);
    }
    // #[test]
    // fn create_player_with_existing_name() {
    //     let mut conn = establish_connection().get().unwrap();
    //     let name = Some("[GRE] p1");
    //     let team = Team::create(name, &mut conn).unwrap();
    //     let existing_team = Team::create(name, &mut conn).unwrap();
    //     assert_eq!(team.id, existing_team.id);
    // }
    // #[test]
    // fn list_teams() {
    //     let mut conn = establish_connection().get().unwrap();
    //     let player_ids = Some("[GRE] p1");
    //     let team = Team::create(player_ids, &mut conn).unwrap();
    //     let existing_teams = Team::list(&mut conn);
    //     assert_eq!(1, existing_teams.len());
    //     assert_eq!(team.id, existing_teams[0].id);
    // }
    // #[test]
    // fn get_team_by_player_ids() {
    //     let mut conn = establish_connection().get().unwrap();
    //     let player_ids = Some("[GRE] p1");
    //     let team = Team::create(player_ids, &mut conn).unwrap();
    //     let existing_team = Team::by_player_ids(&player_ids.unwrap(), &conn).unwrap();
    //     assert_eq!(team.id, existing_team.id);
    // }
    // #[test]
    // fn get_player_by_id() {
    //     let mut conn = establish_connection().get().unwrap();
    //     let player_ids = Some("[GRE] p1");
    //     let team = Team::create(player_ids, &mut conn).unwrap();
    //     let existing_team = Team::by_id(&team.id, &conn).unwrap();
    //     assert_eq!(team.id, existing_team.id);
    // }
}

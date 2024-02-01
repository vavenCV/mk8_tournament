use std::error::Error;

use super::player::Player;
use crate::db::schema::teams::dsl::teams as team_dsl;
use crate::{db::schema::teams, utils};
use diesel::prelude::*;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use uuid::Uuid;
#[derive(Debug, Deserialize, Queryable, Insertable)]
#[table_name = "teams"]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub player_ids: String,
}
impl Serialize for Team {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Team", 3)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field(
            "player_ids",
            &utils::ids::string_to_ids(self.player_ids.clone()).unwrap(),
        )?;
        state.end()
    }
}
impl Team {
    pub fn list(conn: &mut SqliteConnection) -> Vec<Self> {
        team_dsl.load::<Team>(conn).expect("Error loading teams")
    }
    pub fn by_id(id: &i32, conn: &SqliteConnection) -> Option<Self> {
        if let Ok(record) = team_dsl.find(id).get_result::<Team>(conn) {
            Some(record)
        } else {
            None
        }
    }

    pub fn by_team_name(name_str: &str, conn: &SqliteConnection) -> Option<Self> {
        use crate::db::schema::teams::dsl::name;

        if let Ok(record) = team_dsl.filter(name.eq(name_str)).first::<Team>(conn) {
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

    pub fn create(team_name: String, player_names: [&str; 4], conn: &mut SqliteConnection) -> Option<Self> {
        let new_id = utils::ids::get_random_unique_id(Self::by_id, conn);

        if let Some(player) = Self::by_team_name(&team_name, conn) {
            return Some(player);
        }

        let player_ids_vec: Vec<String> = player_names
            .iter()
            .map(|name| -> Result<String, Box<dyn Error>> {
                Ok(Player::create(name, new_id, conn)
                    .ok_or("Could not create player")?
                    .id
                    .to_string())
            })
            .collect::<Result<Vec<String>, Box<dyn Error>>>().ok()?;

        let player_ids = player_ids_vec.join(";");

        if let Some(player) = Self::by_player_ids(&player_ids, conn) {
            return Some(player);
        }

        let new_team = Self::new_team_struct(&new_id, team_name, player_ids);

        diesel::insert_into(team_dsl)
            .values(&new_team)
            .execute(conn)
            .expect("Error saving new team");
        Self::by_id(&new_id, conn)
    }

    fn new_team_struct(id: &i32, team_name: String, player_ids_str: String) -> Self {
        Team {
            id: *id,
            name: team_name,
            player_ids: player_ids_str,
        }
    }
}
#[cfg(test)]
mod team_test {

    use crate::db::{
        establish_connection,
        model::{player::Player, team::Team},
    };
    #[test]
    fn create_team_with_player_ids() {
        let mut conn = establish_connection().get().unwrap();
        let player_names = ["[GRE] p1", "[GRE] p2", "[GRE] p3", "[GRE] p4"];
        let team = Team::create("[GRE 1]".to_string(), player_names, &mut conn).unwrap();
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

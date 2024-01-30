use crate::db::schema::players;
use crate::db::schema::players::dsl::players as player_dsl;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "players"]
pub struct Player {
    pub id: i32,
    pub team_id: i32,
    pub name: String,
}
impl Player {
    pub fn list(conn: &mut SqliteConnection) -> Vec<Self> {
        player_dsl
            .load::<Player>(conn)
            .expect("Error loading players")
    }

    pub fn by_id(id: &i32, conn: &SqliteConnection) -> Option<Self> {
        if let Ok(record) = player_dsl.find(id).get_result::<Player>(conn) {
            Some(record)
        } else {
            None
        }
    }

    pub fn by_team_id(query_team_id: &i32, conn: &SqliteConnection) -> Option<Vec<Self>> {
        use crate::db::schema::players::dsl::team_id;

        if let Ok(record) = player_dsl
            .filter(team_id.eq(query_team_id))
            .load::<Self>(conn)
        {
            Some(record)
        } else {
            None
        }
    }

    pub fn by_name(name_str: &str, conn: &SqliteConnection) -> Option<Self> {
        use crate::db::schema::players::dsl::name;
        if let Ok(record) = player_dsl.filter(name.eq(name_str)).first::<Player>(conn) {
            Some(record)
        } else {
            None
        }
    }

    pub fn create(name: &str, team_id: i32, conn: &mut SqliteConnection) -> Option<Self> {
        let new_id = Uuid::new_v4().as_u128() as i32;

        if let Some(player) = Self::by_name(&name, conn) {
            return Some(player);
        }

        let new_player = Self::new_player_struct(&new_id, team_id, name);
        diesel::insert_into(player_dsl)
            .values(&new_player)
            .execute(conn)
            .expect("Error saving new player");
        Self::by_id(&new_id, conn)
    }

    fn new_player_struct(id: &i32, team_id: i32, name: &str) -> Self {
        Player {
            id: *id,
            team_id: team_id,
            name: name.to_string(),
        }
    }
}
#[cfg(test)]
mod player_test {
    use crate::db::{establish_connection, model::player::Player};
    #[test]
    fn create_player_with_name() {
        let mut conn = establish_connection().get().unwrap();
        let name = "[GRE] p1";
        let player = Player::create(name, 1, &mut conn).unwrap();
        assert_eq!(player.name.as_str(), name);
    }
    #[test]
    fn create_player_with_existing_name() {
        let mut conn = establish_connection().get().unwrap();
        let name = "[GRE] p1";
        let player = Player::create(name, 1, &mut conn).unwrap();
        let existing_player = Player::create(name, 1, &mut conn).unwrap();
        assert_eq!(player.id, existing_player.id);
    }
    #[test]
    fn list_players() {
        let mut conn = establish_connection().get().unwrap();
        let name = "[GRE] p1";
        let player = Player::create(name, 1, &mut conn).unwrap();
        let existing_players = Player::list(&mut conn);
        assert_eq!(1, existing_players.len());
        assert_eq!(player.id, existing_players[0].id);
    }
    #[test]
    fn get_player_by_name() {
        let mut conn = establish_connection().get().unwrap();
        let name = "[GRE] p1";
        let player = Player::create(name, 1, &mut conn).unwrap();
        let existing_player = Player::by_name(&name, &conn).unwrap();
        assert_eq!(player.id, existing_player.id);
    }
    #[test]
    fn get_player_by_id() {
        let mut conn = establish_connection().get().unwrap();
        let name = "[GRE] p1";
        let player = Player::create(name, 1, &mut conn).unwrap();
        let existing_player = Player::by_id(&player.id, &conn).unwrap();
        assert_eq!(player.id, existing_player.id);
    }
}

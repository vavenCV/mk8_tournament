use super::schema::players;
use super::schema::players::dsl::players as player_dsl;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "players"]
pub struct Player {
    pub id: i32,
    pub name: Option<String>,
}
impl Player {
    pub fn list(conn: &mut SqliteConnection) -> Vec<Self> {
        player_dsl
            .load::<Player>(conn)
            .expect("Error loading users")
    }
    pub fn by_id(id: &i32, conn: &SqliteConnection) -> Option<Self> {
        if let Ok(record) = player_dsl.find(id).get_result::<Player>(conn) {
            Some(record)
        } else {
            None
        }
    }
    pub fn by_name(name_str: &str, conn: &SqliteConnection) -> Option<Self> {
        use super::schema::players::dsl::name;
        if let Ok(record) = player_dsl.filter(name.eq(name_str)).first::<Player>(conn) {
            Some(record)
        } else {
            None
        }
    }

    pub fn create(name: Option<&str>, conn: &mut SqliteConnection) -> Option<Self> {
        let new_id = Uuid::new_v4().as_u128() as i32;

        if name.is_none() {
            return None;
        }

        if name.is_some() {
            if let Some(player) = Self::by_name(&name.unwrap(), conn) {
                return Some(player);
            }
        }
        let new_player = Self::new_player_struct(&new_id, name);
        diesel::insert_into(player_dsl)
            .values(&new_player)
            .execute(conn)
            .expect("Error saving new player");
        Self::by_id(&new_id, conn)
    }
    fn new_player_struct(id: &i32, name: Option<&str>) -> Self {
        Player {
            id: *id,
            name: name.map(Into::into),
        }
    }
}
#[cfg(test)]
mod player_test {
    use crate::db::{establish_connection, model::Player};
    #[test]
    fn create_player_with_name() {
        let mut conn = establish_connection().get().unwrap();
        let name = Some("[GRE] p1");
        let player = Player::create(name, &mut conn).unwrap();
        assert_eq!(player.name.unwrap().as_str(), name.unwrap());
    }
    #[test]
    fn create_player_with_existing_name() {
        let mut conn = establish_connection().get().unwrap();
        let name = Some("[GRE] p1");
        let player = Player::create(name, &mut conn).unwrap();
        let existing_player = Player::create(name, &mut conn).unwrap();
        assert_eq!(player.id, existing_player.id);
    }
    #[test]
    fn list_players() {
        let mut conn = establish_connection().get().unwrap();
        let name = Some("[GRE] p1");
        let player = Player::create(name, &mut conn).unwrap();
        let existing_players = Player::list(&mut conn);
        assert_eq!(1, existing_players.len());
        assert_eq!(player.id, existing_players[0].id);
    }
    #[test]
    fn get_player_by_name() {
        let mut conn = establish_connection().get().unwrap();
        let name = Some("[GRE] p1");
        let player = Player::create(name, &mut conn).unwrap();
        let existing_player = Player::by_name(&name.unwrap(), &conn).unwrap();
        assert_eq!(player.id, existing_player.id);
    }
    #[test]
    fn get_player_by_id() {
        let mut conn = establish_connection().get().unwrap();
        let name = Some("[GRE] p1");
        let player = Player::create(name, &mut conn).unwrap();
        let existing_player = Player::by_id(&player.id, &conn).unwrap();
        assert_eq!(player.id, player.id);
    }
}

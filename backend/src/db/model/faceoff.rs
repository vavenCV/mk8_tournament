use std::error::Error;

use crate::db::schema::faceoffs::dsl::faceoffs as faceoff_dsl;
use crate::{db::schema::faceoffs, utils};
use diesel::prelude::*;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};

use super::race::Race;
use super::team::Team;
#[derive(Debug, Deserialize, Queryable, Insertable)]
#[table_name = "faceoffs"]

pub struct Faceoff {
    pub id: i32,
    pub race_number: i32,
    pub race_ids: Option<String>,
    pub team_ids: Option<String>,
}
impl Serialize for Faceoff {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Faceoff", 3)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("race_ids", &utils::ids::string_to_ids(self.race_ids.clone().unwrap()).unwrap())?;
        state.serialize_field("team_ids", &utils::ids::string_to_ids(self.team_ids.clone().unwrap()).unwrap())?;
        state.end()
    }
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
        let new_id = utils::ids::get_random_unique_id(Self::by_id, conn);

        let new_race = Self::new_faceoff_struct(&new_id, &race_number,  None, Some(team_ids));

        diesel::insert_into(faceoff_dsl)
            .values(&new_race)
            .execute(conn)
            .expect("Error saving new faceoff");
        Self::by_id(&new_id, conn)
    }

    fn new_faceoff_struct(
        id: &i32,
        race_number: &i32,
        race_ids: Option<Vec<i32>>,
        team_ids: Option<Vec<i32>>,
    ) -> Self {
        Faceoff {
            id: *id,
            race_number: *race_number,
            race_ids: Some(utils::ids::ids_to_string(race_ids)),
            team_ids: Some(utils::ids::ids_to_string(team_ids)),
        }
    }

    pub fn set_race_ids(query_id: &i32, new_race_ids: &[i32], conn: &mut SqliteConnection) -> Result<Self, Box<dyn Error>> {
        use crate::db::schema::faceoffs::dsl::id;
        use crate::db::schema::faceoffs::dsl::race_ids;

        let str_race_ids = utils::ids::ids_to_string(Some(new_race_ids.to_vec()));

        let _updated_row = diesel::update(faceoff_dsl.filter(id.eq(query_id)))
            .set(race_ids.eq(str_race_ids))
            .execute(conn)?;

        Ok(Self::by_id(query_id, conn).ok_or("Unknown id")?)
    }

    pub fn set_team_ids(query_id: &i32, new_team_ids: &[i32], conn: &mut SqliteConnection) -> Result<Self, Box<dyn Error>> {
        use crate::db::schema::faceoffs::dsl::id;
        use crate::db::schema::faceoffs::dsl::team_ids;

        let str_team_ids = utils::ids::ids_to_string(Some(new_team_ids.to_vec()));

        let _updated_row = diesel::update(faceoff_dsl.filter(id.eq(query_id)))
            .set(team_ids.eq(str_team_ids))
            .execute(conn)?;

        Ok(Self::by_id(query_id, conn).ok_or("Unknown id")?)
    }

    pub fn generate_races(query_id: &i32, conn: &mut SqliteConnection) -> Result<Self, Box<dyn Error>> {
        let faceoff = Self::by_id(&query_id, conn).ok_or("provided faceoff id does not exist")?;

        let team_ids = utils::ids::string_to_ids(faceoff.team_ids.ok_or("no team has been set for faceoff")?)?;

        // Check for teams validity
        for team_id in team_ids.clone() {
            let _ = Team::by_id(&team_id, conn).ok_or("Unknown team_id")?;
        }

        let mut race_ids: Vec<i32> = vec![];

        for _race in 0..faceoff.race_number {
            race_ids.push(
                Race::create(team_ids.clone(), Some(faceoff.id), None, conn)
                    .unwrap()
                    .id,
            );
        }

        Self::set_race_ids(query_id, &race_ids, conn)
    }
}
#[cfg(test)]
mod faceoff_test {
    use crate::{
        db::{
            establish_connection,
            model::{
                faceoff::Faceoff, race::Race, team::Team,
            },
        },
        utils,
    };
    #[test]
    fn create_faceoff() {
        let mut conn = establish_connection().get().unwrap();

        let teams = vec![
            Team::create("[GRE 1]".to_string(), ["P1", "P2", "P3", "P4"], &mut conn).unwrap(),
            Team::create("[GRE 1]".to_string(), ["P5", "P6", "P7", "P8"], &mut conn).unwrap(),
        ];
        let team_ids = teams.iter().map(|team| team.id).collect::<Vec<i32>>();

        let faceoff = Faceoff::create(6, team_ids, &mut conn).unwrap();

        println!("{:?}", teams);
        for race_id in utils::ids::string_to_ids(faceoff.race_ids.unwrap()).unwrap() {
            println!("{:?}", Race::by_id(&race_id, &conn).unwrap());
        }
    }
}

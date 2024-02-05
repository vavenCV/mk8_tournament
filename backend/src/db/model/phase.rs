use crate::db::schema::phases::dsl::phases as phase_dsl;
use crate::utils;
use diesel::prelude::*;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};

use super::faceoff::Faceoff;

#[derive(Debug, Deserialize, Serialize)]
pub struct PhaseResp {
    pub id: i32,
    pub phase_number: i32,
    pub faceoff_ids: Vec<i32>,
}

#[derive(Debug, Deserialize, Queryable, Insertable)]
#[table_name = "phases"]
pub struct Phase {
    pub id: i32,
    pub phase_number: i32,
    pub faceoff_ids: Option<String>,
}
impl Serialize for Phase {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Phase", 3)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("phase_number", &self.phase_number)?;
        state.serialize_field(
            "faceoff_ids",
            &utils::ids::string_to_ids(self.faceoff_ids.clone().unwrap()).unwrap(),
        )?;

        state.end()
    }
}
impl Phase {
    pub fn list(conn: &mut SqliteConnection) -> Vec<Self> {
        phase_dsl.load::<Phase>(conn).expect("Error loading phases")
    }
    pub fn by_id(id: &i32, conn: &SqliteConnection) -> Option<Self> {
        if let Ok(record) = phase_dsl.find(id).get_result::<Phase>(conn) {
            Some(record)
        } else {
            None
        }
    }

    pub fn create(
        phase_number: i32,
        faceoff_number: i32,
        conn: &mut SqliteConnection,
    ) -> Option<Self> {
        let new_id = utils::ids::get_random_unique_id(Self::by_id, conn);

        let mut faceoff_ids: Vec<i32> = vec![];

        for _race in 0..faceoff_number {
            faceoff_ids.push(Faceoff::create(6, new_id, vec![], conn).unwrap().id);
        }

        let new_phase = Self::new_phase_struct(&new_id, &phase_number, Some(faceoff_ids));

        diesel::insert_into(phase_dsl)
            .values(&new_phase)
            .execute(conn)
            .expect("Error saving new phase");
        Self::by_id(&new_id, conn)
    }

    fn new_phase_struct(id: &i32, phase_number: &i32, faceoff_ids: Option<Vec<i32>>) -> Self {
        Phase {
            id: *id,
            phase_number: *phase_number,
            faceoff_ids: Some(utils::ids::ids_to_string(faceoff_ids)),
        }
    }
}
// #[cfg(test)]
// mod phase_test {
//     use crate::{
//         db::{
//             establish_connection,
//             model::{phase::Phase, race::Race, team::Team},
//         },
//         utils,
//     };
//     #[test]
//     fn create_phase() {
//         let mut conn = establish_connection().get().unwrap();

//         let teams = vec![
//             Team::create("[GRE 1]".to_string(), ["P1", "P2", "P3", "P4"], &mut conn).unwrap(),
//             Team::create("[GRE 1]".to_string(), ["P5", "P6", "P7", "P8"], &mut conn).unwrap(),
//         ];
//         let team_ids = teams.iter().map(|team| team.id).collect::<Vec<i32>>();

//         let phase = Phase::create(6, team_ids, &mut conn).unwrap();

//         println!("{:?}", teams);
//         for race_id in utils::ids::string_to_ids(phase.race_ids.unwrap()).unwrap() {
//             println!("{:?}", Race::by_id(&race_id, &conn).unwrap());
//         }
//     }
// }

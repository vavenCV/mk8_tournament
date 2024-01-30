use diesel::SqliteConnection;
use uuid::Uuid;

pub fn get_random_unique_id<T>(by_id: fn(&i32, &SqliteConnection) -> Option<T>, conn: &SqliteConnection ) -> i32 {
    let mut new_id: i32 = 0;
    loop  {
        new_id = Uuid::new_v4().as_u128() as i32;

        if by_id(&new_id, conn).is_none() {
            return new_id;
        }
    }
}

pub fn ids_to_string(ids: Option<Vec<i32>>) -> String {
    match ids {
        Some(ids) => ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(";"),
        None => "".to_string(),
    }
}

pub fn string_to_ids(str: String) -> Result<Vec<i32>, std::num::ParseIntError> {
    match str.as_str() {
        "" => Ok(vec![]),
        _ => str.split(";")
        .into_iter()
        .map(|i_str| i_str.parse::<i32>())
        .collect::<Vec<Result<i32, std::num::ParseIntError>>>()
        .into_iter()
        .collect()
    }
    
}

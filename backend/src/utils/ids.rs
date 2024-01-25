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
    str.split(";")
        .into_iter()
        .map(|i_str| i_str.parse::<i32>())
        .collect::<Vec<Result<i32, std::num::ParseIntError>>>()
        .into_iter()
        .collect()
}

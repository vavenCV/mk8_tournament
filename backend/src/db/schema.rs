// @generated automatically by Diesel CLI.

diesel::table! {
    players (id) {
        id -> Integer,
        name -> Nullable<Text>,
    }
}

diesel::table! {
    teams (id) {
        id -> Integer,
        element_ids -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(players, teams,);

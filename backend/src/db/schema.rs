// @generated automatically by Diesel CLI.

diesel::table! {
    faceoffs (id) {
        id -> Integer,
        race_ids -> Text,
    }
}

diesel::table! {
    players (id) {
        id -> Integer,
        team_id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    race_points (id) {
        id -> Integer,
        race_id -> Nullable<Integer>,
        player_id -> Nullable<Integer>,
        points -> Nullable<Integer>,
    }
}

diesel::table! {
    races (id) {
        id -> Integer,
        faceoff_id -> Nullable<Integer>,
        race_point_ids -> Text,
    }
}

diesel::table! {
    teams (id) {
        id -> Integer,
        player_ids -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    faceoffs,
    players,
    race_points,
    races,
    teams,
);

// @generated automatically by Diesel CLI.

diesel::table! {
    faceoffs (id) {
        id -> Integer,
        race_number -> Integer,
        race_ids -> Nullable<Text>,
        team_ids -> Nullable<Text>,
    }
}

diesel::table! {
    phase (id) {
        id -> Integer,
        phase_number -> Integer,
        faceoff_ids -> Nullable<Text>,
    }
}

diesel::table! {
    phases (id) {
        id -> Integer,
        phase_number -> Integer,
        faceoff_ids -> Nullable<Text>,
    }
}

diesel::table! {
    players (id) {
        id -> Integer,
        team_id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    race_points (id) {
        id -> Integer,
        race_id -> Integer,
        player_id -> Integer,
        points -> Integer,
    }
}

diesel::table! {
    races (id) {
        id -> Integer,
        team_ids -> Nullable<Text>,
        faceoff_id -> Nullable<Integer>,
        race_point_ids -> Nullable<Text>,
    }
}

diesel::table! {
    teams (id) {
        id -> Integer,
        name -> Text,
        player_ids -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    faceoffs,
    phase,
    phases,
    players,
    race_points,
    races,
    teams,
);

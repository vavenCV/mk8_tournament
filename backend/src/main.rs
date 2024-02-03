use actix_web::dev::Server;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

extern crate r2d2_diesel;
extern crate serde_json;
// ...
mod db;
mod services;
mod utils;

fn main() -> std::io::Result<()> {
    use actix_cors::Cors;
    use actix_web::{web::JsonConfig, App, HttpServer};
    let conn_pool = db::establish_connection();

    let mut sys = actix_rt::System::new("server");

    let srv: Server = HttpServer::new(move || {
        App::new()
            .data(conn_pool.clone())
            .data(JsonConfig::default().limit(4096))
            .wrap(Cors::permissive())
            .configure(services::players::init_routes)
            .configure(services::teams::init_routes)
            .configure(services::faceoffs::init_routes)
            .configure(services::races::init_routes)
    })
    .bind("0.0.0.0:5000")?
    .run();

    println!("🍄🍄 Mario Kart 🍄🍄 Server started on localhost:5000");

    sys.block_on(srv)
}

#[cfg(test)]
mod main_tests {
    use std::error::Error;
    use std::thread;
    use std::time::Duration;

    use crate::db::model::faceoff::FaceoffResp;
    use crate::db::model::race::RaceResp;
    use crate::db::model::team::TeamResp;
    use crate::main;
    use crate::services::faceoffs::FaceoffForm;
    use crate::services::players::{PlayerForm, PlayerPointsResp};
    use crate::services::races::{RaceForm, RacePointForm, RaceStatus};
    use crate::services::teams::TeamForm;
    use reqwest::blocking::Client;

    const SERVER_URL: &str = "http://localhost:5000";

    const TEAM_NUMBER: usize = 3;
    const TEAM_NAME: [&str; TEAM_NUMBER] = ["[GRE 4]", "[LYS 4]", "[SOF 4]"];
    const PLAYER_NAMES: [[&str; 4]; TEAM_NUMBER] = [
        ["[GRE 4] p1", "[GRE 4] p2", "[GRE 4] p3", "[GRE 4] p4"],
        ["[LYS 4] p1", "[LYS 4] p2", "[LYS 4] p3", "[LYS 4] p4"],
        ["[SOF 4] p1", "[SOF 4] p2", "[SOF 4] p3", "[SOF 4] p4"],
    ];

    const RACE_NUMBER: i32 = 6;
    const POINTS_BY_ORDER: [u8; 12] = [15, 12, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1];

    /// CREATE
    fn create_teams(client: &Client) -> Result<Vec<TeamResp>, Box<dyn Error>> {
        let mut teams: Vec<TeamResp> = vec![];

        for index in 0..TEAM_NUMBER {
            let (team_name, player_names) = (TEAM_NAME[index], PLAYER_NAMES[index]);
            let resp = client
                .post(format!("{SERVER_URL}/teams"))
                .json(&TeamForm {
                    team_name: team_name.to_string(),
                    player_names: player_names.map(|s| s.to_owned()).into(),
                })
                .send()
                .unwrap();
            let team = resp.json::<TeamResp>()?;
            teams.push(team);
        }
        Ok(teams)
    }

    fn create_faceoff(client: &Client, team_ids: Vec<i32>) -> Result<FaceoffResp, reqwest::Error> {
        client
            .post(format!("{SERVER_URL}/faceoffs"))
            .json(&FaceoffForm {
                race_number: RACE_NUMBER,
                team_ids: team_ids,
            })
            .send()?
            .json::<FaceoffResp>()
    }

    fn generate_faceoff_races(client: &Client, faceoff_id: i32) -> Result<(), Box<dyn Error>> {
        client
            .post(format!("{SERVER_URL}/faceoffs/{faceoff_id}/generate"))
            .send()?;
        Ok(())
    }

    fn set_points_for_race(
        client: &Client,
        race_id: &i32,
        player_point: &RaceForm,
    ) -> Result<(), Box<dyn Error>> {
        client
            .put(format!("{SERVER_URL}/races/{}", *race_id))
            .json(&player_point)
            .send()?;
        Ok(())
    }

    /// GET
    fn get_player_point(player_ids: &Vec<i32>) -> RaceForm {
        let mut race_form = RaceForm::default();

        for (index, player_id) in player_ids.iter().enumerate() {
            race_form.race_points.push(RacePointForm {
                player_id: *player_id,
                points: POINTS_BY_ORDER[index],
            });
        }
        race_form
    }

    fn get_race(client: &Client, race_id: i32) -> Result<RaceResp, reqwest::Error> {
        client
            .get(format!("{SERVER_URL}/races/{race_id}"))
            .send()?
            .json::<RaceResp>()
    }

    fn get_race_status(client: &Client, race_id: i32) -> Result<RaceStatus, reqwest::Error> {
        client
            .get(format!("{SERVER_URL}/races/{race_id}/status"))
            .send()?
            .json::<RaceStatus>()
    }

    fn get_team(client: &Client, team_id: i32) -> Result<TeamResp, reqwest::Error> {
        client
            .get(format!("{SERVER_URL}/teams/{team_id}"))
            .send()?
            .json::<TeamResp>()
    }

    fn get_faceoff(client: &Client, faceoff_id: i32) -> Result<FaceoffResp, reqwest::Error> {
        client
            .get(format!("{SERVER_URL}/faceoffs/{faceoff_id}"))
            .send()?
            .json::<FaceoffResp>()
    }

    // Create a faceoff of 6 race between 3 teams, then add points for the first 2 races
    fn create_faceoff_with_races_and_points(client: &Client, team_ids: Vec<i32>) -> i32 {
        let faceoff = create_faceoff(&client, team_ids).unwrap();
        generate_faceoff_races(&client, faceoff.id).unwrap();
        let faceoff = get_faceoff(&client, faceoff.id).unwrap();

        // > ASSERT Faceoff is created and has `RACE_NUMBER` races
        assert_eq!(faceoff.id, faceoff.id);
        assert_eq!(faceoff.race_ids.is_empty(), false);
        assert_eq!(faceoff.race_ids.len(), RACE_NUMBER as usize);
        // < End of ASSERTS

        let race_id_to_test = faceoff.race_ids.first().unwrap();
        let race_id_second_to_test = faceoff.race_ids.get(2).unwrap();

        let race = get_race(&client, *race_id_to_test).unwrap();

        let player_ids = race
            .team_ids
            .iter()
            .map(|tid| get_team(&client, *tid).unwrap().player_ids)
            .collect::<Vec<Vec<i32>>>()
            .concat();

        // #-# Create one Vec of 11 and one vec of 12
        let full_player_points = get_player_point(&player_ids);
        let mut truncated_player_point = full_player_points.clone();
        truncated_player_point
            .race_points
            .truncate(player_ids.len() - 1);

        let race_status = get_race_status(&client, *race_id_to_test).unwrap();
        assert_eq!(race_status.is_ended, false);

        // #-# FIRST, add a portion of all players points (11/12)
        set_points_for_race(&client, race_id_to_test, &truncated_player_point).unwrap();
        let race_status = get_race_status(&client, *race_id_to_test).unwrap();
        assert_eq!(race_status.is_ended, false);

        // #-# Add all players points (12/12)
        set_points_for_race(&client, race_id_to_test, &full_player_points).unwrap();
        let race_status = get_race_status(&client, *race_id_to_test).unwrap();
        assert_eq!(race_status.is_ended, true);

        set_points_for_race(&client, race_id_second_to_test, &full_player_points).unwrap();

        faceoff.id
    }

    #[test]
    fn create_env() {
        let _main_thread = thread::spawn(|| main());

        thread::sleep(Duration::from_secs(1));

        let client = Client::new();

        // ## GENERATE 3 TEAMS and 2 faceoff with those 3 teams

        let teams = create_teams(&client).unwrap();

        let player_ids = teams
            .iter()
            .map(|t| t.player_ids.clone())
            .collect::<Vec<Vec<i32>>>()
            .concat();

        let _first_faceoff_id =
            create_faceoff_with_races_and_points(&client, teams.iter().map(|t| t.id).collect());
        let second_faceoff_id =
            create_faceoff_with_races_and_points(&client, teams.iter().map(|t| t.id).collect());

        let player_id_to_test = teams.first().unwrap().player_ids.first().unwrap();

        let points_by_players_tournament = POINTS_BY_ORDER
            .iter()
            .map(|p| *p as u32 * 4)
            .collect::<Vec<u32>>();
        let points_by_players_in_faceoff = POINTS_BY_ORDER
            .iter()
            .map(|p| *p as u32 * 2)
            .collect::<Vec<u32>>();

        for index in 0..player_ids.len() {
            let pid = player_ids.get(index).unwrap();
            let points_in_tournament = points_by_players_tournament.get(index).unwrap();
            let points_in_faceoff = points_by_players_in_faceoff.get(index).unwrap();

            let total_points = client
                .get(format!("{SERVER_URL}/players/{pid}/total_points"))
                .send()
                .unwrap()
                .json::<PlayerPointsResp>()
                .unwrap();

            let total_points_in_second_faceoff = client
                .get(format!(
                    "{SERVER_URL}/players/{pid}/total_points_in_faceoff/{second_faceoff_id}"
                ))
                .send()
                .unwrap()
                .json::<PlayerPointsResp>()
                .unwrap();

            assert_eq!(&total_points.total_points, points_in_tournament);
            assert_eq!(
                &total_points_in_second_faceoff.total_points,
                points_in_faceoff
            );
        }
    }
}

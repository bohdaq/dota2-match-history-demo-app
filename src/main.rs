// How to use: 1. First step is to import crate functions.
use steam_webapi_rust_sdk::{get_dota2_match_history};
use steam_webapi_rust_sdk::idota2match_570::get_match_history::{GAME_MODE, PLAYER_SKILL};

fn main() {
    println!("dota2-match-history-demo-app");

    // How to use: 2. Getting Dota2 Match History.
    let boxed_match_history = get_dota2_match_history(
        Some(76561197960361544),
        Some(GAME_MODE.all_pick),
        Some(PLAYER_SKILL.normal),
        None,
        None,
        Some(100), // steam allows to get only 100 matches at max per request
        None
    );

    if boxed_match_history.is_ok() {
        let match_history = boxed_match_history.unwrap();
        let number_of_matches = match_history.matches.len();
        println!("got {} matches", number_of_matches);
        println!("{:?}", match_history); //debug output for the model


        // Getting even more results based on previous query
        let match_id = match_history.matches.last().unwrap();
        let boxed_match_history = get_dota2_match_history(
            Some(76561197960361544),
            Some(GAME_MODE.all_pick),
            Some(PLAYER_SKILL.normal),
            None,
            Some(match_id.match_id),
            Some(100),
            None
        );

        if boxed_match_history.is_ok() {
            let match_history = boxed_match_history.unwrap();
            let number_of_matches = match_history.matches.len();
            println!("got {} matches", number_of_matches);
            println!("{:?}", match_history); // debug output for the model

        }
    }
}

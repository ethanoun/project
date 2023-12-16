use csv::ReaderBuilder;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GameData {
    // Fields related to scoring
    blue_first_blood: i32,
    red_first_blood: i32,
    blue_first_baron: i32,
    red_first_baron: i32,
    blue_first_inhibitor: i32,
    red_first_inhibitor: i32,
    blue_dragon_kills: i32,
    red_dragon_kills: i32,
    blue_baron_kills: i32,
    red_baron_kills: i32,
    blue_tower_kills: i32,
    red_tower_kills: i32,
    blue_inhibitor_kills: i32,
    red_inhibitor_kills: i32,
    blue_ward_placed: i32,
    red_ward_placed: i32,
    blue_ward_kills: i32,
    red_ward_kills: i32,
    blue_kills: i32,
    red_kills: i32,
    blue_assist: i32,
    red_assist: i32,
    blue_total_gold: i32,
    red_total_gold: i32,
    blue_total_minion_kills: i32,
    red_total_minion_kills: i32,
    blue_total_level: i32,
    red_total_level: i32,
    blue_avg_level: i32,
    red_avg_level: i32,
    blue_jungle_minion_kills: i32,
    red_jungle_minion_kills: i32,
    blue_killing_spree: i32,
    red_killing_spree: i32,
    blue_object_damage_dealt: i32,
    red_object_damage_dealt: i32,
    

    // Fields related to win/loss
    blue_wins: i32,
    red_wins: i32,
}

fn calculate_scores(game: &GameData) -> (f64, f64) {
    // How each score is calculated
    let first_dragon_score = compare_and_score(game.blue_first_dragon, game.red_first_dragon);
    let first_baron_score = compare_and_score(game.blue_first_baron, game.red_first_baron);
    let first_inhibitor_score = compare_and_score(game.blue_first_inhibitor, game.red_first_inhibitor);
    let minion_score = compare_and_score(game.blue_total_minion_kills, game.red_total_minion_kills)
    let jungle_minion_score = compare_and_score(game.blue_jungle_minion_kills, game.red_jungle_minion_kills)
    let kills_score = compare_and_score(game.blue_kills, game.red_kills)
    let killing_spree_score = compare_and_score(game.blue_killing_spree, game.red_killing_spree)
    let wards_killed_score = compare_and_score(game.blue_ward_kills, game.red_ward_kills)
    let wards_placed_score = compare_and_score(game.blue_ward_placed, game.red_ward_placed)
    let tower_kills_score = compare_and_score(game.blue_tower_kills, game.red_tower_kills)
    let first_blood_score = compare_and_score(game.blue_first_blood, game.red_first_blood)
    let dragon_score = compare_and_score(game.blue_dragon_kills, game.red_dragon_kills)
    let baron_score = compare_and_score(game.blue_baron_kills, game.red_baron_kills)
    let inhibitor_score = compare_and_score(game.blue_inhibitor_kills, game.red_inhibitor_kills)
    let assists_score = compare_and_score(game.blue_assist, game.red_assist)
    let gold_score = compare_and_score(game.blue_total_gold, game.red_total_gold)
    let total_level_score = compare_and_score(game.blue_total_level, game.red_total_level)
    let avg_level_score = compare_and_score(game.blue_avg_level, game.red_avg_level)
    let objective_dmg_score = compare_and_score(game.blue_object_damage_dealt, game.red_object_damage_dealt)

    // Calculate total scores for blue and red teams
    let blue_score = dragon_score + baron_score + inhibitor_score + first_baron_score + first_dragon_score + first_inhibitor_score + minion_score + jungle_minion_score + kills_score + killing_spree_score + tower_kills_score + wards_killed_score + wards_placed_score + first_blood_score + assists_score + gold_score + total_level_score + avg_level_score + objective_dmg_score
    let red_score = blue_score;

    (blue_score, red_score)
}

// Function that helps compare the score
fn compare_and_score(blue_value: i32, red_value: i32) -> f64 {
    if blue_value > red_value {
        1.0 // Assign 1 point to blue team
    } else if blue_value < red_value {
        -1.0 // Assign -1 point to red team
    } else {
        0.0 // No points if values are equal
    }
}

fn main() {
    // Read and parse the dataset
    let mut rdr = ReaderBuilder::new().from_path("data/Challenger_Ranked_Games.csv").unwrap();

    // Initialize variables to track total scores
    let mut total_blue_score = 0.0;
    let mut total_red_score = 0.0;
    let mut total_games = 0;

    for result in rdr.deserialize() {
        let game: GameData = result.unwrap();

        // Calculate scores for the current game
        let (blue_score, red_score) = calculate_scores(&game);

        // Update total scores
        total_blue_score += blue_score;
        total_red_score += red_score;

        // Increment total games
        total_games += 1;
    }

    // Calculate and print average scores
    let average_blue_score = total_blue_score / total_games as f64;
    let average_red_score = total_red_score / total_games as f64;

    println!("Average Blue Score: {:.2}", average_blue_score);
    println!("Average Red Score: {:.2}", average_red_score);
}


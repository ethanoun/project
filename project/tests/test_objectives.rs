use csv::ReaderBuilder;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GameData {
    // Fields related to objectives

    blue_first_baron: i32,
    red_first_baron: i32,
    blue_first_inhibitor: i32,
    red_first_inhibitor: i32,
    blue_first_dragon: i32,
    red_first_dragon: i32,
    blue_dragon_kills: i32,
    red_dragon_kills: i32,
    blue_baron_kills: i32,
    red_baron_kills: i32,
    blue_tower_kills: i32,
    red_tower_kills: i32,
    blue_inhibitor_kills: i32,
    red_inhibitor_kills: i32,
    blue_wins: i32,
    red_wins: i32,
}

fn calculate_objective_win_rates(game: &GameData) {
    // Calculate wins to objectives for winrate
    let blue_win_rate_first_baron = game.blue_first_baron as f64 / game.blue_wins as f64;
    let blue_win_rate_first_inhibitor = game.blue_first_inhibitor as f64 / game.blue_wins as f64;
    let blue_win_rate_first_dragon = game.blue_first_dragon as f64 / game.blue_wins as f64;
    let blue_win_rate_dragon_kills = game.blue_dragon_kills as f64 / game.blue_wins as f64;
    let blue_win_rate_baron_kills = game.blue_baron_kills as f64 / game.blue_wins as f64;
    let blue_win_rate_tower_kills = game.blue_tower_kills as f64 / game.blue_wins as f64;
    let blue_win_rate_inhibitor_kills = game.blue_inhibitor_kills as f64 / game.blue_wins as f64;

    let red_win_rate_first_baron = game.red_first_baron as f64 / game.red_wins as f64;
    let red_win_rate_first_inhibitor = game.red_first_inhibitor as f64 / game.red_wins as f64;
    let red_win_rate_first_dragon = game.red_first_dragon as f64 / game.red_wins as f64;
    let red_win_rate_dragon_kills = game.red_dragon_kills as f64 / game.red_wins as f64;
    let red_win_rate_baron_kills = game.red_baron_kills as f64 / game.red_wins as f64;
    let red_win_rate_tower_kills = game.red_tower_kills as f64 / game.red_wins as f64;
    let red_win_rate_inhibitor_kills = game.red_inhibitor_kills as f64 / game.red_wins as f64;

    // Analyze the winrates for each objective
    println!("Blue Team Win Rates:");
    println!("First Baron: {:.2}%", blue_win_rate_first_baron * 100.0);
    println!("First Inhibitor: {:.2}%", blue_win_rate_first_inhibitor * 100.0);
    println!("First Dragon: {:.2}%", blue_win_rate_first_dragon * 100.0);
    println!("Dragon Kills: {:.2}%", blue_win_rate_dragon_kills * 100.0);
    println!("Baron Kills: {:.2}%", blue_win_rate_baron_kills * 100.0);
    println!("Tower Kills: {:.2}%", blue_win_rate_tower_kills * 100.0);
    println!("Inhibitor Kills: {:.2}%", blue_win_rate_inhibitor_kills * 100.0);

    println!("Red Team Win Rates:");
    println!("First Baron: {:.2}%", red_win_rate_first_baron * 100.0);
    println!("First Inhibitor: {:.2}%", red_win_rate_first_inhibitor * 100.0);
    println!("First Dragon: {:.2}%", red_win_rate_first_dragon * 100.0);
    println!("Dragon Kills: {:.2}%", red_win_rate_dragon_kills * 100.0);
    println!("Baron Kills: {:.2}%", red_win_rate_baron_kills * 100.0);
    println!("Tower Kills: {:.2}%", red_win_rate_tower_kills * 100.0);
    println!("Inhibitor Kills: {:.2}%", red_win_rate_inhibitor_kills * 100.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_objective_win_rates() {
        let mut rdr = ReaderBuilder::new().from_path("data/Challenger_Ranked_Games.csv").unwrap();

        for result in rdr.deserialize() {
            let game: GameData = result.unwrap();

            calculate_objective_win_rates(&game);
        }
    }
}

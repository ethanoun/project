use csv::ReaderBuilder;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GameData {
    // Existing fields with correlation to each name in dataset
    #[serde(rename = "blueWins")]
    blue_wins: i32,
    #[serde(rename = "redWins")]
    red_wins: i32,
    #[serde(rename = "blueFirstBlood")]
    blue_first_blood: i32,
    #[serde(rename = "redFirstBlood")]
    red_first_blood: i32,
    #[serde(rename = "blueFirstBaron")]
    blue_first_baron: i32,
    #[serde(rename = "redFirstBaron")]
    red_first_baron: i32,
    #[serde(rename = "blueFirstInhibitor")]
    blue_first_inhibitor: i32,
    #[serde(rename = "redFirstInhibitor")]
    red_first_inhibitor: i32,
    #[serde(rename = "blueDragonKills")]
    blue_dragon_kills: i32,
    #[serde(rename = "redDragonKills")]
    red_dragon_kills: i32,
    #[serde(rename = "blueBaronKills")]
    blue_baron_kills: i32,
    #[serde(rename = "redBaronKills")]
    red_baron_kills: i32,
    #[serde(rename = "blueTowerKills")]
    blue_tower_kills: i32,
    #[serde(rename = "redTowerKills")]
    red_tower_kills: i32,
    #[serde(rename = "blueInhibitorKills")]
    blue_inhibitor_kills: i32,
    #[serde(rename = "redInhibitorKills")]
    red_inhibitor_kills: i32,
    #[serde(rename = "blueWardPlaced")]
    blue_ward_placed: i32,
    #[serde(rename = "redWardPlaced")]
    red_ward_placed: i32,
    #[serde(rename = "blueWardKills")]
    blue_ward_kills: i32,
    #[serde(rename = "redWardKills")]
    red_ward_kills: i32,
    #[serde(rename = "blueKills")]
    blue_kills: i32,
    #[serde(rename = "redKills")]
    red_kills: i32,
    #[serde(rename = "blueDeath")]
    blue_death: i32,
    #[serde(rename = "redDeath")]
    red_death: i32,
    #[serde(rename = "blueAssist")]
    blue_assist: i32,
    #[serde(rename = "redAssist")]
    red_assist: i32,
    #[serde(rename = "blueChampionDamageDealt")]
    blue_champion_damage_dealt: i32,
    #[serde(rename = "redChampionDamageDealt")]
    red_champion_damage_dealt: i32,
    #[serde(rename = "blueTotalGold")]
    blue_total_gold: i32,
    #[serde(rename = "redTotalGold")]
    red_total_gold: i32,
    #[serde(rename = "blueTotalMinionKills")]
    blue_total_minion_kills: i32,
    #[serde(rename = "redTotalMinionKills")]
    red_total_minion_kills: i32,
    #[serde(rename = "blueTotalLevel")]
    blue_total_level: i32,
    #[serde(rename = "redTotalLevel")]
    red_total_level: i32,
    #[serde(rename = "blueAvgLevel")]
    blue_avg_level: i32,
    #[serde(rename = "redAvgLevel")]
    red_avg_level: i32,
    #[serde(rename = "blueJungleMinionKills")]
    blue_jungle_minion_kills: i32,
    #[serde(rename = "redJungleMinionKills")]
    red_jungle_minion_kills: i32,
    #[serde(rename = "blueKillingSpree")]
    blue_killing_spree: i32,
    #[serde(rename = "redKillingSpree")]
    red_killing_spree: i32,
    #[serde(rename = "blueTotalHeal")]
    blue_total_heal: i32,
    #[serde(rename = "redTotalHeal")]
    red_total_heal: i32,
    #[serde(rename = "blueObjectDamageDealt")]
    blue_object_damage_dealt: i32,
    #[serde(rename = "redObjectDamageDealt")]
    red_object_damage_dealt: i32,
}

fn calculate_score(game: &GameData) -> (i32, i32) {
    let blue_score = 
        game.blue_wins +
        game.blue_first_blood +
        game.blue_first_baron +
        game.blue_first_inhibitor +
        game.blue_dragon_kills +
        game.blue_baron_kills +
        game.blue_tower_kills +
        game.blue_inhibitor_kills +
        game.blue_ward_placed +
        game.blue_ward_kills +
        game.blue_kills +
        game.blue_death +
        game.blue_assist +
        game.blue_champion_damage_dealt +
        game.blue_total_gold +
        game.blue_total_minion_kills +
        game.blue_total_level +
        game.blue_avg_level +
        game.blue_jungle_minion_kills +
        game.blue_killing_spree +
        game.blue_total_heal +
        game.blue_object_damage_dealt;

    let red_score = 
        game.red_wins +
        game.red_first_blood +
        game.red_first_baron +
        game.red_first_inhibitor +
        game.red_dragon_kills +
        game.red_baron_kills +
        game.red_tower_kills +
        game.red_inhibitor_kills +
        game.red_ward_placed +
        game.red_ward_kills +
        game.red_kills +
        game.red_death +
        game.red_assist +
        game.red_champion_damage_dealt +
        game.red_total_gold +
        game.red_total_minion_kills +
        game.red_total_level +
        game.red_avg_level +
        game.red_jungle_minion_kills +
        game.red_killing_spree +
        game.red_total_heal +
        game.red_object_damage_dealt;

    (blue_score, red_score)
}

fn main() {
    // Read and parse the dataset
    let mut rdr = ReaderBuilder::new().from_path("data/Challenger_Ranked_Games.csv").unwrap();
    
    for result in rdr.deserialize() {
        let game: GameData = result.unwrap();
        
        // Calculate scores for each game
        let (blue_score, red_score) = calculate_score(&game);

        // Compare scores and analyze results
        if blue_score > red_score {
            println!("Blue team wins!");
        } else if red_score > blue_score {
            println!("Red team wins!");
        } else {
            println!("It's a tie!");
        }
    }
}

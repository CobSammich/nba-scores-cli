extern crate reqwest;

use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

// TODO:
// * function that tells you if a game has started yet or not -- given a game_block?
// * struct Game: will hold all information of a game block

enum TimeZone {
    // defines the timezone index as listed on the nbcsports website
    Pacific,
    Mountain,
    Central,
    Eastern,
}

// Parameters set by some config file
static MY_TIMEZONE: TimeZone = TimeZone::Eastern;
// end Params

//fn get_game_start_time() -> String {

//}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {


    let url = "https://scores.nbcsports.com/nba/scoreboard.asp?meta=true";
    let resp = reqwest::get(url).await?;
    assert!(resp.status().is_success());
    let document = Document::from(&*resp.text().await?);

    let mut n_games = 0;
    for row in document.find(Class("shsScoreboardRow")) {
        // there are two games per row
        for game_block in row.find(Class("shsScoreboardCol")) {
            // find home and away team names
            let teams: Vec<String> = game_block
                .find(Class("shsNamD").descendant(Name("a")))
                .map(|tag| tag.text())
                .collect::<Vec<String>>();

            // there are 15 values in here formatted like this:
            // 1 2 3 4 Tot
            // _ _ _ _ ___
            // _ _ _ _ ___
            // where the underscores represent values scored in the 1st, 2nd, 3rd, and 4th quarters
            // as well as the whole game -- labeled by the header row
            let scores: Vec<String> = game_block
                .find(Class("shsTotD"))
                .map(|tag| tag.text())
                .collect::<Vec<String>>();

            let away_team = teams.get(0).expect("No team names in this vector");
            let home_team = teams.get(1).expect("No team names in this vector");

            let game_time: String;
            let away_score: u32;
            let home_score: u32;
            // check if game has started -- if it hasn't the quarter number header will not be
            // present
            if scores.len() == 10 {
                // handle when the game has not yet started -- initialize scores to zero and scrape
                // game start time
                println!("Game has not started");
                away_score = 0;
                home_score = 0;
                let time_zones: Vec<String> = game_block
                    .find(Class("shsTimezone"))
                    .map(|tag| tag.text())
                    .collect::<Vec<String>>();

                //game_time = get_game_start_time(game_block);
                game_time = match MY_TIMEZONE {
                    TimeZone::Pacific => String::from(time_zones
                                                      .get(0)
                                                      .expect("Could not read time zone")),
                    TimeZone::Mountain => String::from(time_zones
                                                       .get(1)
                                                       .expect("Could not read time zone")),
                    TimeZone::Central => String::from(time_zones
                                                      .get(2)
                                                      .expect("Could not read time zone")),
                    TimeZone::Eastern => String::from(time_zones
                                                      .get(3)
                                                      .expect("Could not read time zone")),
                    // NaN case
                    _ => String::from(""),
                };
            }
            else {
                // if game goes into OT, they tack on a score column: solution is to find the
                // number of columns and index with that
                let n_cols = scores.len() / 3;
                away_score = scores.get(n_cols * 2 - 1)
                    .expect("Could not get away team's score")
                    .parse::<u32>()
                    .unwrap();
                home_score = scores.get(n_cols * 3 - 1)
                    .expect("Could not get home team's score")
                    .parse::<u32>()
                    .unwrap();
                game_time = game_block.find(Class("shsTeamCol"))
                    .next()
                    .unwrap()
                    .text();
            }

            println!("{} @ {}", away_team, home_team);
            println!("{} - {}", away_score, home_score);
            println!("{}", game_time);
            println!("");
            n_games += 1;
        }
    }
    println!("{} NBA games today.", n_games);

    return Ok(());
}

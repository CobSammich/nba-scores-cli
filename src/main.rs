// external packages
extern crate reqwest;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

// internal packages
mod team;
use team::Team;

// TODO:
// * function that tells you if a game has started yet or not -- given a game_block?
// * struct Game: will hold all information of a game block

enum TimeZone {
    // Defines different timezones (US only for now)
    Pacific,
    Mountain,
    Central,
    Eastern,
}

struct Game {
    has_started: bool,
    away_team: Team,
    home_team: Team,
    game_time: String,
}


// Parameters set by some config file
static MY_TIMEZONE: TimeZone = TimeZone::Eastern;
// end Params

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn form_teams(game_block: select::node::Node) -> (Team, Team) {
    // Does all the html parsing to make teams
    // returns (home_team, away_team)

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

    // get team leaders
    let stat_leaders_raw: Vec<String> = game_block
        .find(Class("shsLeader"))
        .map(|tag| tag.text())
        .collect::<Vec<String>>();

    // Done parsing html
    let away_team_name = teams.get(0).expect("No team names in this vector");
    let home_team_name = teams.get(1).expect("No team names in this vector");

    // store the stat leader names and the values of those stats
    let mut stat_leader_names: Vec<String> = Vec::new();
    let mut stat_leader_values: Vec<u32> = Vec::new();

    // get each teams scores:
    // Critical design choice here: We decide that if there are no scores found (there are 10
    // whitespace regions), then we return just the names of the teams
    if scores.len() == 10 {
        //let away_team = Team::default(away_team_name);
        let home_team = Team {
            name: String::from(home_team_name),
            ..Team::default()
        };
        let away_team = Team {
            name: String::from(away_team_name),
            ..Team::default()
        };
        return (home_team, away_team);
    }

    // there are scores shown -- continue
    let n_cols = scores.len() / 3;
    let away_score = scores.get(n_cols * 2 - 1)
        .expect("Could not get away team's score")
        .parse::<u32>()
        .unwrap();
    let home_score = scores.get(n_cols * 3 - 1)
        .expect("Could not get home team's score")
        .parse::<u32>()
        .unwrap();

    // there will be 6 values in this
    for val in stat_leaders_raw {
        let val_as_str = String::from(val);
        let val_split_by_whitespace = val_as_str.split_whitespace().collect::<Vec<&str>>();
        // form player name from every value but the final string
        let player_name = &val_split_by_whitespace[..val_split_by_whitespace.len() - 1].join(" ");
        // final string in vector represents the value of the stat category
        let number = val_split_by_whitespace
            .last()
            .expect("Can't retrieve last value in array")
            .parse::<u32>()
            .unwrap();
        stat_leader_names.push(player_name.clone());
        stat_leader_values.push(number);
        // stat_leaders_raw is freed
    }

    // Instantiate teams from the values we just scraped
    let home_team = Team {
        name: String::from(home_team_name),
        score: home_score,
        points_leader: String::from(stat_leader_names
                                    .get(1)
                                    .expect("Could not read game leader")),
        points_leader_value: *stat_leader_values
            .get(1)
            .expect("Could not read game leader"),
        rebounds_leader: String::from(stat_leader_names
                                      .get(3)
                                      .expect("Could not read game leader")),
        rebounds_leader_value: *stat_leader_values
            .get(3)
            .expect("Could not read game leader"),
        assists_leader: String::from(stat_leader_names
                                     .get(5)
                                     .expect("Could not read game leader")),
        assists_leader_value: *stat_leader_values
            .get(5)
            .expect("Could not read game leader"),
    };

    let away_team = Team {
        name: String::from(away_team_name),
        score: away_score,
        points_leader: String::from(stat_leader_names
                                    .get(0)
                                    .expect("Could not read game leader")),
        points_leader_value: *stat_leader_values
            .get(0)
            .expect("Could not read game leader"),
        rebounds_leader: String::from(stat_leader_names
                                      .get(2)
                                      .expect("Could not read game leader")),
        rebounds_leader_value: *stat_leader_values
            .get(2)
            .expect("Could not read game leader"),
        assists_leader: String::from(stat_leader_names
                                     .get(4)
                                     .expect("Could not read game leader")),
        assists_leader_value: *stat_leader_values
            .get(4)
            .expect("Could not read game leader"),
    };

    return (home_team, away_team);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let url = String::from("https://scores.nbcsports.com/nba/scoreboard.asp?meta=true");
    let url = String::from("https://scores.nbcsports.com/nba/scoreboard.asp?day=20220103");
    let resp = reqwest::get(url).await?;
    assert!(resp.status().is_success());
    let document = Document::from(&*resp.text().await?);

    let mut n_games = 0;
    for row in document.find(Class("shsScoreboardRow")) {
        // there are two games per row
        for game_block in row.find(Class("shsScoreboardCol")) {
            // given a game block, form two Teams and a Game

            let (home_team, away_team) = form_teams(game_block);
            println!("{:?}", home_team);
            println!("{:?}", away_team);


            //print_type_of(&game_block);
            // find home and away team names
            //let game_time: String;
            //let away_score: u32;
            //let home_score: u32;
            // check if game has started -- if it hasn't the quarter number header will not be
            // present
            //if scores.len() == 10 {
                // handle when the game has not yet started -- initialize scores to zero and scrape
                // game start time
                //println!("Game has not started");
                //away_score = 0;
                //home_score = 0;
                //let time_zones: Vec<String> = game_block
                    //.find(Class("shsTimezone"))
                    //.map(|tag| tag.text())
                    //.collect::<Vec<String>>();

                //game_time = get_game_start_time(game_block);
                //game_time = match MY_TIMEZONE {
                    //TimeZone::Pacific => String::from(time_zones
                                                      //.get(0)
                                                      //.expect("Could not read time zone")),
                    //TimeZone::Mountain => String::from(time_zones
                                                       //.get(1)
                                                       //.expect("Could not read time zone")),
                    //TimeZone::Central => String::from(time_zones
                                                      //.get(2)
                                                      //.expect("Could not read time zone")),
                    //TimeZone::Eastern => String::from(time_zones
                                                      //.get(3)
                                                      //.expect("Could not read time zone")),
                    // NaN case
                    //_ => String::from(""),
                //};
            //}
            //else {
                //// if game goes into OT, they tack on a score column: solution is to find the
                //// number of columns and index with that
                //let n_cols = scores.len() / 3;
                //away_score = scores.get(n_cols * 2 - 1)
                    //.expect("Could not get away team's score")
                    //.parse::<u32>()
                    //.unwrap();
                //home_score = scores.get(n_cols * 3 - 1)
                    //.expect("Could not get home team's score")
                    //.parse::<u32>()
                    //.unwrap();
                //game_time = game_block.find(Class("shsTeamCol"))
                    //.next()
                    //.unwrap()
                    //.text();
            //}

            //println!("{} @ {}", away_team, home_team);
            //println!("{} - {}", away_score, home_score);
            //println!("{}", game_time);
            //println!("");
            //n_games += 1;
        }
    }
    //println!("{} NBA games today.", n_games);

    return Ok(());
}

// external packages
extern crate reqwest;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
use chrono;

// internal packages
mod team;
use team::Team;

// TODO:
// * Major refactoring of form_game function
// * down side of making team.rs public??
// * Implement Scraping module that scrapes the relevant information from the site
// * How to display and format game leaders info
// * Team name colors
// * command line arguments
//      * today to show today's schedules
//      * tomorrow to show tomorrow's schedules
//      * yesterday to show yesterday's schedules
// * Change all array accesses from .expect to a match statement checking for Ok or Err
//      - See chapter 9.2
//      - JK maybe not?


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

impl Game {
    fn display(&self) {
        println!("{}", format!("{:^16}@{:^16}{:^3} - {:^3}\t{:^9}",
                         self.away_team.name, self.home_team.name,
                         self.away_team.score, self.home_team.score,
                         self.game_time
                         )
                 );
        // if game has started, then print the stat leaders
    }
}


// Parameters set by some config file
static MY_TIMEZONE: TimeZone = TimeZone::Eastern;
// end Params

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn form_game(game_block: select::node::Node) -> Game {
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
    let away_team_name = String::from(teams.get(0).expect("No team names in this vector"));
    let home_team_name = String::from(teams.get(1).expect("No team names in this vector"));

    // get each teams scores:
    // Critical design choice here: We decide that if there are no scores found (there are 10
    // whitespace regions), then we return just the names of the teams
    // TODO/refactor: functionalize this
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
        // find game start time based on Timezone
        let time_zones: Vec<String> = game_block
            .find(Class("shsTimezone"))
            .map(|tag| tag.text())
            .collect::<Vec<String>>();

        // TODO/refactor: FUNCTIONALIZE
        //game_time = get_game_start_time(game_block);
        let game_time = match MY_TIMEZONE {
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

        let game = Game {
            has_started: false,
            away_team: away_team,
            home_team: home_team,
            game_time: game_time,
        };

        return game;
    }

    // scrape gametime -- this is unfortunately a different html tag if the game hasn't started yet
    let game_time = game_block.find(Class("shsTeamCol"))
        .next()
        .unwrap()
        .text();

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

    // store the stat leader names and the values of those stats
    let mut home_leader_names: Vec<String> = Vec::new();
    let mut away_leader_names: Vec<String> = Vec::new();
    let mut home_leader_values: Vec<u32> = Vec::new();
    let mut away_leader_values: Vec<u32> = Vec::new();

    // there will be 6 values in this
    let mut counter = 0;
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
        if counter % 2 == 1 {
            // home team values
            home_leader_names.push(player_name.clone());
            home_leader_values.push(number);
        }
        else {
            // away team values
            away_leader_names.push(player_name.clone());
            away_leader_values.push(number);
        }
        counter += 1;
        // stat_leaders_raw is freed
    }

    // Instantiate teams from the values we just scraped
    let home_team = Team::from_leader_vector(home_team_name, home_score, home_leader_names,
                                             home_leader_values);
    let away_team = Team::from_leader_vector(away_team_name, away_score, away_leader_names,
                                             away_leader_values);

    let game = Game {
        has_started: true,
        away_team: away_team,
        home_team: home_team,
        game_time: game_time,
    };

    return game;
}

fn print_header() {
    println!("{}", format!("{:^16} {:^16}{:^10}\t{:^10}", "Away", "Home", "Score", "Status"));
    println!("{}", format!("{:^16} {:^16}{:^10}\t{:^10}", "----", "----", "-----", "------"));
}

fn get_current_date() -> String {
    // Returns String in the form YYYYMMDD

    // get current time
    let current_time = chrono::offset::Local::now().to_string();

    // Process the string to get the info we want
    // first line filters to YYYY-MM-DD
    // second line makes it a vector with ["YYYY", "MM", "DD"]
    return current_time.split_whitespace().collect::<Vec<&str>>()[0]
        .split('-').collect::<Vec<&str>>()[0..3]
        .join("");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current_date = get_current_date();
    let url_base = String::from("https://scores.nbcsports.com/nba/scoreboard.asp?day=");
    //let url = format!("{}{}", )
    let url = url_base + &current_date;
    //let url = String::from("https://scores.nbcsports.com/nba/scoreboard.asp?meta=true");
    //let url = String::from("https://scores.nbcsports.com/nba/scoreboard.asp?day=20220108");
    let resp = reqwest::get(url).await?;
    assert!(resp.status().is_success());
    let document = Document::from(&*resp.text().await?);

    print_header();

    //let mut n_games = 0;
    for row in document.find(Class("shsScoreboardRow")) {
        // there are two games per row
        for game_block in row.find(Class("shsScoreboardCol")) {
            // given a game block, form two Teams and a Game
            // parse the current game block for game info
            let game: Game = form_game(game_block);
            //println!("{:?}", game.home_team);
            //println!("{:?}", game.away_team);

            // print current game info to terminal
            game.display();

            //print_type_of(&game_block);
        }
    }
    return Ok(());
}

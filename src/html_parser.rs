use select::predicate::{Class, Name, Predicate};

use crate::team::Team;
use crate::game::{Game, create_nonstarted_game};

/// Parses a game block document node to retrieve the two team names.
///
/// # Arguments
///
/// * `game_block` - A Node (from select.rs) object containing tags with
///
/// # Examples
///
/// ```
/// let (home_team_name, away_team_name) = get_team_names(game_block);
/// ```
fn get_team_names(game_block: select::node::Node) -> (String, String) {
    let teams: Vec<String> = game_block
        .find(Class("shsNamD").descendant(Name("a")))
        .map(|tag| tag.text())
        .collect::<Vec<String>>();

    // TODO: panicking here when (I'm assuming) we don't have an HTML document to actually parse --
    // it failed to read the site
    let away_team_name = String::from(teams.get(0).expect("No team names in this vector"));
    let home_team_name = String::from(teams.get(1).expect("No team names in this vector"));
    return (home_team_name, away_team_name);
}

/// Parses a game block document node to retrieve two team scores.
///
/// # Arguments
///
/// * `game_block` - A Node (from select.rs) object containing tags with
///
/// # Examples
///
/// ```
/// let (home_score, away_score) = get_team_scores(game_block);
/// ```
fn get_team_scores(game_block: select::node::Node) -> (u32, u32) {
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

    // Critical design choice here: We decide that if there are no scores found (there are 10
    // whitespace regions), then we return the max u32 value
    if scores.len() == 10 {
        return (u32::MAX, u32::MAX);
    }


    // there are scores shown -- continue
    let n_cols = scores.len() / 3;
    let away_score = scores
        .get(n_cols * 2 - 1)
        .expect("Could not get away team's score")
        .parse::<u32>()
        .unwrap();
    let home_score = scores
        .get(n_cols * 3 - 1)
        .expect("Could not get home team's score")
        .parse::<u32>()
        .unwrap();

    return (home_score, away_score);
}

/// Parses the HTML game block region and populates the given vectors with Names and values of game
/// leaders
///
/// # Arguments
///
/// * `game_block` - The HTML region representing the game containing game leaders in points,
/// rebounds and assists
/// * `home_leader_names` - The empty vector to populate game leader names for the home team
/// * `home_leader_values` - The empty vector to populate game leader values for the home team
/// * `away_leader_names` - The empty vector to populate game leader names for the away team
/// * `away_leader_values` - The empty vector to populate game leader values for the away team
///
/// # Examples
/// Assuming game_block is already defined
///
/// ```
/// let mut home_leader_names: Vec<String> = Vec::new();
/// let mut away_leader_names: Vec<String> = Vec::new();
/// let mut home_leader_values: Vec<u32> = Vec::new();
/// let mut away_leader_values: Vec<u32> = Vec::new();
///
/// // populate the vectores defined above.
/// get_game_leaders(game_block, &mut home_leader_names, &mut home_leader_values, &mut away_leader_names, &mut away_leader_values);
/// ```
fn get_game_leaders(game_block: select::node::Node,
    home_leader_names: &mut Vec<String>, home_leader_values: &mut Vec<u32>,
    away_leader_names: &mut Vec<String>, away_leader_values: &mut Vec<u32>) {
    // get team leaders from html
    let stat_leaders_raw: Vec<String> = game_block
        .find(Class("shsLeader"))
        .map(|tag| tag.text())
        .collect::<Vec<String>>();

    // store the stat leader names and the values of those stats

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
        } else {
            // away team values
            away_leader_names.push(player_name.clone());
            away_leader_values.push(number);
        }
        counter += 1;
        // stat_leaders_raw is freed
    }
}

// Public functions

/// Parses the HTML document (Node) and forms two Team objects and then a Game object from those
/// two teams.
///
/// # Arguments
///
/// * `game_block` - A Node (from select.rs) object containing tags with
///
/// # Examples
///
/// ```
/// // retrieve a "game block" from a html document
/// let document = Document::from(&*resp.text().await?);
/// let game_block = document.find(Class("shsScoreboardRow")).find(Class("shsScoreboardCol"));
/// let game: Game = form_game(game_block);
/// // To display the game
/// game.display();
/// ```
pub fn form_game(game_block: select::node::Node) -> Game {
    // Does all the html parsing to make teams
    let (home_team_name, away_team_name) = get_team_names(game_block);
    let (home_score, away_score) = get_team_scores(game_block);
    let mut home_leader_names: Vec<String> = Vec::new();
    let mut away_leader_names: Vec<String> = Vec::new();
    let mut home_leader_values: Vec<u32> = Vec::new();
    let mut away_leader_values: Vec<u32> = Vec::new();

    get_game_leaders(game_block, &mut home_leader_names, &mut home_leader_values, &mut away_leader_names, &mut away_leader_values);
    // Done parsing html

    // NOTE: This is a check for if the game has started yet or not
    // The get_team_scores function returns max u32 value if the game has yet to start
    if home_score == u32::MAX && away_score == u32::MAX {
        let game = create_nonstarted_game(&home_team_name, &away_team_name, game_block);
        return game;
    }

    // scrape gametime -- this is unfortunately a different html tag if the game hasn't started yet
    let game_time = game_block.find(Class("shsTeamCol")).next().unwrap().text();

    // Instantiate teams from the values we just scraped
    let home_team = Team::from_leader_vector(
        home_team_name,
        home_score,
        home_leader_names,
        home_leader_values
    );
    let away_team = Team::from_leader_vector(
        away_team_name,
        away_score,
        away_leader_names,
        away_leader_values
    );

    let game = Game {
        has_started: true,
        away_team,
        home_team,
        game_time,
    };

    return game;
}

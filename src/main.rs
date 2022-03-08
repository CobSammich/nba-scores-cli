// external packages
extern crate reqwest;
use chrono;
use clap::Parser;
use select::document::Document;
use select::predicate::Class;

// internal packages
mod constants;
mod display;
mod game;
mod parser;
mod team;
mod timezones;

use crate::game::Game;
use crate::parser::form_game;
use crate::display::print_header;

// TODO:
// * Major refactoring of form_game function
// * down side of making team.rs public??
// * Implement Scraping module that scrapes the relevant information from the site
// * How to display and format game leaders info
// * Team name colors
// * A class that handles making dates look human readable and interprettable to the program

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Date of games to retrieve. must be in YYYYMMDD format.
    /// Shortcuts:
    /// t: today
    /// T: tomorrow
    /// y: yesterday
    #[clap(short, long, default_value = "t")]
    date: String,
}

// end Params

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

// --------------------------------------
fn format_date(date: String) -> String {
    // Given a String in the form YYYY-MM-DD, Returns String in the form YYYYMMDD
    return date.split('-').collect::<Vec<&str>>()[0..3].join("");
}

// TODO: return Result??
/// Parse date argument and return the given date in YYYYMMDD format
fn extract_date_argument(date: &String) -> String {
    // retrieve current date -- chrono makes getting surrounding days EASY
    let current_date = chrono::offset::Local::now().date();

    match date.as_str() {
        // check for shortcut arguments -- use current date
        "t" => return format_date(current_date.to_string()),
        "T" => return format_date(current_date.succ().to_string()),
        "y" => return format_date(current_date.pred().to_string()),
        // TODO: handle the different date formats here by passing them into conversion function
        // If the string given is a usable date, panic!
        _ => panic!(
            "Please give a recognizable date format. The formats recognized are \
                    YYYYMMDD, ..."
        ),
    };
    // TODO check if in usable format
    //if !date_format_usable(&date) {
    // TODO if not supplied with valid date -- panic!
    // How do we decide what date formats we can use?

    //}

    //println!("Could not understand date given.. showing Today's scores.");
    //return extract_date_argument(&String::from("t"));
}

// Should this be Result?

fn date_format_usable(date: &String) -> bool {
    // trim off extra characters, if any
    if date.len() != 8 {
        return false;
    }
    println!("{}", &date);

    return false;
}
// --------------------------------------

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args = Args::parse();
    // handle date
    let date = extract_date_argument(&args.date);
    // form the url to look for based on the date.
    let url_base = String::from("https://scores.nbcsports.com/nba/scoreboard.asp?day=");
    let url = url_base + &date;

    // Get the webpage
    let resp = reqwest::get(url).await?;
    assert!(resp.status().is_success());
    let document = Document::from(&*resp.text().await?);

    print_header();

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
        }
    }
    return Ok(());
}

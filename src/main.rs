// external packages
extern crate reqwest;
use chrono;
use clap::Parser;
use select::document::Document;
use select::predicate::Class;

// internal packages
mod constants;
mod date_handler;
mod display;
mod game;
mod html_parser;
mod team;
mod timezones;

use crate::date_handler::extract_date_argument;
use crate::display::print_header;
use crate::game::Game;
use crate::html_parser::form_game;

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

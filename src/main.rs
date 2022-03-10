// external packages
extern crate reqwest;
extern crate termion;

use std::{thread, time};
use chrono;
use clap::Parser;
use select::document::Document;
use select::predicate::Class;


use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::async_stdin;


//use tokio::io::stdout;
use std::io::{Read, Write, stdout, stdin};

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
    //let mut stdin = async_stdin().bytes();
    // Parse command line arguments
    let args = Args::parse();
    // handle date
    let date = extract_date_argument(&args.date);
    // form the url to look for based on the date.
    let url_base = String::from("https://scores.nbcsports.com/nba/scoreboard.asp?day=");
    let url = url_base + &date;

    //stdin controls user input
    // program loop -- re-fetch html and display games every 10 seconds
    let mut stdin = async_stdin().bytes();
    'program_loop: loop {
        // controller for detecting 'q' key to exit program
        // clear terminal
        // Get the webpage
        let resp = reqwest::get(&url).await?;
        assert!(resp.status().is_success());
        let document = Document::from(&*resp.text().await?);

        // clear terminal and set program to write in top left of terminal
        // TODO: Functionalize this (1)
        clear_terminal();
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

        // loop to exit
        //let mut bytes = stdin.bytes();
        let mut counter = 0;
        'inner: loop {
            //let b = bytes.next().unwrap().unwrap();
            let stdout = stdout();
            let mut stdout = stdout.lock().into_raw_mode().unwrap();
            let b = stdin.next();
            //write!(stdout, "\r{:?}    <- This demonstrates the async read input char. Between each update a 100 ms. is waited, simply to demonstrate the async fashion. \n\r", b).unwrap();
            write!(stdout, "\r").unwrap();
            //println!("{:?}", b);
            if let Some(Ok(b'q')) = b {
                // clean up and end program
                print!("\x1B[2J");
                write!(stdout,
                       "{}{}",
                       termion::cursor::Goto(1, 1),
                       termion::cursor::Show)
                       .unwrap();
                stdout.flush().unwrap();
                break 'program_loop;
            }

            let sleep_time_in_ms = 50;
            let sleep_time = time::Duration::from_millis(sleep_time_in_ms);
            thread::sleep(sleep_time);
            counter += 1;
            // wait 10 seconds before re-running program
            if counter * sleep_time_in_ms >= 10000 {
                break 'inner;
            }
        }
        //let sleep_time = time::Duration::from_secs(10);
        //thread::sleep(sleep_time);

        //break 'program_loop
    }
    return Ok(());
}


fn clear_terminal() {
    write!(stdout(),
           "{}{}{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           termion::cursor::Hide)
           .unwrap();
}

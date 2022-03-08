use select::predicate::{Class, Name, Predicate};

use crate::team::Team;
use colored::Colorize;
use crate::timezones::TimeZone;
use crate::constants::{TEAM_COLORS, MY_TIMEZONE};

pub struct Game {
    pub has_started: bool,
    pub away_team: Team,
    pub home_team: Team,
    pub game_time: String,
}

impl Game {
    pub fn display(&self) {
        println!(
            "{}",
            format!(
                "{:^16}@{:^16}{:^5} - {:^5}\t{:^9}",
                &self.away_team.name[..].on_truecolor(self.away_team.color_value.0, self.away_team.color_value.1, self.away_team.color_value.2),
                &self.home_team.name[..].on_truecolor(self.home_team.color_value.0, self.home_team.color_value.1, self.home_team.color_value.2),
                self.away_team.score,
                self.home_team.score,
                self.game_time
            )
        );
        // if game has started, then print the stat leaders
    }
}

pub fn create_nonstarted_game(home_team_name: &str, away_team_name: &str, game_block: select::node::Node) -> Game {
    //let away_team = Team::default(away_team_name);
    // TODO: need to put team color in here
    let home_team = Team {
        name: String::from(home_team_name),
        color_value: TEAM_COLORS[&home_team_name[..]],
        ..Team::default()
    };
    let away_team = Team {
        name: String::from(away_team_name),
        color_value: TEAM_COLORS[&away_team_name[..]],
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
        TimeZone::Pacific => String::from(time_zones.get(0).expect("Could not read time zone")),
        TimeZone::Mountain => {
            String::from(time_zones.get(1).expect("Could not read time zone"))
        }
        TimeZone::Central => String::from(time_zones.get(2).expect("Could not read time zone")),
        TimeZone::Eastern => String::from(time_zones.get(3).expect("Could not read time zone")),
    };

    let game = Game {
        has_started: false,
        away_team,
        home_team,
        game_time,
    };
    return game;
}


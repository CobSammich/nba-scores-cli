use crate::constants::TEAM_COLORS;

#[derive(Debug)]
pub struct Team {
    // team name
    pub name: String,
    // team score -- TODO Does it make sense to make this mutable? Yes if the program runs in a
    // loop later on
    pub score: u32,
    // points leader name and their number of points scored
    pub points_leader: String,
    pub points_leader_value: u32,
    // rebounds leader name and number of rebounds
    pub rebounds_leader: String,
    pub rebounds_leader_value: u32,
    // assists leader name and number of assists
    pub assists_leader: String,
    pub assists_leader_value: u32,

    // Color information
    pub color_value: (u8, u8, u8),
}

// default constructor when passed no arguments to
impl Default for Team {
    fn default() -> Team {
        Team {
            // team name
            name: String::from(""),
            // team score
            score: 0,
            // points leader name and their number of points scored
            points_leader: String::from(""),
            points_leader_value: 0,
            // rebounds leader name and number of rebounds
            rebounds_leader: String::from(""),
            rebounds_leader_value: 0,
            // assists leader name and number of assists
            assists_leader: String::from(""),
            assists_leader_value: 0,
            color_value: (0, 0, 0)
        }

    }
}

impl Team {
    pub fn from_leader_vector(name: String, score: u32,
                          leader_names: Vec<String>, leader_values: Vec<u32>) -> Team {
        // create team from vectors of team leaders and values
        Team {
            name: String::from(&name),
            score,
            points_leader: String::from(leader_names
                                        .get(0)
                                        .expect("Could not read game leader")),
            points_leader_value: *leader_values
                .get(0)
                .expect("Could not read game leader"),
            rebounds_leader: String::from(leader_names
                                          .get(1)
                                          .expect("Could not read game leader")),
            rebounds_leader_value: *leader_values
                .get(1)
                .expect("Could not read game leader"),
            assists_leader: String::from(leader_names
                                         .get(2)
                                         .expect("Could not read game leader")),
            assists_leader_value: *leader_values
                .get(2)
                .expect("Could not read game leader"),
            color_value: TEAM_COLORS[&name[..]]
        }

    }

}


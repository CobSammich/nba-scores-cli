
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
}

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
        }

    }

}

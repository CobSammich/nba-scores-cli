
fn format_date(date: String) -> String {
    // Given a String in the form YYYY-MM-DD[-HH-MM], Returns String in the form ?month=MM&day=DD&year=YYYY
    let date_vec: Vec<&str> = date.split('-').collect();
    if date_vec.len() < 3 {
        panic!("Invalid date format: expected at least YYYY-MM-DD");
    }
    let year = date_vec[0];
    let month = date_vec[1];
    let day = date_vec[2];
    format!("?month={}&day={}&year={}", month, day, year)
}

// TODO: return Result??
/// Parse date argument and return the given date in YYYYMMDD format
pub fn extract_date_argument(date: &String) -> String {
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

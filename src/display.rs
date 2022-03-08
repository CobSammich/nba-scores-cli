pub fn print_header() {
    println!(
        "{}",
        format!(
            "{:^16} {:^16}{:^13}\t{:^9}",
            "Away", "Home", "Score", "Status"
        )
    );
    println!(
        "{}",
        format!(
            "{:^16} {:^16}{:^13}\t{:^9}",
            "----", "----", "-----", "------"
        )
    );
}


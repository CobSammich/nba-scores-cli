use std::io::{Write, stdout};

/// Prints a header for the program in the following format:
///       Away             Home          Score       Status                                                                                                                                                             │
///       ----             ----          -----       ------
///
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

/// Clears the terminal and repositions any output to be written at the top left of the terminal.
/// This is used right before we write any output to the terminal.
///
/// # Examples
///
/// Basic usage:
/// ```
/// clear_terminal();
/// ```
pub fn clear_terminal() {
    write!(stdout(),
           "{}{}{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           termion::cursor::Hide)
           .unwrap();
}

/// Clears the terminal, shows the cursor at the top left and flushed stdout. Used when we are
/// ending the program
///
/// # Examples
///
/// ```
/// cleanup_terminal();
/// ```
pub fn cleanup_terminal() {
    let mut stdout = stdout();
    write!(stdout,
           "{}{}{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           termion::cursor::Show)
           .unwrap();
    stdout.flush().unwrap();
}

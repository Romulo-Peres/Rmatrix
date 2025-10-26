use std::io::Stdout;

use crossterm::{cursor, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};

pub fn configure_terminal(stdout: &mut Stdout) {
    execute!(stdout, EnterAlternateScreen, cursor::Hide).expect("Could not configure terminal to run the application");

    enable_raw_mode().expect("Could not enable raw mode");
}

pub fn reset_terminal_to_default_settings(stdout: &mut Stdout) {
    execute!(stdout, LeaveAlternateScreen, cursor::Show).expect("Could not reset the terminal");

    disable_raw_mode().expect("Could not disable raw mode");
}
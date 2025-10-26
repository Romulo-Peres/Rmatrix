use chrono::{DateTime, Utc};
use crossterm::{self, cursor::MoveTo, execute, style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor}, QueueableCommand};
use rand::{self, Rng};
use std::io::{Stdout, Write};

use crate::utilities;

#[derive(Debug)]
pub struct Stream {
    characters: String,
    length: u16,
    column: u16,
    row: u16,
    trail_color: [u8; 3],
    head_color: [u8; 3],
    next_row_delay: u16,
    last_row_increment: DateTime<Utc>,
}

impl Stream {
    pub fn new(
        terminal_columns: u16,
        mut trail_color: [u8; 3],
        mut head_color: [u8; 3],
        min_delay: u16,
        max_delay: u16,
        is_background_stream: bool,
    ) -> Self {
        let next_row_delay;

        if is_background_stream {
            next_row_delay = max_delay;
            utilities::shade_stream_colors(&mut trail_color, &mut head_color);
        } else {
            next_row_delay = rand::thread_rng().gen_range(min_delay..max_delay);
        }

        let stream_characters = utilities::generate_stream_characters(terminal_columns);
        let stream_length = utilities::stream_length();
        let stream_position = utilities::stream_position(terminal_columns);

        return Stream {
            characters: stream_characters,
            length: stream_length,
            column: stream_position,
            row: 0,
            trail_color,
            head_color,
            next_row_delay,
            last_row_increment: Utc::now(),
        };
    }

    pub fn try_to_increment_row(&mut self) {
        let diff = Utc::now() - self.last_row_increment;

        if diff.num_milliseconds() > self.next_row_delay as i64 {
            self.last_row_increment = Utc::now();
            self.row += 1;
        }
    }

    pub fn draw(&self, terminal_rows: u16, stdout: &mut Stdout) {
        let mut string_to_print: char;
        let mut char_position: u8 = 0;
        let mut enable_path_cleaning: bool = true;

        let last_visible_position = match self.row.checked_sub(self.length) {
            Some(value) => value,
            None => 0,
        };

        if last_visible_position == 0 {
            enable_path_cleaning = false;
        }

        for i in (last_visible_position..=self.row).rev() {
            /* Ignore the part of the stream that is offscreen */
            if i > terminal_rows {
                continue;
            }

            if (i as usize) <= self.characters.len() {
                string_to_print = self.characters.chars().collect::<Vec<_>>()[i as usize];

                stdout
                    .queue(MoveTo(self.column, i))
                    .expect("Failed to update cursor position");

                if i == self.row {
                    print_stream_head(stdout, string_to_print, self.head_color);
                } else {
                    print_stream_trail(
                        stdout,
                        string_to_print,
                        char_position,
                        self.trail_color,
                    );
                }
            }

            if i == last_visible_position && enable_path_cleaning {
                match i.checked_sub(1) {
                    Some(v) => execute!(stdout, MoveTo(self.column, v), Print(" "))
                        .expect("Could not clear the stream path"),
                    None => {}
                }
            }

            char_position += 1;
        }
    }

    pub fn is_offscreen(&self, terminal_rows: u16) -> bool {
        match self.row.checked_sub(self.length) {
            Some(column_tail) => {
                if column_tail > terminal_rows {
                    return true;
                }

                return false;
            }
            None => false,
        }
    }
}

pub fn print_stream_head(stdout: &mut Stdout, value: char, head_color: [u8; 3]) {
    stdout
        .queue(SetForegroundColor(Color::Rgb {
            r: head_color[0],
            g: head_color[1],
            b: head_color[2],
        }))
        .expect("Could not set foreground color");

    stdout
        .queue(SetAttribute(Attribute::Bold))
        .expect("Could not set bold style");
    stdout
        .write(value.to_string().as_bytes())
        .expect("Could not print a stream character");
    stdout
        .queue(ResetColor)
        .expect("Could not reset terminal color");
    stdout
        .queue(SetAttribute(Attribute::Reset))
        .expect("Could not reset bold style");
}

pub fn print_stream_trail(
    stdout: &mut Stdout,
    value: char,
    char_position: u8,
    mut body_color: [u8; 3],
) {
    for color in &mut body_color {
        *color = match color.checked_sub(char_position * 9) {
            Some(result) => result,
            None => 0,
        }
    }

    let body_color = Color::Rgb {
        r: body_color[0],
        g: body_color[1],
        b: body_color[2],
    };

    stdout
        .queue(SetForegroundColor(body_color))
        .expect("Could not set foreground color");
    stdout
        .write(value.to_string().as_bytes())
        .expect("Could not print a stream character");
    stdout
        .queue(ResetColor)
        .expect("Could not reset terminal color");
}

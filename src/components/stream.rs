use crossterm::{self, cursor::MoveTo, execute, style::Print, QueueableCommand};
use rand::{self, Rng};
use std::io::Stdout;
use chrono::{DateTime, Utc};

use crate::colors;

#[derive(Debug)]
pub struct Stream {
    characters: String,
    length: u16,
    column: u16,
    row: u16,
    body_color: [u8; 3],
    edge_color: [u8; 3],
    next_row_delay: u16,
    last_row_increment: DateTime<Utc>
}

impl Stream {
    pub fn new(terminal_columns: u16, body_color: [u8; 3], edge_color: [u8; 3], min_delay: u16, max_delay: u16) -> Self {
        return Stream {
            characters: column_characters(terminal_columns),
            length: column_length(),
            column: column_position(terminal_columns),
            row: 0,
	        body_color,
	        edge_color,
            next_row_delay: rand::thread_rng().gen_range(min_delay..max_delay),
            last_row_increment: Utc::now()
        };
    }

    pub fn try_to_increment_row(&mut self) {
        let diff = Utc::now() - self.last_row_increment;

        if diff.num_milliseconds() > self.next_row_delay as i64{
            self.last_row_increment = Utc::now();
            self.row += 1;
        }
    }

    pub fn draw(
        &self,
        terminal_rows: u16,
        stdout: &mut Stdout
    ) {
        let mut string_to_print: char;
        let mut char_position: u8 = 0;
        let can_clear_the_path: bool;

        let last_visible_position = match self.row.checked_sub(self.length) {
            Some(value) => {
                can_clear_the_path = true;

                value
            }
            None => {
                can_clear_the_path = false;

                0
            }
        };

        for i in (last_visible_position..=self.row).rev() {

            if i > terminal_rows {
                continue;
            }

            if (i as usize) <= self.characters.len() {
                string_to_print = self.characters.chars().collect::<Vec<_>>()[i as usize];

                stdout
                    .queue(MoveTo(self.column, i))
                    .expect("Error on update cursor position");
    
                if i == self.row {
                    colors::print_column_nose(stdout, string_to_print, self.edge_color);
                } else {
                    colors::print_column_body(stdout, string_to_print, char_position, self.body_color);
                }
            }

            if i == last_visible_position && can_clear_the_path {
                match i.checked_sub(1) {
                    Some(v) => execute!(stdout, MoveTo(self.column, v), Print(" "))
                        .expect("Could not clear the column path"),
                    None => {}
                }
            }

            char_position += 1;
        }
    }

    pub fn out_of_visible_area(&self, terminal_rows: u16) -> bool {
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

fn column_position(columns: u16) -> u16 {
    let mut rng = rand::thread_rng();

    return rng.gen_range(0..columns);
}

fn column_length() -> u16 {
    let mut rng = rand::thread_rng();

    return rng.gen_range(5..=18);
}

fn column_characters(terminal_columns: u16) -> String {
    let available_characters = "'/a0b!1c@2d#3e$%4f&3g*(6h)7i|_8j-+9k=[]\\1l,/2m3n;.4o5p~`6q7r8s9\"^t0u1v2w3x4y5z6";
    let mut characters = String::new();
    let mut rng = rand::thread_rng();
    let mut slice_index;
    let mut i = 0;

    while i <= terminal_columns + 5 {
        slice_index = rng.gen_range(1..=available_characters.len());
        characters.push_str(&available_characters[slice_index - 1..slice_index]);

        i += 1;
    }

    return characters;
}

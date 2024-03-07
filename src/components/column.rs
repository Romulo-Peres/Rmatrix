use std::io::Stdout;
use rand::{self, Rng};
use crossterm::{self, cursor::MoveTo, execute, style::Print, QueueableCommand};

use crate::colors;

#[derive(Debug)]
pub struct Column {
  characters : String,
  length : u16,
  column : u16,
  row : u16
}

impl Column {
  pub fn new(terminal_columns : u16) -> Self {
    return Column {
      characters : column_characters(terminal_columns),
      length : column_length(),
      column : column_position(terminal_columns),
      row : 0
    }
  }

  pub fn increment_row(&mut self) {
    self.row += 1;
  }

  pub fn draw(&self, terminal_rows : u16, stdout : &mut Stdout, edge_color : [u8; 3], body_color : [u8; 3]){
    let mut string_to_print : &str;
    let mut char_position : u8 = 0;
    let can_clear_the_path : bool;

    let last_visible_position = match self.row.checked_sub(self.length) {
      Some(value) => {
        can_clear_the_path = true;

        value
      },
      None => {
        can_clear_the_path = false;

        0
      }
    };

    for i in (last_visible_position..=self.row).rev() {
      if i > terminal_rows {
        continue;
      }

      string_to_print = &self.characters[(i as usize)..(i as usize)+1];
      
      stdout.queue(MoveTo(self.column, i))
       .expect("Error on update cursor position");

      if i == self.row {
        colors::print_column_nose(stdout, string_to_print, edge_color);
      } else {
        colors::print_column_body(stdout, string_to_print, char_position, body_color);
      }
      

      if i == last_visible_position && can_clear_the_path {
        match i.checked_sub(1) {
            Some(v) => {
              execute!(
                stdout,
                MoveTo(self.column, v),
                Print(" ")
              ).expect("Could not clear the column path")
            },
            None => {}
        }
      }

      char_position += 1;
    }

  }

  pub fn out_of_visible_area(&self, terminal_rows : u16) -> bool {
    match self.row.checked_sub(self.length) {
      Some(column_tail) => {
        if column_tail > terminal_rows {
          return true;
        }

        return false;
      },
      None => false
    }
  }
}

fn column_position(columns : u16) -> u16 {
  let mut rng = rand::thread_rng();

  return rng.gen_range(0..columns);
}

fn column_length() -> u16 {
  let mut rng = rand::thread_rng();

  return rng.gen_range(5..=18);
}

fn column_characters(terminal_columns : u16) -> String {
  let available_characters = "a0b1c2d3e4f3g6h7i8j9k1l2m3n4o5p6q7r8s9t0u1v2w3x4y5z6";
  let mut characters = String::new();
  let mut rng = rand::thread_rng();
  let mut slice_index;

  for _ in 0..terminal_columns {
    slice_index = rng.gen_range(1..=available_characters.len());
    characters.push_str(&available_characters[slice_index-1..slice_index]);
  }

  return characters;
}
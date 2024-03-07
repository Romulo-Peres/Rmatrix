use std::io::{Stdout, Write};

use crossterm::{style::{Attribute, Color, ResetColor, SetAttribute, SetForegroundColor}, QueueableCommand};

pub fn print_column_nose(stdout : &mut Stdout, value : &str, edge_color : [u8; 3]){
  stdout.queue(SetForegroundColor(
    Color::Rgb { r: edge_color[0], g: edge_color[1], b: edge_color[2]}
  )).expect("Could not set foreground color");

  stdout.queue(SetAttribute(Attribute::Bold)).expect("Could not set bold style");
  stdout.write(value.as_bytes()).expect("Could not print column character");
  stdout.queue(ResetColor).expect("Could not reset terminal color");
  stdout.queue(SetAttribute(Attribute::Reset)).expect("Could not reset bold style");
}

pub fn print_column_body(stdout : &mut Stdout, value : &str, char_position : u8, mut body_color : [u8; 3]) {
  
  for color in &mut body_color {
    *color = match color.checked_sub(char_position * 9) {
      Some(result) => result,
      None => 0
    }
  }

  let body_color = Color::Rgb { r: body_color[0], g: body_color[1], b: body_color[2] };

  stdout.queue(SetForegroundColor(body_color)).expect("Could not set foreground color");
  stdout.write(value.as_bytes()).expect("Could not print column character");
  stdout.queue(ResetColor).expect("Could not reset terminal color");
}
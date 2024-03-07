use std::io::{Stdout, Write};

use crossterm::{style::{Attribute, Color, ResetColor, SetAttribute, SetForegroundColor}, QueueableCommand};

pub fn print_column_nose(stdout : &mut Stdout, value : &str){
  stdout.queue(SetForegroundColor(Color::White)).expect("Could not set foreground color");
  stdout.queue(SetAttribute(Attribute::Bold)).expect("Could not set bold style");
  stdout.write(value.as_bytes()).expect("Could not print column character");
  stdout.queue(ResetColor).expect("Could not reset terminal color");
  stdout.queue(SetAttribute(Attribute::Reset)).expect("Could not reset bold style");
}

pub fn print_column_body(stdout : &mut Stdout, value : &str, char_position : u8, column_len : u8) {
  let green_value = 255 - char_position * 9;

  let rgb = Color::Rgb { r: 0, g: green_value, b: 0 };

  stdout.queue(SetForegroundColor(rgb)).expect("Could not set foreground color");
  stdout.write(value.as_bytes()).expect("Could not print column character");
  stdout.queue(ResetColor).expect("Could not reset terminal color");
}
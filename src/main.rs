use std::{io::{stdout, Write}, sync::mpsc, thread, time::Duration};

use components::column::Column;
use crossterm::{cursor::{self}, terminal::{size, Clear, ClearType}, ExecutableCommand, QueueableCommand};

mod components;
mod controllers;
mod colors;

fn main() {
  let (column_create_tx, column_create_rx) = mpsc::channel();
  let (exit_program_tx, exit_program_rx) = mpsc::channel();
  let mut columns_vector : Vec<Column> = Vec::new();
  let (terminal_width, terminal_height) = size().unwrap();
  let mut stdout = stdout();

  controllers::column_creator_controller(column_create_tx, 50);
  controllers::exit_program_controller(exit_program_tx);

  stdout.execute(cursor::Hide).expect("Could not hide the cursor");
  stdout.queue(Clear(ClearType::All))
     .expect("Could not clear the terminal");

  while
    match exit_program_rx.try_recv() {
      Ok(_) => false,
      Err(_) => true
    }
  {
    match column_create_rx.try_recv() {
      Ok(_) => {
        columns_vector.push(Column::new(terminal_width));
      },
      Err(_) => {}
    }

    
    columns_vector.retain_mut(|e| {
      e.increment_row();

      if e.out_of_visible_area(terminal_height) {
        return false;
      }

      e.draw(terminal_height, &mut stdout);

      return true;
    });

    stdout.flush().unwrap();

    thread::sleep(Duration::from_millis(50));
  }

  stdout.execute(cursor::Show).expect("Could not show the cursor");
}
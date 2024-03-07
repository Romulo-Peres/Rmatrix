use std::{io::{stdout, Write}, sync::{mpsc, Arc, Mutex}, thread, time::Duration};

use components::column::Column;
use crossterm::{cursor::{self}, terminal::{size, Clear, ClearType}, ExecutableCommand, QueueableCommand};

mod components;
mod controllers;
mod colors;

fn main() {
  let (exit_program_tx, exit_program_rx) = mpsc::channel();
  let (terminal_width, terminal_height) = size().unwrap();
  let mut stdout = stdout();
  let mutex: Arc<Mutex<Vec<Column>>> = Arc::new(Mutex::new(Vec::new()));

  controllers::column_creator_controller(10, Arc::clone(&mutex), terminal_width);
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
    {
      let mut vector_mutex = mutex.lock().unwrap();

      vector_mutex.retain_mut(|e| {
        e.increment_row();

        if e.out_of_visible_area(terminal_height) {
          return false;
        }

        e.draw(terminal_height, &mut stdout);

        return true;
      });
    }

    stdout.flush().unwrap();

    thread::sleep(Duration::from_millis(30));
  }

  stdout.execute(cursor::Show).expect("Could not show the cursor");
}
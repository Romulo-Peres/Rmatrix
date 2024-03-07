use std::{io::stdin, sync::{mpsc::Sender, Arc, Mutex}, thread, time::Duration};

use crate::components::column::Column;

pub fn column_creator_controller(creation_timeout : u64, mutex : Arc<Mutex<Vec<Column>>>, terminal_columns : u16) {
  thread::spawn(move || {
    loop {
      {
        let mut columns_vector = mutex.lock().unwrap();

        columns_vector.push(Column::new(terminal_columns));
      }

      thread::sleep(Duration::from_millis(creation_timeout));
    }
  });
}

pub fn exit_program_controller(tx_channel : Sender<bool>) {
  thread::spawn(move || {
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Could not read user input");

    tx_channel.send(true).expect("Could not send exit program message through the channel ");
  });
}
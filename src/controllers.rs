use std::{io::stdin, sync::mpsc::Sender, thread, time::Duration};

pub fn column_creator_controller(tx_channel : Sender<bool>, creation_timeout : u64) {
  thread::spawn(move || {
    loop {
      thread::sleep(Duration::from_millis(creation_timeout));
      tx_channel.send(true).expect("Could not send column create message through the channel");
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
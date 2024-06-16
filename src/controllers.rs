use std::{
    sync::{
        atomic::{AtomicU16, Ordering},
        mpsc::Sender,
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use crossterm::event::{read, Event, KeyCode};

use crate::components::column::Column;

pub fn column_creator_controller(
    creation_timeout: u64,
    mutex: Arc<Mutex<Vec<Column>>>,
    terminal_columns: Arc<AtomicU16>,
) {
    thread::spawn(move || loop {
        {
            let mut columns_vector = mutex.lock().unwrap();

            columns_vector.push(Column::new(terminal_columns.load(Ordering::SeqCst)));
        }

        thread::sleep(Duration::from_millis(creation_timeout));
    });
}

pub fn exit_program_controller(tx_channel: Sender<bool>) {
    thread::spawn(move || loop {
        match read().unwrap() {
            Event::Key(event) => {
                if event.code == KeyCode::Enter {
                    tx_channel
                        .send(true)
                        .expect("Could not send message to stop the program")
                }
            }
            _ => {}
        }
    });
}

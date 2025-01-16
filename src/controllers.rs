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

use crate::{components::column::Column, utilities::generate_random_color};

pub fn column_creator_controller(
    creation_timeout: u64,
    mutex: Arc<Mutex<Vec<Column>>>,
    terminal_columns: Arc<AtomicU16>,
    body_color: [u8; 3],
    edge_color: [u8; 3],
    rainbow_mode: bool
) {
    thread::spawn(move || loop {
        {
            let mut columns_vector = mutex.lock().unwrap();

	    if rainbow_mode {
		columns_vector.push(Column::new(terminal_columns.load(Ordering::SeqCst), generate_random_color(), [255, 255, 255]));
	    } else {
		columns_vector.push(Column::new(terminal_columns.load(Ordering::SeqCst), body_color, edge_color));
	    }
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

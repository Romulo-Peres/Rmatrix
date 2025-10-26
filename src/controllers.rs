use std::{
    sync::{
        atomic::{AtomicU16, Ordering},
        mpsc::Sender,
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

use rand::Rng;

use crossterm::event::{read, Event, KeyCode};

use crate::{components::stream::Stream, utilities::generate_random_color};

pub fn stream_creator_controller(
    creation_timeout: u64,
    mutex: Arc<Mutex<Vec<Stream>>>,
    terminal_columns: Arc<AtomicU16>,
    body_color: [u8; 3],
    edge_color: [u8; 3],
    rainbow_mode: bool,
    minimum_stream_delay: u16,
    maximum_stream_delay: u16
) {
    thread::spawn(move || loop {
        {
            let mut columns_vector = mutex.lock().unwrap();

	        if rainbow_mode {
		        columns_vector.push(Stream::new(terminal_columns.load(Ordering::SeqCst), generate_random_color(), [255, 255, 255], minimum_stream_delay, maximum_stream_delay, fifty_percent()));
	        } else {
    		    columns_vector.push(Stream::new(terminal_columns.load(Ordering::SeqCst), body_color, edge_color, minimum_stream_delay, maximum_stream_delay, fifty_percent()));
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


fn fifty_percent() -> bool {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..2) == 0
}
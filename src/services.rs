use crate::{components::stream::Stream, utilities};
use crossterm::event::{read, Event, KeyCode};

use std::{
    sync::{atomic::{AtomicU16, Ordering}, mpsc::Sender, Arc, Mutex},
    thread,
    time::Duration,
};

pub fn spawn_stream_creation_service(
    creation_timeout: u64,
    mutex: Arc<Mutex<Vec<Stream>>>,
    terminal_columns: Arc<AtomicU16>,
    mut trail_color: [u8; 3],
    mut head_color: [u8; 3],
    rainbow_mode: bool,
    minimum_stream_delay: u16,
    maximum_stream_delay: u16
) {
    thread::spawn(move || loop {
        {
            let mut columns_vector = mutex.lock().unwrap();

            if rainbow_mode {
                trail_color = utilities::generate_random_color();
                head_color = [255, 255, 255];
            }

            let stream = Stream::new(
                terminal_columns.load(Ordering::SeqCst),
                trail_color, head_color,
                minimum_stream_delay,
                maximum_stream_delay,
                utilities::fifty_percent()
            );

            columns_vector.push(stream);
        }

        thread::sleep(Duration::from_millis(creation_timeout));
    });
}

pub fn spawn_exit_program_listener_service(tx_channel: Sender<bool>) {
    thread::spawn(move || loop {
        match read().unwrap() {
            Event::Key(event) => {
                if event.code == KeyCode::Enter {
                    tx_channel
                        .send(false)
                        .expect("Could not send message to stop the program")
                }
            }
            _ => {}
        }
    });
}


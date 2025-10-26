mod arguments;
mod components;
mod services;
mod terminal;
mod utilities;

use arguments::{vec_to_array, Args};
use clap::Parser;
use components::stream::Stream;
use crossterm::{
    terminal::{size, Clear, ClearType},
    QueueableCommand,
};
use std::{
    io::{stdout, Write},
    sync::{ atomic::{AtomicU16, Ordering}, mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

fn main() {
    let (exit_program_tx, exit_program_rx) = mpsc::channel();
    let (terminal_width, mut terminal_height) = size().unwrap();
    let mut stdout = stdout();
    let streams_mutex: Arc<Mutex<Vec<Stream>>> = Arc::new(Mutex::new(Vec::new()));
    let program_arguments = Args::parse();
    let terminal_width = Arc::new(AtomicU16::new(terminal_width));
    let program_colors = (
        vec_to_array!(program_arguments.head_color),
        vec_to_array!(program_arguments.trail_color),
    );

    program_arguments.validate();

    services::spawn_stream_creation_service(
        program_arguments.string_interval,
        Arc::clone(&streams_mutex),
        Arc::clone(&terminal_width),
        program_colors.1,
        program_colors.0,
        program_arguments.raindow_mode,
        program_arguments.minimum_stream_delay,
        program_arguments.maximum_stream_delay,
    );
    services::spawn_exit_program_listener_service(exit_program_tx);

    terminal::configure_terminal(&mut stdout);

    while exit_program_rx.try_recv().unwrap_or(true) {
        {
            let mut streams = streams_mutex.lock().unwrap();

            let (new_width, new_height) = size().expect("Error while measuring terminal size");

            if terminal_width.load(Ordering::SeqCst) != new_width || terminal_height != new_height {
                terminal_width.store(new_width, Ordering::SeqCst);
                terminal_height = new_height;

                streams.clear();

                stdout
                    .queue(Clear(ClearType::All))
                    .expect("Could not clear terminal screen");
            }

            streams.retain_mut(|stream| {
                stream.try_to_increment_row();

                stream.draw(terminal_height, &mut stdout);
                
                return !stream.is_offscreen(terminal_height);
            });
        }

        stdout.flush().unwrap();

        thread::sleep(Duration::from_millis(10));
    }

   terminal::reset_terminal_to_default_settings(&mut stdout);
}

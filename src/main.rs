
mod arguments;
mod colors;
mod components;
mod controllers;
mod utilities;

use arguments::{vec_to_array, Args};
use clap::Parser;
use components::stream::Stream;
use crossterm::{
    cursor, execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    QueueableCommand,
};

use std::{
    io::{stdout, Write},
    sync::{
        atomic::{AtomicU16, Ordering},
        mpsc, Arc, Mutex,
    },
    thread,
    time::Duration,
};

fn main() {
    let (exit_program_tx, exit_program_rx) = mpsc::channel();
    let (terminal_width, mut terminal_height) = size().unwrap();
    let mut stdout = stdout();
    let mutex: Arc<Mutex<Vec<Stream>>> = Arc::new(Mutex::new(Vec::new()));
    let program_arguments = Args::parse();
    let terminal_width = Arc::new(AtomicU16::new(terminal_width));
    let program_colors = (
        vec_to_array!(program_arguments.edge_color),
        vec_to_array!(program_arguments.body_color),
    );

    program_arguments.validate();
    
    controllers::stream_creator_controller(
        program_arguments.string_interval,
        Arc::clone(&mutex),
        Arc::clone(&terminal_width),
	    program_colors.1,
	    program_colors.0,
	    program_arguments.raindow_mode,
        program_arguments.minimum_stream_delay,
        program_arguments.maximum_stream_delay
    );
    controllers::exit_program_controller(exit_program_tx);

    execute!(stdout, EnterAlternateScreen, cursor::Hide)
        .expect("Could not configure terminal to run the application");

    enable_raw_mode().expect("Could not enable raw mode");

    while match exit_program_rx.try_recv() {
        Ok(_) => false,
        Err(_) => true,
    } {
        {
            let mut vector_mutex = mutex.lock().unwrap();

            let (new_width, new_height) = size().expect("err");

            if terminal_width.load(Ordering::SeqCst) != new_width || terminal_height != new_height {
                terminal_width.store(new_width, Ordering::SeqCst);
                terminal_height = new_height;
                eprintln!("sdfdsfsdf{terminal_height}");

                vector_mutex.clear();

                stdout
                    .queue(Clear(ClearType::All))
                    .expect("Could not clear terminal screen");
            }

            vector_mutex.retain_mut(|e| {
                e.try_to_increment_row();

                if e.out_of_visible_area(terminal_height) {

                    return false;
                }

		if program_arguments.raindow_mode {
		    e.draw(
			terminal_height,
			&mut stdout
                    );
		} else {
		    e.draw(
			terminal_height,
			&mut stdout
		    );
		}



                return true;
            });
        }

        stdout.flush().unwrap();

        thread::sleep(Duration::from_millis(program_arguments.render_speed));
    }

    execute!(stdout, LeaveAlternateScreen, cursor::Show).expect("Could not reset the terminal");

    disable_raw_mode().expect("Could not disable raw mode");
}

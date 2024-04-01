pub mod model;
use std::io::{stdin, stdout, Read, Write};
use std::sync::mpsc::Receiver;
use console::Key;
use console::Term;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use clap::Parser;
use crossterm::cursor;
use crossterm::QueueableCommand;

fn spawn_stdin_listener() -> Receiver<model::Signal> {
    let (tx, rx) = std::sync::mpsc::channel::<model::Signal>();
    thread::spawn(move || loop {
        let term = Term::stdout();
        let res = term.read_key();
        let signal = res.into_iter().flat_map(|k| model::Signal::get_signal(&k)).collect();
    });
    rx
}

fn main() {
    let args = model::Cli::parse();

    let mut stdout = stdout();

    for i in 0..args.cycles {
        println!("Cycle: {}", i + 1);
        for j in 0..args.sessions_per_cycle {
            println!("Session {}", j + 1);


            for minute in 0..args.work_in_minutes {
                for second in 0..60 {
                    stdout.queue(cursor::SavePosition).unwrap();
                    stdout.write_all(format!("{min:0>2}:{sec:0>2}", min=minute, sec=second).as_bytes()).unwrap();
                    stdout.queue(cursor::RestorePosition).unwrap();
                    stdout.flush().unwrap();
                    println!("up here");
                    let mut on_pause = false;
                    let mut ms_count = 0;
                    println!("before while");
                    while on_pause || ms_count < 1000 {
                        println!("before sleep");
                        sleep(Duration::from_millis(1));
                        println!("before read_key");
                        
                        if let Ok(Key::Char(' ')) = res {
                            on_pause = !on_pause;
                        };
                        println!("on_pause: {}", on_pause);
                        if !on_pause {
                            ms_count += 1;
                            println!("ms_count: {}", ms_count);
                        }
                    } 
                }
                
            }
        }
    };
}

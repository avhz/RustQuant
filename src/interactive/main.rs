//! Interactive binary for the library.
//! This will hopefully be a user interface for the library,
//! in the form of a command line interface (CLI).

// Strictly enforce documentation.
#![forbid(missing_docs)]
// Allow snake case.
// This is because much of this library is based on mathematics, so I
// want to adhere to the standard mathematical notation.
#![allow(non_snake_case)]
// Strictly enforce SAFETY comments.
// There is no unsafe code currently, but for anyone to add any, it must be
// documented with a SAFETY comment.
#![forbid(clippy::undocumented_unsafe_blocks)]

// use crossterm::event;
// use crossterm::terminal::enable_raw_mode;
use serde_json::Value;
// use std::thread;
// use std::time::{Duration, Instant};
use std::{fs::File, sync::mpsc};

const DB: &str = "./src/interactive/db.json";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(DB).expect("Could not open JSON file");
    let json: Value = serde_json::from_reader(file).expect("JSON was not well-formatted");

    println!("Hello, {}!", json.get("users").unwrap());

    // enable_raw_mode().expect("can run in raw mode");

    // let (tx, rx) = mpsc::channel();
    // let tick_rate = Duration::from_millis(200);
    // thread::spawn(move || {
    //     let mut last_tick = Instant::now();
    //     loop {
    //         let timeout = tick_rate
    //             .checked_sub(last_tick.elapsed())
    //             .unwrap_or_else(|| Duration::from_secs(0));

    //         if event::poll(timeout).expect("poll works") {
    //             if let CEvent::Key(key) = event::read().expect("can read events") {
    //                 tx.send(Event::Input(key)).expect("can send events");
    //             }
    //         }

    //         if last_tick.elapsed() >= tick_rate {
    //             if let Ok(_) = tx.send(Event::Tick) {
    //                 last_tick = Instant::now();
    //             }
    //         }
    //     }
    // });

    Ok(())
}

// enum Event<I> {
//     Input(I),
//     Tick,
// }

// enum MenuItem {
//     Home,
//     Pricing,
//     About,
//     Quit,
// }

// impl From<MenuItem> for usize {
//     fn from(input: MenuItem) -> usize {
//         match input {
//             MenuItem::Home => 0,
//             MenuItem::Pricing => 1,
//             MenuItem::About => 2,
//             MenuItem::Quit => 3,
//         }
//     }
// }

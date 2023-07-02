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

/// The banner for the interactive binary.
pub mod banner;

use banner::BANNER;
use serde_json::Value;
use std::fs::File;

const DB: &str = "./src/interactive/db.json";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(DB).expect("Could not open JSON file");
    let json: Value = serde_json::from_reader(file).expect("JSON was not well-formatted");

    println!("Hello, {}!", json.get("users").unwrap());

    println!("{}", BANNER);

    Ok(())
}

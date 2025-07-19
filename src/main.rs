// src/main.rs

mod calculate_pi;

use std::io;
use std::io::Write;
use log::{debug, error, info, warn, trace};
use crate::calculate_pi::{get_pi_derive, get_pi_const};

fn main() {
    // Initialize the logger. `env_logger` will read the RUST_LOG environment
    // variable to determine what log level to show.
    // If RUST_LOG is not set, it defaults to `error`.
    env_logger::init();

    println!("--- Logging Example ---");
    println!("You can change the log level by setting the RUST_LOG environment variable.");
    println!("Example: RUST_LOG=debug cargo run\n");

    trace!("This is such a minor detail that it probably won't get logged very often...");
    
    // These log messages will be filtered based on the RUST_LOG level.
    info!("This is an informational message. It indicates a normal event in the application.");

    debug!("This is a debug message. It's useful for developers to trace the program's execution.");

    warn!("This is a warning. It indicates a potential issue that doesn't prevent the program from running.");

    error!("This is an error message. It signifies a failure or critical problem.");

    let pi_const = get_pi_const();
    println!("Value of pi using rust constant: {}", pi_const);
    let pi_derive = get_pi_derive();
    println!("  Value of pi using rust derive: {}", pi_derive);

    println!("\n--- End of Example ---");
    io::stdout().flush().expect("Could not flush stdout");
}
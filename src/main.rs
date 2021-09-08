use std::process;

use choose_some::Config;

fn main() {
    let config = Config::new().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = choose_some::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

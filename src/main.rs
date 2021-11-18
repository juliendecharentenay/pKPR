use std::process;

mod config;
mod run;

fn main() {
    let config = config::Config::new().unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

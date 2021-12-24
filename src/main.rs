use std::env;
use std::process;

use copy_clone::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::parse(&args).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        process::exit(1);
    });
    if let Err(e) = copy_clone::run(config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
    process::exit(0);
}

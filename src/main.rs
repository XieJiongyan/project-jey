use std::env;
use std::process;

use project_jey::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args);

    if let Err(e) = project_jey::run(config) {
        eprintln!("Application Error: {e}");

        process::exit(1);
    }
}

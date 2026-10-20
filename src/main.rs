mod cli;
mod config;
mod events;

use crate::cli::run_cli;

fn main() {
    let res = run_cli();

    if let Err(error) = res {
        match error {
            config::AppError::File(error) => eprintln!("{:?}", error),
            config::AppError::Seralizer(error) => eprintln!("{:?}", error),
        }
    }
}

use clap::Parser;

mod cli;
mod events;
mod ticket;

use events::TicketEvents;

use crate::cli::run_cli;

fn main() {
    let _ = run_cli();
}

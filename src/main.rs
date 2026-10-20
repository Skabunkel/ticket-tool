use clap::Parser;

mod events;
mod ticket;

use events::TicketEvents;

#[derive(Parser)]
#[command(name = "tk", about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: TicketEvents,
}

fn main() {
    println!("Hello, world!");
}

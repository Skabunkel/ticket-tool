//! The reason for this structure is to allow a server to run and send json requests and use the same events.
//!

use clap::{Subcommand, ValueEnum};

mod create;
mod methods;

use create::CreationEvent;

#[derive(Clone, ValueEnum)]
pub enum TicketType {
    Task,
    Bug,
}

#[derive(Subcommand)]
pub enum TicketEvents {
    #[command(alias = "i")]
    Init,
    #[command(alias = "c", subcommand)]
    Create(CreationEvent),
}

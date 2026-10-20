//! The reason for this structure is to allow a server to run and send json requests and use the same events.
//!

use clap::{Subcommand, ValueEnum};

mod create;
mod methods;

use create::CreationEvent;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug, ValueEnum)]
#[serde(rename_all = "PascalCase")]
/// The Current state of a ticket.
enum TicketState {
    Open,
    InProgress,
    Closed,
    Blocked,
    Deferred,
}

#[derive(Clone, Deserialize, Serialize, Debug, ValueEnum)]
#[serde(rename_all = "lowercase")]
/// The type of tickets we support, to keep it simple we support as few as possible.
pub enum TicketType {
    Task,
    Bug,
}

#[derive(Subcommand)]
/// The cli subcommands.
pub enum TicketEvents {
    #[command(alias = "i")]
    Init,
    #[command(alias = "c", subcommand)]
    Create(CreationEvent),
}

use clap::{Args, Subcommand};
use serde::{Deserialize, Serialize};
use time::Date;

use crate::events::{
    TicketType,
    methods::{parse_date, today},
};

#[derive(Subcommand)]
pub enum CreationEvent {
    #[command(
        alias = "t",
        about = "Creates an open ticket, sets title of the even from name"
    )]
    Ticket(TicketCreation),
    #[command(
        alias = "i",
        about = "Creates an open initiative, sets title of the even from name"
    )]
    Initiative(InitiativeCreation),
}

#[derive(Args, Deserialize, Serialize)]
pub struct TicketCreation {
    name: Box<str>,
    #[arg(short = 'o', default_value_t = false)]
    open: bool,
    #[arg(short = 't', long = "type", value_enum, default_value_t = TicketType::Task)]
    ticket_type: TicketType,
    #[arg(short = 's', long = "start", value_parser = parse_date, default_value_t = today())]
    start_date: Date,
    #[arg(short = 'e', long = "end", value_parser = parse_date)]
    end_date: Option<Date>,
}

#[derive(Args, Deserialize, Serialize)]
pub struct InitiativeCreation {
    name: Box<str>,
    #[arg(short = 'o', default_value_t = false)]
    open: bool,
    #[arg(short = 's', long = "start", value_parser = parse_date, default_value_t = today())]
    start_date: Date,
    #[arg(short = 'e', long = "end", value_parser = parse_date)]
    end_date: Option<Date>,
}

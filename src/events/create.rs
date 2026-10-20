use clap::{Args, Subcommand};
use serde::{Deserialize, Serialize};
use time::Date;

use crate::events::{
    ItemState, TicketType,
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
    pub name: Box<str>,
    #[arg(short = 'o', default_value_t = false)]
    pub open: bool,
    #[arg(short = 't', long = "type", value_enum, default_value_t = TicketType::Task)]
    pub ticket_type: TicketType,
    #[arg(short = 's', long = "start", value_parser = parse_date, default_value_t = today())]
    pub start_date: Date,
    #[arg(short = 'e', long = "end", value_parser = parse_date)]
    pub end_date: Option<Date>,
}
#[derive(Deserialize, Serialize)]
pub struct TicketObj {
    pub ticket_type: TicketType,
    pub state: ItemState,
    pub start_date: Date,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Date>,
}

#[derive(Args, Deserialize, Serialize)]
pub struct InitiativeCreation {
    pub name: Box<str>,
    #[arg(short = 'o', default_value_t = false)]
    pub open: bool,
    #[arg(short = 's', long = "start", value_parser = parse_date, default_value_t = today())]
    pub start_date: Date,
    #[arg(short = 'e', long = "end", value_parser = parse_date)]
    pub end_date: Option<Date>,
}

#[derive(Deserialize, Serialize)]
pub struct InitiativeObj {
    pub state: ItemState,
    pub start_date: Date,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Date>,
}

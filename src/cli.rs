use std::{fs, path::PathBuf};

use crate::{
    config::{self, AppError},
    events::{
        CreationEvent, InitiativeCreation, InitiativeObj, ItemState, TicketCreation, TicketEvents,
        TicketObj,
    },
};
use clap::Parser;
use rand::RngExt;

#[derive(Parser)]
#[command(name = "tk", about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: TicketEvents,
}

pub const BASE_FOLDER_NAME: &str = ".ticket";
const TICKET_FOLDER_NAME: &str = "tickets";
const INIT_FOLDER_NAME: &str = "initaitves";
pub const CONFIG_FILE_NAME: &str = "conf.yaml";

fn init(mut dir: PathBuf) -> Result<(), AppError> {
    // Create base and tickt folder
    dir.push(BASE_FOLDER_NAME);
    dir.push(TICKET_FOLDER_NAME);
    fs::create_dir_all(&dir).map_err(AppError::File)?;

    // Create initiatives folder
    dir.pop();
    dir.push(INIT_FOLDER_NAME);
    fs::create_dir_all(&dir).map_err(AppError::File)?;

    // Creates the .conf file
    dir.pop();
    dir.push(CONFIG_FILE_NAME);
    if !dir.try_exists().is_ok_and(|x| x) {
        fs::write(dir, include_str!("conf_default.yaml")).map_err(AppError::File)?;
    }

    Ok(())
}

fn random_name(prefix: Box<str>, len: usize) -> String {
    const HEX: &[u8] = b"0123456789abcdef";
    let mut rng = rand::rng();
    let number: String = (0..len)
        .map(|_| HEX[rng.random_range(0..16)] as char)
        .collect();

    format!("{}-{}.md", prefix, number)
}

fn create_ticket(
    mut current_dir: PathBuf,
    ticket: TicketCreation,
    prefix: Box<str>,
    length: u8,
) -> Result<(), AppError> {
    current_dir.push(TICKET_FOLDER_NAME);
    let yml = TicketObj {
        state: ItemState::Open,
        ticket_type: ticket.ticket_type,
        start_date: ticket.start_date,
        end_date: ticket.end_date,
    };

    let name = random_name(prefix, length as usize);
    current_dir.push(name);

    let yaml = yaml_serde::to_string(&yml).map_err(AppError::Seralizer)?;
    write_file(current_dir, yaml, ticket.name, ticket.open)
}
fn create_initiative(
    mut current_dir: PathBuf,
    initiative: InitiativeCreation,
    prefix: Box<str>,
    length: u8,
) -> Result<(), AppError> {
    current_dir.push(INIT_FOLDER_NAME);
    let yml = InitiativeObj {
        state: ItemState::Open,
        start_date: initiative.start_date,
        end_date: initiative.end_date,
    };

    let name = random_name(prefix, length as usize);
    current_dir.push(name);

    let yaml = yaml_serde::to_string(&yml).map_err(AppError::Seralizer)?;
    write_file(current_dir, yaml, initiative.name, initiative.open)
}

fn write_file(path: PathBuf, yaml: String, title: Box<str>, open: bool) -> Result<(), AppError> {
    // This is the format used to create a ticket; i will probably change this at some point to make it better, but i want to get started.
    let text = format!("---\n{}\n---\n\n# {}\n", yaml, title);
    fs::write(&path, text).map_err(AppError::File)?;

    if open {
        let editor = std::env::var("EDITOR")
            .ok()
            .unwrap_or(std::env::var("VISUAL").unwrap_or("nano".into()));
        let _ss = std::process::Command::new(editor).arg(&path).status();
    }

    Ok(())
}

fn create(mut current_dir: PathBuf, creation_event: CreationEvent) -> Result<(), AppError> {
    current_dir.push(BASE_FOLDER_NAME);
    let conf = config::read_config(current_dir.clone())?;

    match creation_event {
        CreationEvent::Ticket(ticket_creation) => create_ticket(
            current_dir,
            ticket_creation,
            conf.ticket_prefix,
            conf.length,
        ),
        CreationEvent::Initiative(initiative_creation) => create_initiative(
            current_dir,
            initiative_creation,
            conf.initiative_prefix,
            conf.length,
        ),
    }
}

pub fn run_cli() -> Result<(), AppError> {
    let cli = Cli::parse();
    let current_dir = std::env::current_dir().map_err(AppError::File)?;

    match cli.command {
        TicketEvents::Init => init(current_dir),
        TicketEvents::Create(_creation_event) => create(current_dir, _creation_event),
    }
}

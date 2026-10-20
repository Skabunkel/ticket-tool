use std::{fs, path::PathBuf};

use crate::events::TicketEvents;
use clap::Parser;

#[derive(Parser)]
#[command(name = "tk", about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: TicketEvents,
}

const BASE_FOLDER_NAME: &str = ".ticket";
const TICKET_FOLDER_NAME: &str = "tickets";
const INIT_FOLDER_NAME: &str = "initaitves";
const CONFIG_FILE_NAME: &str = "conf.yaml";

fn init(mut dir: PathBuf) -> std::io::Result<()> {
    // Create base and tickt folder
    dir.push(BASE_FOLDER_NAME);
    dir.push(TICKET_FOLDER_NAME);
    fs::create_dir_all(&dir)?;

    // Create initiatives folder
    dir.pop();
    dir.push(INIT_FOLDER_NAME);
    fs::create_dir_all(&dir)?;

    // Creates the .conf file
    dir.pop();
    dir.push(CONFIG_FILE_NAME);
    if !dir.try_exists().is_ok_and(|x| x) {
        fs::write(dir, include_str!("conf_default.yaml"))?;
    }

    Ok(())
}

pub fn run_cli() -> std::io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        TicketEvents::Init => init(std::env::current_dir()?)?,
        TicketEvents::Create(_creation_event) => todo!(),
    }

    Ok(())
}

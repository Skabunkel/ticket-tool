use std::{fs::File, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::cli;

#[derive(Serialize, Deserialize)]
pub struct Conf {
    pub ticket_prefix: Box<str>,
    pub initiative_prefix: Box<str>,
    pub length: u8,
}

pub enum AppError {
    File(std::io::Error),
    Seralizer(yaml_serde::Error),
}

pub fn read_config(mut path: PathBuf) -> Result<Conf, AppError> {
    path.push(cli::CONFIG_FILE_NAME);

    let reader = File::open(path).map_err(|x| AppError::File(x))?;
    let conf: Conf = yaml_serde::from_reader(reader).map_err(|x| AppError::Seralizer(x))?;
    Ok(conf)
}

// SPDX-License-Identifier: Apache-2.0

#![warn(clippy::all, clippy::pedantic)]

use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

use dirs::preference_dir;
use eyre::{eyre, Result, WrapErr};

pub(crate) fn get_config_file_path() -> Result<PathBuf> {
    let mut config_file_path =
        preference_dir().ok_or_else(|| eyre!("Could not find config directory"))?;
    config_file_path.push("tad");
    if !config_file_path.exists() {
        fs::create_dir_all(&config_file_path)
            .wrap_err_with(|| format!("Could not create directory at {:?}", &config_file_path))?;
    }
    config_file_path.push("tad.json");
    if config_file_path.exists() && config_file_path.is_file() {
        Ok(config_file_path)
    } else if config_file_path.is_dir() {
        Err(eyre!(
            "{:#?} found, but it is a directory",
            &config_file_path
        ))
    } else {
        File::create(&config_file_path)?;
        Err(eyre!(
            "No config file found, a blank one was created at {:#?}",
            &config_file_path
        ))
    }
}

pub(crate) fn load_config_file<P: AsRef<Path> + Debug>(path: P) -> Result<HashMap<String, String>> {
    let config_file =
        File::open(&path).wrap_err_with(|| format!("Could not open file at {:?}.", path))?;
    let mut buffered_reader = BufReader::new(config_file);
    let mut contents = String::new();
    buffered_reader
        .read_to_string(&mut contents)
        .wrap_err_with(|| "Could not read file into string!")?;
    let map: HashMap<String, String> = serde_json::from_str(&contents).wrap_err_with(|| {
        format!(
            "The config file at {:?} is empty! Please populate it!",
            &path
        )
    })?;
    Ok(map)
}

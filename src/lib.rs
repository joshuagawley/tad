// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

use chrono::Utc;
use chrono_tz::Tz;
use dirs::config_dir;
use eyre::{bail, eyre, Result, WrapErr};

fn get_config_file_path() -> Result<PathBuf> {
    let mut config_file_path = config_dir().unwrap();
    config_file_path.push("tad");
    if !config_file_path.exists() {
        fs::create_dir_all(&config_file_path)
            .wrap_err_with(|| eyre!("Could not create directory"))?;
    }
    config_file_path.push("tad.json");
    if config_file_path.exists() && config_file_path.is_file() {
        Ok(config_file_path)
    } else {
        if config_file_path.is_dir() {
            Err(eyre!(
                "{:#?} found, but it is a directory",
                config_file_path
            ))
        } else {
            File::create(&config_file_path)?;
            Err(eyre!(
                "No config file found, a blank one was created at {:#?}",
                config_file_path
            ))
        }
    }
}

fn load_config_file(path: PathBuf) -> Result<HashMap<String, String>> {
    let config_file = File::open(path)?;
    let mut buffered_reader = BufReader::new(config_file);
    let mut contents = String::new();
    buffered_reader.read_to_string(&mut contents)?;
    let map: HashMap<String, String> = serde_json::from_str(&contents)?;
    Ok(map)
}

pub fn print_result(person: &str, timezone: &str) -> Result<()> {
    let tz: Tz = match timezone.parse() {
        Ok(v) => v,
        Err(..) => bail!("Could not parse timezone string {}.\n", timezone),
    };
    let utc = Utc::now().with_timezone(&tz);
    let date_string = utc.format("%A %d %B %Y");
    let time_string = utc.format("%H:%M");
    println!(
        "The current date and time for {} is {} {}.",
        person, date_string, time_string
    );
    Ok(())
}

pub fn run(person_to_find: &str) -> Result<()> {
    let person_to_find = &person_to_find.to_lowercase();
    let config_file_path = get_config_file_path()?;
    let config: HashMap<String, String> = load_config_file(config_file_path)?;
    for (person, timezone) in config {
        if *person_to_find == *person {
            print_result(&person, &timezone)?;
            return Ok(());
        }
    }
    Err(eyre!("Could not find {} in config file", &person_to_find))
}

// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use chrono::Utc;
use chrono_tz::Tz;
use eyre::{bail, eyre, Result};

use crate::config::{get_config_file_path, load_config_file};

mod config;

fn print_result(person: &str, timezone: &str) -> Result<()> {
    let tz: Tz = match timezone.parse() {
        Ok(v) => v,
        Err(..) => bail!("Could not parse timezone string {}.\n", timezone),
    };
    let current_time = Utc::now().with_timezone(&tz);
    let date_string = current_time.format("%A %d %B %Y");
    let time_string = current_time.format("%H:%M");
    println!(
        "The current date and time for {} is {} {}.",
        person, date_string, time_string
    );
    Ok(())
}

pub fn run(person_to_find: &str) -> Result<()> {
    let person_to_find = &person_to_find.to_lowercase();
    let config_file_path = get_config_file_path()?;
    let config: HashMap<String, String> = load_config_file(&config_file_path)?;
    if config.contains_key(person_to_find) {
        print_result(person_to_find, &*config[person_to_find])?;
        Ok(())
    } else {
        Err(eyre!("Could not find {} in config file", &person_to_find))
    }
}

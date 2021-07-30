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

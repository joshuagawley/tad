// SPDX-License-Identifier: Apache-2.0

#![warn(clippy::all, clippy::pedantic)]

use eyre::{eyre, Result};

use tad::run;

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.len() {
        1 => {
            run(&args[0])?;
            Ok(())
        }
        0 => Err(eyre!("Too few arguments specified")),
        _ => Err(eyre!("Too many arguments specified")),
    }
}

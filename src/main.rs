// SPDX-License-Identifier: Apache-2.0

#![warn(clippy::all, clippy::pedantic)]

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let args: Vec<String> = std::env::args().skip(1).collect();
    if !(args.len() == 1) {
        Err(eyre::eyre!("Too many arguments specified"))
    } else {
        tad::run(&args[0])?;
        Ok(())
    }
}

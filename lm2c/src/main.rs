use std::time::{Duration, Instant};

use clap::Parser;

use anyhow::Result;
use common::{assembler_error, assembler_info};


mod args;
mod common;
mod tokeniser;
mod generator;

fn main() -> Result<()>{
    let start_time = Instant::now();
    let cli_args = args::Args::parse();

    let code = std::fs::read_to_string(&cli_args.src_file)?;


    let mut tokeniser = tokeniser::Tokeniser::new(cli_args.src_file, code);
    tokeniser.parse()?;

    if tokeniser.errors != 0 {
        assembler_error(format!("Could not compile due to {} previous errors", tokeniser.errors).as_str());
        return Ok(());
    }
    // dbg!(tokeniser.get_program());

    let mut generator = generator::Generator::new(tokeniser.get_program());
    generator.generate();

    if generator.size > 240 {
        assembler_error("Binary size too big");
        return Ok(());
    } else {
        std::fs::write(cli_args.output, generator.get_bin())?;

        let duration = start_time.elapsed();
        assembler_info(format!("Compilation OK, took {}ms, bin size 480B", duration.as_millis()).as_str())
    }

    // dbg!(generator.get_bin());

    Ok(())
}

use clap::Parser;

use anyhow::Result;
use common::assembler_error;


mod args;
mod common;
mod tokeniser;
mod generator;

fn main() -> Result<()>{

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

    std::fs::write(cli_args.output, generator.get_bin())?;

    Ok(())
}

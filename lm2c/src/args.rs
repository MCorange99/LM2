use clap::{Parser, arg};

const AUTHOR: &'static str = "MCorange";
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

const ABOUT: &'static str = "An assembler made for Markceluna's custom 16 bit architecture.";
const LONG_ABOUT: &'static str = "\
An assembler made for Markceluna's custom 16 bit architecture.
This was increadibly fun to make in 2 days and i thank her for the inspiration. <3\
";


#[derive(Parser)]
#[command(author=AUTHOR, version=VERSION, about=ABOUT, long_about=LONG_ABOUT)]
pub struct Args {
    pub src_file: String,

    #[arg(short='o', default_value="a.out")]
    pub output: String,
}
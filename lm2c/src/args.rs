use clap::{Parser, arg};

#[derive(Parser)]
pub struct Args {
    pub src_file: String,

    #[arg(short='o', default_value="a.out")]
    pub output: String,
}
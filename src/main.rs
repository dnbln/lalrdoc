extern crate clap;
extern crate lalrdoc;
extern crate pico_args;

use clap::Clap;
use lalrdoc::cli::Cli;

fn main() -> Result<(), lalrdoc::reference_builder::LalrdocError> {
    let cli: Cli = lalrdoc::cli::Cli::parse();

    lalrdoc::cli::run(cli)
}

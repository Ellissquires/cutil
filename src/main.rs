mod core;
use crate::core::commands::add::*;
use crate::core::commands::search::*;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "cutil", about = "command utilities")]
pub enum Cli {
    #[structopt(name = "add")]
    Add {
        #[structopt(short = "c")]
        command: String,
        #[structopt(short = "i")]
        identifier: Option<String>,
    },
    #[structopt(name = "alias")]
    Alias {
        #[structopt(short = "n")]
        name: String,
    },
    #[structopt(name = "search")]
    Search {
        #[structopt(short = "q")]
        query: String,
    },
}

fn main() {
    match Cli::from_args() {
        Cli::Add {
            command,
            identifier,
        } => add(command, identifier),
        Cli::Alias { name: _ } => (),
        Cli::Search { query } => search(query),
    }
}

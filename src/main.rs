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
        #[structopt(short = "t")]
        tag: Option<String>,
    },
    #[structopt(name = "alias")]
    Alias {
        #[structopt(short = "n")]
        name: String,
    },
    #[structopt(name = "search")]
    Search {
        #[structopt(short = "q", required_unless = "tag")]
        query: Option<String>,
        #[structopt(short = "t", required_unless = "query")]
        tag: Option<String>,
    },
}

fn main() {
    match Cli::from_args() {
        Cli::Add { command, tag } => add(command, tag),
        Cli::Alias { name: _ } => (),
        Cli::Search { query, tag } => match search(query, tag) {
            Ok(_res) => (),
            Err(e) => panic!("Problem selecting command: {:?}", e),
        },
    }
}

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::io::Write;

use crate::core::models::command::Command;
use crate::core::util::file::open;

#[derive(Serialize, Deserialize, Debug)]
pub struct Commands {
  commands: Vec<Command>,
}

impl Commands {
  pub fn add(&mut self, command: Command) {
    self.commands.push(command);
  }

  pub fn new() -> Commands {
    Commands {
      commands: Vec::new(),
    }
  }

  pub fn write(self) {
    let commands: String = serde_json::to_string(&self).unwrap();
    std::fs::write("commands.json", &commands).expect("Problem writing to commands file");
  }
}

pub fn load_commands() -> Commands {
  let commands_file = open("commands.json", &init_commands_file);
  return parse_commands_file(commands_file);
}

fn parse_commands_file(mut file: File) -> Commands {
  let mut data = String::new();
  file.read_to_string(&mut data).unwrap();

  return serde_json::from_str(&data).expect("JSON was not well-formatted");
}

fn init_commands_file(mut file: File) -> File {
  let commands = Commands::new();
  let initial_commands: String = serde_json::to_string(&commands).unwrap();
  file
    .write_all(initial_commands.as_bytes())
    .expect("Problem writing initial commands file content");

  return File::open("commands.json").expect("Problem opening the created commands file");
}

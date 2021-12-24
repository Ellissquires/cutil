// use crate::core::models::command::Command;
use crate::core::models::commands::load_commands;
use crate::core::models::commands::Commands;

pub fn search(query: String) {
  let mut commands: Commands = load_commands();
  println!("Executing query: {}", query);
}

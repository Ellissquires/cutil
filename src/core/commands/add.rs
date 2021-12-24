use crate::core::models::command::Command;
use crate::core::models::commands::load_commands;
use crate::core::models::commands::Commands;

pub fn add(command: String, identifier: Option<String>) {
  let mut commands: Commands = load_commands();
  let new_command: Command = Command {
    value: command,
    identifier: identifier.unwrap_or_default(),
  };
  println!("Adding command: {:?}", new_command);
  commands.add(new_command);
  commands.write();
}

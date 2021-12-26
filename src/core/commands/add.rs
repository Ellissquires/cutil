use crate::core::models::command::Command;
use crate::core::models::commands::load_commands;
use crate::core::models::commands::Commands;

pub fn add(command: String, tag: Option<String>) {
  let mut commands: Commands = load_commands();
  let new_command: Command = Command {
    value: command,
    tag: tag.unwrap_or_default(),
  };
  commands.add(new_command);
  commands.write();
}

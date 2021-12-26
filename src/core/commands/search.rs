// use crate::core::models::command::Command;
use crate::core::models::command::Command;
use crate::core::models::commands::load_commands;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;

pub fn search(query: Option<String>, tag: Option<String>) -> std::io::Result<()> {
  let commands: Vec<Command> = load_commands().commands;
  let query = query.unwrap_or_default();
  let tag = tag.unwrap_or_default();

  let results: Vec<&String> = commands
    .iter()
    .filter(|command| command.value.contains(&query) || tag == command.tag)
    .map(|command| &command.value)
    .collect();

  if !results.is_empty() {
    let selection = Select::with_theme(&ColorfulTheme::default())
      .default(0)
      .items(&results)
      .interact()?;
    println!("{:?}", commands[selection]);
  } else {
    println!("No results found.");
  }

  Ok(())
}

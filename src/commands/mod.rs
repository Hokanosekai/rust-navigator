pub mod ls;
pub mod cd;

use std::collections::HashMap;

use crate::acli::ACli;

pub type CommandFn = fn(Vec<&str>, &mut ACli) -> Result<(), String>;

#[derive(Clone, Debug)]
pub struct CommandWrapper(pub CommandFn);

#[derive(Clone, Debug)]
pub struct CommandManager {
  commands: HashMap<String, CommandWrapper>,
}

impl CommandManager {
  pub fn new() -> CommandManager {
    CommandManager {
      commands: HashMap::new(),
    }
  }

  pub fn add_command(&mut self, name: String, command: CommandFn) {
    if self.commands.contains_key(&name) {
      println!("Command already exists: {}", name);
      return;
    }

    self.commands.insert(name, CommandWrapper(command));
  }

  pub fn get_command(&self, name: String) -> Option<&CommandWrapper> {
    self.commands.get(&name)
  }
}
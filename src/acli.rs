use std::fs::File;
use std::io::{BufRead, BufReader, self, Write};

use crate::commands::{CommandManager, CommandFn, self};
use crate::node::NodeData;
use crate::{tree::Tree, node::Node};


#[derive(Clone, Debug)]
pub struct ACli {
  pub tree: Tree,
  pub current_node: Option<Node>,
  pub current_path: String,
  pub current_depth: u64,
  pub command_manager: CommandManager,
}

impl ACli {
  fn read_file(path: String) -> Vec<String> {
    let file = File::open(path).expect("Unable to open file.");
    let reader = BufReader::new(file);

    let mut lines = Vec::new();

    for line in reader.lines() {
      lines.push(line.unwrap());
    }

    lines
  }

  fn parse_file(lines: Vec<String>) -> Vec<Node> {
    let mut nodes = Vec::new();

    for line in lines {
      let mut split: Vec<&str> = line.split("/").collect();

      // Remove the first element if it is a dot.
      if split[0] == "." {
        split.remove(0);
      }

      // Remove the last element if it is empty.
      if split[split.len() - 1] == "" {
        split.remove(split.len() - 1);
      }

      let data = NodeData {
        length: line.len() as u64,
        default_path: line.clone(),
      };

      let depth = split.len() as u64;

      let name = split[(depth - 1) as usize].to_string();
      let path = split.join("/");

      let node = if split[(depth - 1) as usize].contains(".") {
        Node::new_file(data, depth, path, name)
      } else {
        Node::new_directory(data, depth, path, name)
      };

      nodes.push(node);
    }

    nodes
  }

  fn create_tree(nodes: Vec<Node>) -> Tree {
    let mut tree = Tree::new();

    for node in nodes {
      tree.insert(node);
    }

    tree
  }

  pub fn register_command(&mut self, name: String, command: CommandFn) {
    self.command_manager.add_command(name, command);
  }

  fn init_commands(&mut self) {
    self.register_command("ls".to_string(), commands::ls::ls);
    self.register_command("cd".to_string(), commands::cd::cd);
  }

  pub fn init(&mut self) {
    self.init_commands();
  }

  pub fn new_from_path(path: String) -> ACli {
    let lines = ACli::read_file(path);

    let nodes = ACli::parse_file(lines);

    let tree = ACli::create_tree(nodes);

    let current_node = match tree.root.clone() {
      Some(node) => {
        Some(node.clone())
      },
      None => {
        None
      },
    };

    ACli {
      current_node: current_node,
      current_depth: 0,
      current_path: "/".to_string(),
      tree: tree,
      command_manager: CommandManager::new(),
    }
  }

  fn run_command(&mut self, cmd_name: String, args: Vec<&str>) -> Result<(), String> {
    match self.command_manager.get_command(cmd_name.clone()) {
      Some(command) => {
        (command.0)(args, self)
      },
      None => {
        Err(format!("Unknown command : {}", cmd_name))
      },
    }
  }

  pub fn run(&mut self) {
    println!("Welcome to ACli.");

    loop {
      print!("{}", format!("{} > ", self.current_path));
      let _ = io::stdout().flush();

      let mut input = String::new();

      std::io::stdin().read_line(&mut input).expect("Unable to read line.");

      let input = input.trim();

      let split: Vec<&str> = input.split(" ").collect();

      let command = split[0].to_lowercase();

      let args = split[1..].to_vec();

      match self.run_command(command, args) {
        Ok(_) => {},
        Err(e) => {
          println!("{}", e);
        },
      }
    }
  }

  pub fn set_current_node(&mut self, node: Node) {
    self.current_path = format!("/{}", node.path);
    self.current_node = Some(node);
  }

  pub fn get_current_node(&self) -> Option<Node> {
    match &self.current_node {
      Some(node) => {
        Some(node.clone())
      },
      None => {
        None
      },
    }
  }
}
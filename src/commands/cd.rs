use crate::{node::{Node, NodeType}, acli::ACli};

/// Change directory.
/// 
/// # Arguments
/// 
/// * `args` - The arguments passed to the command.
/// 
pub fn cd(args: Vec<&str>, cli: &mut ACli) -> Result<(), String> {
  if args.len() > 1 {
    return Err(format!("Unknown argument : {:?}", args[1]));
  }

  if args.len() == 0 {
    cli.set_current_node(cli.tree.root.clone().unwrap());
    return Ok(())
  }

  let togo = args[0];

  if togo.contains("/") {
    let mut togo_split = togo.split("/");
    let mut togo_vec: Vec<&str> = Vec::new();

    while let Some(togo) = togo_split.next() {
      togo_vec.push(togo);
    }

    for togo in togo_vec {
      match cd(vec![togo], cli) {
        Ok(_) => {},
        Err(e) => {
          return Err(e);
        }
      }
    }

    return Ok(())
  }

  let current_node = match cli.current_node.clone() {
    Some(node) => {
      node
    },
    None => {
      return Err("Unable to find node".to_string());
    },
  };

  if togo == ".." {
    match current_node.parent {
      Some(node) => {
        cli.set_current_node(*node)
      },
      None => {
        // Do nothing (already at root)
      },
    }
    return Ok(())
  }

  let children = current_node.children.clone();
  let mut new_node: Option<Node> = None;

  for node in children {
    if togo == node.name {
      new_node = Some(node.clone());
    }
  }

  match new_node {
    Some(node) => {
      match node.node_type {
        NodeType::Directory() => {
          cli.set_current_node(node);
        },
        NodeType::File() => {
          return Err(format!("'{}' is not a directory", togo))
        }
      }
    },
    None => {
      return Err(format!("The directory '{}' does not exist", togo));
    }
  }

  Ok(())
}
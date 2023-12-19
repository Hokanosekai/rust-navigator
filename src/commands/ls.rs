use crate::acli::ACli;

pub fn ls(args: Vec<&str>, cli: &mut ACli) -> Result<(), String> {
  if args.len() > 1 {
    return Err(format!("Unknown argument : {:?}", args[0]));
  }

  let current_node = match cli.current_node.clone() {
    Some(node) => {
      node
    },
    None => {
      return Err("Unable to find node".to_string());
    },
  };

  let mut children = current_node.children.clone();

  children.sort_by(|a, b| a.name.cmp(&b.name));

  for child in children {
    println!("{}", child.name);
  }

  Ok(())
}
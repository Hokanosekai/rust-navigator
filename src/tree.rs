use crate::node::*;

#[derive(Clone, Debug)]
pub struct Tree {
  pub root: Option<Node>,
}

impl Tree {
  pub fn new() -> Tree {
    Tree {
      root: None,
    }
  }

  pub fn display(&self) {
    match &self.root {
      Some(root) => {
        root.display();
      },
      None => {
        println!("No root node.");
      },
    }
  }

  pub fn insert(&mut self, node: Node) {
    match &mut self.root {
      Some(root) => {
        Tree::insert_recursive(root, node);
      },
      None => {
        self.root = Some(Node::new_root().into());
        self.insert(node);
      },
    }
  }

  pub fn find_by_name(&self, name: String) -> Option<Node> {
    match &self.root {
      Some(root) => {
        Tree::find_by_name_recursive(root, name)
      },
      None => {
        None
      },
    }
  }

  pub fn find_by_name_recursive(node: &Node, name: String) -> Option<Node> {
    if node.name() == name {
      return Some(node.clone());
    }

    for child in &node.children {
      let found = Tree::find_by_name_recursive(child, name.clone());
      match found {
        Some(node) => {
          return Some(node);
        },
        None => {
          continue;
        },
      }
    }

    None
  }

  pub fn insert_recursive(node: &mut Node, child: Node) {
    if child.depth() == node.depth() + 1 {
      let mut child = child;
      child.set_parent(node.clone());
      node.children.push(child);
    } else {
      let child_path = child.path();
      let path = child_path.split("/").collect::<Vec<&str>>();
      let to_find = path[node.depth() as usize];

      for c in &mut node.children {
        if c.name() == to_find {
          Tree::insert_recursive(c, child);
          return;
        }
      }
    }
  }
}
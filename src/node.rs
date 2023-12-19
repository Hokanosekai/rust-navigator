/// Represents node data.
#[derive(Clone, Debug)]
pub struct NodeData {
  /// The default path of the node.
  pub default_path: String,
  /// The length of the path.
  pub length: u64,
}

#[derive(Clone, Debug)]
pub enum NodeType {
    File(),
    Directory(),
}

#[derive(Clone, Debug)]
pub struct Node {
  pub node_type: NodeType,
  pub data: NodeData,
  pub depth: u64,
  pub path: String,
  pub name: String,
  pub parent: Option<Box<Node>>,
  pub children: Vec<Node>,
}

impl From<Node> for Option<Box<Node>> {
    fn from(node: Node) -> Self {
        Some(Box::new(node))
    }
}

impl Node {
  /// Creates a new root node directory.
  /// 
  /// # Returns
  /// 
  /// A new root node directory.
  /// 
  /// # Examples
  /// 
  /// ```
  /// use node::*;
  /// 
  /// let root = Node::new_root();
  /// assert_eq!(root.depth, 0);
  /// assert_eq!(root.path, "/");
  /// assert_eq!(root.name, "root");
  /// ```
  pub fn new_root() -> Node {
    Node {
      node_type: NodeType::Directory(),
      data: NodeData {
        default_path: "/".to_string(),
        length: 1,
      },
      depth: 0,
      path: "".to_string(),
      name: "root".to_string(),
      parent: None,
      children: Vec::new(),
    }
  }

  pub fn new_file(data: NodeData, depth: u64, path: String, name: String) -> Node {
    Node {
      node_type: NodeType::File(),
      data,
      depth,
      path,
      name,
      parent: None,
      children: Vec::new(),
    }
  }

  pub fn new_directory(data: NodeData, depth: u64, path: String, name: String) -> Node {
    Node {
      node_type: NodeType::Directory(),
      data,
      depth,
      path,
      name,
      parent: None,
      children: Vec::new(),
    }
  }

  /// Gets the depth of the node.
  /// 
  /// # Returns
  /// 
  /// The depth of the node.
  /// 
  /// # Examples
  /// 
  /// ```
  /// use node::*;
  /// 
  /// let root = Node::new_root();
  /// assert_eq!(root.depth(), 0);
  /// ```
  pub fn depth(&self) -> u64 {
    self.depth
  }

  /// Displays the node and its children.
  /// 
  /// # Example
  /// 
  /// ```
  /// use node::*;
  /// 
  /// let root = Node::new_root();
  /// root.display();
  /// ```
  /// 
  /// # Output
  /// 
  /// ```
  /// Directory: /
  ///  Children: 0
  /// ```
  /// 
  /// # Example
  /// 
  /// ```
  /// use node::*;
  /// 
  /// let root = Node::new_root();
  /// let mut src = Node::Directory(NodeDirectory::new(
  ///  NodeData {
  ///   default_path: "src/".to_string(),
  ///   length: 4
  ///  },
  ///  1,
  ///  "src/".to_string(),
  ///  "src".to_string()
  /// ));
  /// 
  /// src.add_child(Node::File(NodeFile::new(
  ///  NodeData {
  ///   default_path: "src/main.rs".to_string(),
  ///   length: 11
  ///  },
  ///  2,
  ///  "main.rs".to_string(),
  ///  "main.rs".to_string()
  /// )));
  /// 
  /// root.insert(src);
  /// root.display();
  /// ```
  /// 
  /// # Output
  /// 
  /// ```
  /// Directory: /
  ///  Children: 1
  ///  Directory: src/
  ///   Children: 1
  ///   File: main.rs
  /// ```
  pub fn display(&self) {
    let ds = "  ".repeat(self.depth as usize);
    match self.node_type {
      NodeType::File() => {
        println!("{}File: {}", ds, self.name);
        println!("{} Path: {}", ds, self.path);
        println!("{} Depth: {}", ds, self.depth);
        /*if !self.parent.is_none() {
          println!("{} Parent: {:?}", ds, self.parent);
        }*/
      },
      NodeType::Directory() => {
        println!("{}Directory: {}", ds, self.name);
        println!("{} Children: {}", ds, self.children.len());
        println!("{} Path: {}", ds, self.path);
        println!("{} Depth: {}", ds, self.depth);
        /*if !self.parent.is_none() {
          println!("{} Parent: {:?}", ds, self.parent);
        }*/

        for child in &self.children {
          child.display();
        }
      },
    }
  }

  /// Gets the name of the node.
  /// 
  /// # Returns
  /// 
  /// The name of the node.
  /// 
  /// # Examples
  /// 
  /// ```
  /// use node::*;
  /// 
  /// let root = Node::new_root();
  /// assert_eq!(root.name(), "root");
  /// ```
  pub fn name(&self) -> String {
    self.name.clone()
  }

  pub fn path(&self) -> String {
    self.path.clone()
  }

  pub fn set_parent(&mut self, parent: Node) {
    self.parent = Some(Box::new(parent));
  }
}

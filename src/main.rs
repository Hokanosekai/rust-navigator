pub mod node;
pub mod tree;
pub mod acli;
pub mod commands;

use acli::*;

fn main() {
    let mut acli = ACli::new_from_path("paths.txt".to_string());

    acli.init();
    acli.run();
}

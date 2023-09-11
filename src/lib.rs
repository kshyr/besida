pub mod dialogue_node;
pub mod parser;

use crate::dialogue_node::DialogueNode;
use crate::parser::parse;
use std::path::Path;

#[derive(Debug)]
pub struct Besida {
    pub name: String,
    dialogue_nodes: Vec<DialogueNode>,

    curr_node_idx: usize,
}

impl Besida {
    pub fn new(dialogue_file_path: &Path) -> Self {
        let (name, dialogue_nodes) = parse(dialogue_file_path);
        Self {
            name,
            dialogue_nodes,
            curr_node_idx: 0,
        }
    }

    pub fn next_node(&mut self) {
        self.curr_node_idx += 1;
    }

    pub fn prev_node(&mut self) {
        self.curr_node_idx -= 1;
    }

    pub fn set_node_index(&mut self, index: usize) {
        self.curr_node_idx = index;
    }

    pub fn get_node(&self) -> Option<&DialogueNode> {
        self.dialogue_nodes.get(self.curr_node_idx)
    }

    pub fn get_node_mut(&mut self) -> Option<&mut DialogueNode> {
        self.dialogue_nodes.get_mut(self.curr_node_idx)
    }
}

#[cfg(test)]
mod tests {
    use crate::Besida;
    use std::path::Path;

    #[test]
    fn mut_node() {
        let dialogue_path = Path::new("examples/recursion.besida");
        let mut besida = Besida::new(dialogue_path);
        println!("{:?}", besida.dialogue_nodes);
        let node = besida.get_node_mut();
        node.unwrap().next_event();
    }
}

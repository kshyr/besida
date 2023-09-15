use std::path::Path;

use crate::branch::Branch;
use crate::dialogue_node::DialogueNode;
use crate::parser::parse;

#[derive(Debug)]
pub struct Besida {
    branches: Vec<Branch>,
    pub(crate) dialogue_nodes: Vec<DialogueNode>,

    curr_node_idx: usize,
}

impl Besida {
    pub fn new(dialogue_file_path: &Path) -> Self {
        let branches = parse(dialogue_file_path);
        Self {
            branches,
            dialogue_nodes: vec![],
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
    use std::path::Path;

    use crate::Besida;

    #[test]
    fn mut_node() {
        let dialogue_path = Path::new("examples/basic.besida");
        let mut besida = Besida::new(dialogue_path);
        println!("Branches Count: {:?}", besida.branches.iter().count());
        besida.branches.iter().for_each(|branch| {
            println!("--- {:?} ---", branch.name);
            branch.nodes.iter().for_each(|node| {
                println!("{}: {}", node.speaker, node.speech);
            })
        });
        //let node = besida.get_node_mut();
        //node.unwrap().next_event();
    }
}

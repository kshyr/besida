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

        // consider first branch an entry, if there are no entry tags
        let entry_branches: Vec<Branch> =
            branches.iter().filter(|br| br.is_entry).cloned().collect();

        let mut entry_branch: Branch;

        if entry_branches.len() > 1 {
            panic!("Multiple entry points found. Remove \"!\" from other branch definitions.");
        } else if entry_branches.len() == 0 {
            entry_branch = branches.first().expect("No branches were found.").clone();
        } else {
            entry_branch = entry_branches.first().cloned().unwrap();
        }

        let dialogue_nodes = entry_branch.nodes.clone();

        Self {
            branches,
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
    use std::path::Path;

    use crate::Besida;

    fn display_dialogue(besida: &Besida) {
        println!("\n");
        besida.branches.iter().for_each(|branch| {
            println!("--- {:?} ---", branch.name);
            branch.nodes.iter().for_each(|node| {
                println!("{}: {}", node.speaker, node.speech);
            })
        });
        println!("\n");
    }

    #[test]
    fn mut_node() {
        let dialogue_path = Path::new("examples/basic.besida");
        let besida = Besida::new(dialogue_path);
        println!("Branches Count: {:?}", besida.branches.iter().count());
        display_dialogue(&besida);
        //let node = besida.get_node_mut();
        //node.unwrap().next_event();
    }
}

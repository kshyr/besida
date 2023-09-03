use crate::dialogue_node::DialogueNode;
use crate::parser::parse;
use std::path::Path;

#[derive(Debug)]
pub struct Besida {
    pub name: String,
    pub dialogue_nodes: Vec<DialogueNode>,

    pub curr_node_idx: usize,
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

    pub fn has_next_node(&self) -> bool {
        self.dialogue_nodes.get(self.curr_node_idx + 1).is_some()
    }

    pub fn next_node(&mut self) {
        if !self.has_next_node() {
            panic!("Out of bounds: there are no nodes left to read from.")
        }

        self.curr_node_idx += 1;
    }

    pub fn get_node(&mut self) -> &mut DialogueNode {
        &mut self.dialogue_nodes[self.curr_node_idx]
    }
}

#[cfg(test)]
mod tests {
    use crate::besida::Besida;
    use std::path::Path;

    #[test]
    fn besida_init() {
        let dialogue_path = Path::new("examples/basic.besida");
        Besida::new(dialogue_path);
    }
}

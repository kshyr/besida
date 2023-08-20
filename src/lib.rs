mod parse;
use parse::parse;
use std::path::Path;

pub struct Besida {
    name: String,
    pub dialogue_nodes: Vec<DialogueNode>,
}

#[derive(Debug)]
pub struct DialogueNode {
    speaker: String,
    speech: String,
}

impl Besida {
    pub fn new(dialogue_file_path: &Path) -> Self {
        let (name, dialogue_nodes) = parse(dialogue_file_path);
        Self {
            name,
            dialogue_nodes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let dialogue_path = Path::new("examples/basic.bsd");
        let besida = Besida::new(dialogue_path);
        println!("{:?}", besida.dialogue_nodes);
    }
}

use std::{fs, path::Path};

pub struct Besida {
    pub dialogue_nodes: Vec<DialogueNode>,
}

#[derive(Debug)]
pub struct DialogueNode {
    name: String,
    speech: String,
}

fn parse(file_path: &Path) -> Vec<DialogueNode> {
    let contents = fs::read_to_string(file_path).expect("should've read the file");
    println!("{contents}");
    vec![DialogueNode {
        name: "You".into(),
        speech: "First line".into(),
    }]
}

impl Besida {
    pub fn new(dialogue_file_path: &Path) -> Self {
        Self {
            dialogue_nodes: parse(dialogue_file_path),
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

use std::{fs, path::Path};

use crate::DialogueNode;

pub fn parse(file_path: &Path) -> (String, Vec<DialogueNode>) {
    let contents = fs::read_to_string(file_path).expect("should've read the file");
    println!("{contents}");
    (
        "Intro".into(),
        vec![DialogueNode {
            speaker: "You".into(),
            speech: "First line".into(),
        }],
    )
}

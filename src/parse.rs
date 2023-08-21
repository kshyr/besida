use crate::DialogueNode;
use nom::{bytes::complete::tag, character::complete::multispace0, IResult};
use std::{fs, path::Path};

pub fn parse(file_path: &Path) -> (String, Vec<DialogueNode>) {
    let contents = fs::read_to_string(file_path).expect("should've read the file");
    let name = "Intro dialogue".into();
    let dialogue_nodes = vec![DialogueNode {
        speaker: "You".into(),
        speech: "First line".into(),
    }];

    (name, dialogue_nodes)
}

fn dialogue_name(input: &str) -> IResult<&str, &str> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("#")(input)?;
    let (input, c) = multispace0(input)?;
    Ok((input, c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dialogue_name_parsing() {
        let name = dialogue_name("# Intro Dialogue     ")
            .unwrap()
            .0
            .trim_end()
            .to_lowercase()
            .replace(' ', "_");

        assert_eq!("intro_dialogue", name);
    }
}

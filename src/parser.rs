use crate::dialogue_node::{DialogueNode, Event, EventType};

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::multispace0,
    IResult,
};
use std::{fs, path::Path};

pub fn parse(file_path: &Path) -> (String, Vec<DialogueNode>) {
    let contents = fs::read_to_string(file_path).expect("should've read the file");
    let name = dialogue_name(contents.as_str()).unwrap().1.into();
    let dialogue_nodes = vec![DialogueNode {
        speaker: "You".into(),
        events: vec![Event {
            event_type: EventType::PrintChar,
            char: Some('c'),
        }],
        curr_event_idx: 0,
    }];

    (name, dialogue_nodes)
}

fn dialogue_name(input: &str) -> IResult<&str, &str> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("--- ")(input)?;
    let (input, c) = take_until(" ---")(input)?;
    let (input, _) = tag(" ---")(input)?;
    println!("{}", c);
    Ok((input, c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dialogue_name_parsing() {
        let name = dialogue_name("--- Intro Dialogue ---")
            .unwrap()
            .1
            .trim_end()
            .to_lowercase()
            .replace(' ', "_");

        assert_eq!("intro_dialogue", name);
    }
}

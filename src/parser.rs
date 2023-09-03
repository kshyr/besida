use crate::dialogue_node::{DialogueNode, Event};

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{multispace0, newline},
    multi::{many0, many1},
    IResult,
};
use std::{fs, path::Path};

pub fn parse(file_path: &Path) -> (String, Vec<DialogueNode>) {
    let contents = fs::read_to_string(file_path).expect("should've read the file");
    let input = contents.as_str();

    let (input, name) = dialogue_name(input).unwrap();
    let (_, nodes) = dialogue_nodes(input).unwrap();

    (name.into(), nodes)
}

fn dialogue_name(input: &str) -> IResult<&str, &str> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("--- ")(input)?;
    let (input, c) = take_until(" ---")(input)?;
    let (input, _) = tag(" ---")(input)?;
    Ok((input, c))
}

fn dialogue_nodes(input: &str) -> IResult<&str, Vec<DialogueNode>> {
    let (input, _) = many1(newline)(input)?;
    let (input, nodes) = many1(dialogue_node)(input)?;
    Ok((input, nodes))
}

fn dialogue_node(input: &str) -> IResult<&str, DialogueNode> {
    let (input, speaker) = take_until(":")(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, text) = take_until("\n\n")(input)?;
    let (input, _) = many0(newline)(input)?;

    let events = parse_text_to_events(text);
    let node = DialogueNode {
        speaker: speaker.into(),
        speech: text.into(),
        events,
        curr_event_idx: 0,
    };

    Ok((input, node))
}

fn parse_text_to_events(text: &str) -> Vec<Event> {
    let mut events = Vec::new();
    let mut action_buffer = String::new();
    let mut is_in_action = false;

    for c in text.chars() {
        if c == '[' {
            is_in_action = true;
            action_buffer.clear();
        } else if c == ']' && is_in_action {
            is_in_action = false;
            events.push(Event::Action(action_buffer.clone()));
        } else if is_in_action {
            action_buffer.push(c);
        } else if c == '_' {
            events.push(Event::Pause);
        } else {
            events.push(Event::PrintChar(c));
        }
    }

    events
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dialogue_name_parsing() {
        let name = dialogue_name("--- Intro dialogue ---").unwrap().1;

        assert_eq!("Intro dialogue", name);
    }
}

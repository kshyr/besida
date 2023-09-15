use std::{fs, path::Path};

use nom::branch::alt;
use nom::bytes::complete::take_till;
use nom::character::complete::{char, one_of};
use nom::combinator::{not, opt};
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{multispace0, newline},
    multi::many0,
    IResult,
};

use crate::branch::Branch;
use crate::dialogue_node::DialogueNode;
use crate::event::Event;

pub fn parse(file_path: &Path) -> Vec<Branch> {
    let contents = fs::read_to_string(file_path).expect("should've read the file");
    let input = contents.as_str();

    let (_, branches) = branches(input).unwrap();

    branches
}

fn branches(input: &str) -> IResult<&str, Vec<Branch>> {
    let (input, branches) = many0(branch)(input)?;

    Ok((input, branches))
}

fn branch(input: &str) -> IResult<&str, Branch> {
    let (input, is_entry) = entry_branch_tag(input)?;
    let (input, name) = branch_name(input)?;
    let (input, nodes) = dialogue_nodes(input)?;
    let (input, _) = multispace0(input)?;

    let branch = Branch {
        name,
        is_entry,
        nodes,
    };

    Ok((input, branch))
}

fn entry_branch_tag(input: &str) -> IResult<&str, bool> {
    let (input, is_entry_option) = opt(tag("!"))(input)?;

    let is_entry = is_entry_option.is_some();
    Ok((input, is_entry))
}

fn branch_name(input: &str) -> IResult<&str, String> {
    let (input, _) = tag("--- ")(input)?;
    let (input, name) = take_until(" ---")(input)?;
    let (input, _) = tag(" ---")(input)?;

    Ok((input, name.to_string()))
}

fn dialogue_nodes(input: &str) -> IResult<&str, Vec<DialogueNode>> {
    let (input, nodes) = many0(dialogue_node)(input)?;

    Ok((input, nodes))
}

fn dialogue_node(input: &str) -> IResult<&str, DialogueNode> {
    let (input, _) = multispace0(input)?;
    let (input, _) = not(char('-'))(input)?;
    let (input, speaker) = take_until(":")(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, events) = events(input)?;
    let (input, _) = multispace0(input)?;

    let speech: String = events
        .iter()
        .filter_map(|event| match event {
            Event::PrintChar(c) => Some(c),
            _ => None,
        })
        .collect();

    let node = DialogueNode {
        speaker: speaker.to_string(),
        speech,
        events,
        curr_event_idx: 0,
        jump_dest: None,
    };

    Ok((input, node))
}

fn events(input: &str) -> IResult<&str, Vec<Event>> {
    let (input, _) = multispace0(input)?;
    let (input, event_list) = many0(event)(input)?;

    Ok((input, event_list))
}

fn event(input: &str) -> IResult<&str, Event> {
    let (input, _) = opt(newline)(input)?;
    let (input, event) = alt((action_event, pause_event, print_char_event))(input)?;
    Ok((input, event))
}

fn print_char_event(input: &str) -> IResult<&str, Event> {
    let (input, char) =
        one_of("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz.!'\" ")(input)?;
    let event = Event::PrintChar(char);
    Ok((input, event))
}

fn pause_event(input: &str) -> IResult<&str, Event> {
    let (input, _) = tag("_")(input)?;
    let (input, pause_string) = take_till(|c| c != '_')(input)?;

    let pause_length = pause_string.chars().count() + 1;
    let event = Event::Pause(pause_length);
    Ok((input, event))
}

fn action_event(input: &str) -> IResult<&str, Event> {
    let (input, _) = tag("[")(input)?;
    let (input, action_name) = take_until("]")(input)?;
    let (input, _) = tag("]")(input)?;

    let event = Event::Action(action_name.to_string());
    Ok((input, event))
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use crate::event::Event;
    use crate::event::Event::PrintChar;

    use super::*;

    fn nom_parsing_test<F, E>(parser_fn: F, input: &str, expected_output: E)
    where
        F: Fn(&str) -> IResult<&str, E>,
        E: PartialEq + Debug,
    {
        let (input, output) = parser_fn(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(output, expected_output);
    }

    #[test]
    fn entry_branch_tag_parsing() {
        let input = "!";
        let expected = true;
        nom_parsing_test(entry_branch_tag, input, expected);
    }

    #[test]
    fn branch_name_parsing() {
        let input = "--- Name ---";
        let expected = "Name".to_string();
        nom_parsing_test(branch_name, input, expected);
    }

    #[test]
    fn print_char_event_parsing() {
        let input = ".";
        let expected = Event::PrintChar('.');
        nom_parsing_test(print_char_event, input, expected);
    }

    #[test]
    fn pause_event_parsing() {
        let input = "___";
        let expected = Event::Pause(3);
        nom_parsing_test(pause_event, input, expected);
    }

    #[test]
    fn action_event_parsing() {
        let input = "[action_name]";
        let expected = Event::Action("action_name".to_string());
        nom_parsing_test(action_event, input, expected);
    }

    #[test]
    fn events_parsing() {
        let input = "Oh...___[surprised]";
        let expected = vec![
            Event::PrintChar('O'),
            Event::PrintChar('h'),
            Event::PrintChar('.'),
            Event::PrintChar('.'),
            Event::PrintChar('.'),
            Event::Pause(3),
            Event::Action("surprised".to_string()),
        ];
        nom_parsing_test(events, input, expected);
    }

    #[test]
    fn nodes_parsing() {
        let input = "You:\nHey.\n\nMan:\nHey.";
        let expected = vec![
            DialogueNode {
                speaker: "You".to_string(),
                speech: "Hey.".to_string(),
                events: vec![
                    PrintChar('H'),
                    PrintChar('e'),
                    PrintChar('y'),
                    PrintChar('.'),
                ],
                curr_event_idx: 0,
                jump_dest: None,
            },
            DialogueNode {
                speaker: "Man".to_string(),
                speech: "Hey.".to_string(),
                events: vec![
                    PrintChar('H'),
                    PrintChar('e'),
                    PrintChar('y'),
                    PrintChar('.'),
                ],
                curr_event_idx: 0,
                jump_dest: None,
            },
        ];
        nom_parsing_test(dialogue_nodes, input, expected);
    }

    #[test]
    fn branch_parsing() {}
}

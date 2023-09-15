use std::{fs, path::Path};

use nom::branch::alt;
use nom::bytes::complete::take_till;
use nom::character::complete::{anychar, line_ending, one_of};
use nom::combinator::opt;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{multispace0, newline},
    multi::{many0, many1},
    IResult,
};

use crate::dialogue_node::DialogueNode;
use crate::event::Event;

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
    let (input, nodes) = many0(dialogue_node)(input)?;

    Ok((input, nodes))
}

fn dialogue_node(input: &str) -> IResult<&str, DialogueNode> {
    let (input, speaker) = take_until(":")(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, text) = take_until("\n\n")(input)?;
    let (input, _) = many0(newline)(input)?;

    let (input, events) = events(input)?;

    let node = DialogueNode {
        speaker: speaker.into(),
        speech: text.into(),
        events,
        curr_event_idx: 0,
        jump_dest: None,
    };

    Ok((input, node))
}

fn events(input: &str) -> IResult<&str, Vec<Event>> {
    let (input, _) = multispace0(input)?;
    let (input, event_list) = many0(event)(input)?;
    let (input, _) = opt(line_ending)(input)?;

    Ok((input, event_list))
}

fn event(input: &str) -> IResult<&str, Event> {
    let (input, event) = alt((action_event, pause_event, print_char_event))(input)?;
    Ok((input, event))
}

fn print_char_event(input: &str) -> IResult<&str, Event> {
    let (input, char) = alt((anychar, one_of(".!")))(input)?;
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
    use crate::event::Event;

    use super::*;

    #[test]
    fn dialogue_name_parsing() {
        let name = dialogue_name("--- Intro dialogue ---").unwrap().1;

        assert_eq!("Intro dialogue", name);
    }

    #[test]
    fn print_char_event_parsing() {
        let input = ".";
        let (input, char_event) = print_char_event(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(char_event, Event::PrintChar('.'));
    }

    #[test]
    fn pause_event_parsing() {
        let input = "___!";
        let (input, pause_event) = pause_event(input).unwrap();
        assert_eq!(input, "!");
        assert_eq!(pause_event, Event::Pause(3));
    }

    #[test]
    fn action_event_parsing() {
        let input = "[action_name]";
        let (input, action_event) = action_event(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(action_event, Event::Action("action_name".to_string()));
    }

    #[test]
    fn events_parsing() {
        let input = "Oh...___[surprised]";
        let (_, events) = events(input).unwrap();
        assert_eq!(
            events,
            vec![
                Event::PrintChar('O'),
                Event::PrintChar('h'),
                Event::PrintChar('.'),
                Event::PrintChar('.'),
                Event::PrintChar('.'),
                Event::Pause(3),
                Event::Action("surprised".to_string())
            ]
        );
    }

    #[test]
    fn branch_parsing() {}
}

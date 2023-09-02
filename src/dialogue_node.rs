#[derive(Debug)]
pub enum EventType {
    PrintChar,
    SpeedChange,
    EmotionChange,
}

#[derive(Debug)]
pub struct Event {
    pub event_type: EventType,
    pub char: Option<char>,
}

#[derive(Debug)]
pub struct DialogueNode {
    pub speaker: String,
    pub events: Vec<Event>,

    pub curr_event_idx: usize,
}

impl DialogueNode {
    pub fn has_next_event(&self) -> bool {
        self.events.get(self.curr_event_idx + 1).is_some()
    }

    pub fn next_event(&mut self) {
        if !self.has_next_event() {
            panic!("Out of bounds: there are no events left in current node.")
        }

        self.curr_event_idx += 1;
    }

    pub fn get_event(&self) -> &Event {
        &self.events[self.curr_event_idx]
    }
}

impl Default for DialogueNode {
    fn default() -> Self {
        DialogueNode {
            speaker: "You".into(),
            events: vec![
                Event {
                    event_type: EventType::PrintChar,
                    char: Some('H'),
                },
                Event {
                    event_type: EventType::PrintChar,
                    char: Some('e'),
                },
                Event {
                    event_type: EventType::PrintChar,
                    char: Some('l'),
                },
                Event {
                    event_type: EventType::PrintChar,
                    char: Some('l'),
                },
                Event {
                    event_type: EventType::PrintChar,
                    char: Some('o'),
                },
            ],
            curr_event_idx: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_curr_and_next_nodes() {
        let mut dialogue_node = DialogueNode::default();

        assert_eq!(dialogue_node.curr_event_idx, 0);
        let event = dialogue_node.get_event();
        assert_eq!(event.char, Some('H'));

        dialogue_node.next_event();

        assert_eq!(dialogue_node.curr_event_idx, 1);
        let event = dialogue_node.get_event();
        assert_eq!(event.char, Some('e'));
    }
}

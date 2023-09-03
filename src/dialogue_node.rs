#[derive(Debug)]
pub enum Event {
    PrintChar(char),
    Pause,
    SpeedChange(f32),
    EmotionChange(String),
    Action(String),
}

#[derive(Debug)]
pub struct DialogueNode {
    pub speaker: String,
    pub speech: String,
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

    pub fn get_event(&mut self) -> &mut Event {
        &mut self.events[self.curr_event_idx]
    }
}

impl Default for DialogueNode {
    fn default() -> Self {
        DialogueNode {
            speaker: "You".into(),
            speech: "Hello".into(),
            events: vec![
                Event::PrintChar('H'),
                Event::PrintChar('e'),
                Event::PrintChar('l'),
                Event::PrintChar('l'),
                Event::PrintChar('o'),
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
        let char = match event {
            Event::PrintChar(c) => *c,
            _ => 'c',
        };
        assert_eq!(char, 'H');

        dialogue_node.next_event();

        assert_eq!(dialogue_node.curr_event_idx, 1);
        let event = dialogue_node.get_event();
        let char = match event {
            Event::PrintChar(c) => *c,
            _ => 'c',
        };
        assert_eq!(char, 'e');
    }
}

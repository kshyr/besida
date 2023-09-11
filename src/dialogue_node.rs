#[derive(Debug, PartialEq)]
pub enum Event {
    PrintChar(char),
    Pause(usize),
    Action(String),
    Choice(ChoiceOption),
}

#[derive(Debug, PartialEq)]
pub struct ChoiceOption {
    pub text: String,
    pub action: Option<String>,
    pub(super) jump_dest: Option<String>,
}

#[derive(Debug)]
pub struct DialogueNode {
    pub speaker: String,
    pub speech: String,

    pub(super) events: Vec<Event>,
    pub(super) curr_event_idx: usize,
    pub(super) jump_dest: Option<String>,
}

impl DialogueNode {
    pub fn next_event(&mut self) {
        self.curr_event_idx += 1;
    }

    pub fn prev_event(&mut self) {
        self.curr_event_idx -= 1;
    }

    pub fn set_event_index(&mut self, index: usize) {
        self.curr_event_idx = index;
    }

    pub fn get_event(&self) -> Option<&Event> {
        self.events.get(self.curr_event_idx)
    }

    pub fn get_event_mut(&mut self) -> Option<&mut Event> {
        self.events.get_mut(self.curr_event_idx)
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
            jump_dest: None,
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
        if let Some(event) = dialogue_node.get_event() {
            let char = match event {
                Event::PrintChar(c) => *c,
                _ => todo!(),
            };
            assert_eq!(char, 'H');
        }
        dialogue_node.next_event();

        assert_eq!(dialogue_node.curr_event_idx, 1);
        if let Some(event) = dialogue_node.get_event() {
            let char = match event {
                Event::PrintChar(c) => *c,
                _ => todo!(),
            };
            assert_eq!(char, 'e');
        }
    }
}

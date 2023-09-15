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

use crate::dialogue_node::DialogueNode;

#[derive(Debug)]
pub struct Branch {
    pub(crate) name: String,
    pub(crate) is_entry: bool,
    pub(crate) nodes: Vec<DialogueNode>,
}

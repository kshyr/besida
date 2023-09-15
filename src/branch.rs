use crate::dialogue_node::DialogueNode;

#[derive(Debug, Clone)]
pub struct Branch {
    pub(crate) name: String,
    pub(crate) is_entry: bool,
    pub(crate) nodes: Vec<DialogueNode>,
}

use VNode;

/// The representation of string in virtual dom tree.
#[derive(Debug)]
pub struct VText {
    /// The content of a text string
    pub content: String,
    /// Whether the content is a comment
    pub is_comment: bool,
}

impl From<VText> for VNode {
    fn from(text: VText) -> VNode {
        VNode::Text(text)
    }
}

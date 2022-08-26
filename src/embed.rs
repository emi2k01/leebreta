use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub(crate) struct Embed {
    pub file: String,
    pub kind: EmbedKind,
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) enum EmbedKind {
    Snippet { from: EmbedPoint, to: EmbedPoint },
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) enum EmbedPoint {
    Line(i32),
}

#[derive(Serialize, Deserialize)]
pub(crate) struct EmbedProcessed {
    pub file: String,
    pub before: String,
    pub after: String,
    pub content: String,
    pub kind: EmbedKind,
}

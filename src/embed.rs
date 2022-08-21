use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub(crate) struct Embed {
    pub file: String,
    pub kind: EmbedKind,
}

#[derive(Deserialize)]
pub(crate) enum EmbedKind {
    Snippet(EmbedSnippet),
}

#[derive(Deserialize)]
pub(crate) struct EmbedSnippet {
    pub from: EmbedPoint,
    pub to: EmbedPoint,
}

#[derive(Deserialize)]
pub(crate) enum EmbedPoint {
    Line(i32),
}

#[derive(Serialize, Deserialize)]
pub(crate) struct EmbedProcessed {
    pub file: String,
    pub before: String,
    pub content: String,
    pub after: String,
}

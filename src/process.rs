use anyhow::Context;
use serde::Serialize;

use crate::embed::{Embed, EmbedKind, EmbedPoint, EmbedProcessed};

struct LeebretaLanguageMapper {
    on_codeblock: bool,
}

impl LeebretaLanguageMapper {
    fn new() -> Self {
        Self {
            on_codeblock: false,
        }
    }

    fn map<'a>(&mut self, event: md::Event<'a>) -> md::Event<'a> {
        match &event {
            original_tag
            @ md::Event::Start(md::Tag::CodeBlock(md::CodeBlockKind::Fenced(lang))) => {
                if lang.to_string() == "leebreta" {
                    self.on_codeblock = true;
                    md::Event::Start(md::Tag::CodeBlock(md::CodeBlockKind::Fenced(
                        md::CowStr::Inlined(md::InlineStr::try_from("leebreta_json").unwrap()),
                    )))
                } else {
                    original_tag.clone()
                }
            },
            original_tag @ md::Event::End(md::Tag::CodeBlock(md::CodeBlockKind::Fenced(lang))) => {
                if lang.to_string() == "leebreta" {
                    self.on_codeblock = false;
                    md::Event::End(md::Tag::CodeBlock(md::CodeBlockKind::Fenced(
                        md::CowStr::Inlined(md::InlineStr::try_from("leebreta_json").unwrap()),
                    )))
                } else {
                    original_tag.clone()
                }
            },
            md::Event::Code(code) if self.on_codeblock => {
                let embed: Embed = ron::from_str(code).unwrap();
                match &embed.kind {
                    EmbedKind::Snippet(snippet) => {
                        let file_contents = std::fs::read_to_string(&embed.file)
                            .with_context(|| format!("path: {}", embed.file))
                            .expect("");

                        let start_offset = match snippet.from {
                            EmbedPoint::Line(line) => {
                                file_contents
                                    .lines()
                                    .take((line - 1) as usize)
                                    .flat_map(|line| line.bytes())
                                    .count()
                            },
                        };

                        let end_offset = match snippet.to {
                            EmbedPoint::Line(line) => {
                                file_contents
                                    .lines()
                                    .take((line - 1) as usize)
                                    .flat_map(|line| line.bytes())
                                    .count()
                            },
                        };

                        let processed = EmbedProcessed {
                            file: embed.file.clone(),
                            before: String::new(),
                            content: file_contents[start_offset..end_offset].to_string(),
                            after: String::new(),
                        };
                        let processed_json = serde_json::to_string_pretty(&processed).unwrap();

                        md::Event::Code(processed_json.into())
                    },
                }
            },
            event => event.clone(),
        }
    }
}

fn process_file(file: &std::path::Path) -> anyhow::Result<String> {
    let file_contents = std::fs::read_to_string(file)?;

    let md_parser = md::Parser::new_ext(&file_contents, md::Options::all());

    let mut leebreta_mapper = LeebretaLanguageMapper::new();
    let md_parser = md_parser.map(|e| leebreta_mapper.map(e));

    let mut html = String::new();
    md::html::push_html(&mut html, md_parser);

    Ok(html)
}

use eyre::Context;
use serde::Serialize;

use crate::embed::{Embed, EmbedKind, EmbedPoint, EmbedProcessed};

enum Section<'a> {
    Events(Vec<md::Event<'a>>),
    Embed(EmbedProcessed),
}

#[derive(Serialize)]
enum OutputKind {
    Html(String),
    Embed(EmbedProcessed),
}

struct LeebretaLanguageMapper<'a> {
    on_codeblock: bool,
    sections: Vec<Section<'a>>,
    events: Vec<md::Event<'a>>,
}

impl<'a> LeebretaLanguageMapper<'a> {
    fn new() -> Self {
        Self {
            on_codeblock: false,
            sections: vec![],
            events: vec![],
        }
    }

    fn for_each(&mut self, event: md::Event<'a>) {
        match &event {
            original_tag
            @ md::Event::Start(md::Tag::CodeBlock(md::CodeBlockKind::Fenced(lang))) => {
                if lang.to_string() == "leebreta" {
                    self.sections
                        .push(Section::Events(std::mem::take(&mut self.events)));
                    self.on_codeblock = true;
                } else {
                    self.events.push(original_tag.clone());
                }
            },
            original_tag @ md::Event::End(md::Tag::CodeBlock(md::CodeBlockKind::Fenced(lang))) => {
                if lang.to_string() == "leebreta" {
                    self.on_codeblock = false;
                } else {
                    self.events.push(original_tag.clone());
                }
            },
            md::Event::Text(code) if self.on_codeblock => {
                let embed: Embed = ron::from_str(code).unwrap();
                match &embed.kind {
                    embed_kind @ EmbedKind::Snippet { from, to } => {
                        let file_contents = std::fs::read_to_string(&embed.file)
                            .with_context(|| format!("path: {}", embed.file))
                            .expect("");

                        let start_offset = match from {
                            EmbedPoint::Line(line) => {
                                file_contents
                                    .lines()
                                    .take((line - 1) as usize)
                                    .flat_map(|line| line.bytes())
                                    .count()
                            },
                        };

                        let end_offset = match to {
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
                            after: String::new(),
                            content: file_contents[start_offset..end_offset].to_string(),
                            kind: embed_kind.clone(),
                        };
                        let processed_json = serde_json::to_string_pretty(&processed).unwrap();

                        self.sections.push(Section::Embed(processed))
                    },
                };
            },
            event => self.events.push(event.clone()),
        };
    }

    fn finish(&mut self) {
        self.sections
            .push(Section::Events(std::mem::take(&mut self.events)));
    }
}

fn process_file(file: &std::path::Path) -> eyre::Result<String> {
    let file_contents = std::fs::read_to_string(file)?;
    process_str(&file_contents)
}

fn process_str(contents: &str) -> eyre::Result<String> {
    let md_parser = md::Parser::new_ext(&contents, md::Options::all());

    let mut leebreta_mapper = LeebretaLanguageMapper::new();
    md_parser.for_each(|e| leebreta_mapper.for_each(e));
    leebreta_mapper.finish();

    let output: Vec<_> = leebreta_mapper
        .sections
        .into_iter()
        .map(|s| {
            match s {
                Section::Events(events) => {
                    let mut buf = String::new();
                    md::html::push_html(&mut buf, events.into_iter());
                    OutputKind::Html(buf)
                },
                Section::Embed(embed) => OutputKind::Embed(embed),
            }
        })
        .collect();

    Ok(serde_json::to_string_pretty(&output)?)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_basic_snippet() -> eyre::Result<()> {
        color_eyre::install()?;
        k9::snapshot!(
            process_str(&indoc! {"
            # Hello, my friends

            **This is a test**

            ```leebreta
            Embed(
                file: \"src/main.rs\",
                kind: Snippet(
                    from: Line(1),
                    to: Line(2),
                )
            )
            ```

            ~ wtf ~
        "})?,
            r##"
[
  {
    "Html": "<h1>Hello, my friends</h1>\
<p><strong>This is a test</strong></p>\
"
  },
  {
    "Embed": {
      "file": "src/main.rs",
      "before": "",
      "after": "",
      "content": "#![allow(unused)]",
      "kind": {
        "Snippet": {
          "from": {
            "Line": 1
          },
          "to": {
            "Line": 2
          }
        }
      }
    }
  },
  {
    "Html": "<p>~ wtf ~</p>\
"
  }
]
"##
        );

        Ok(())
    }
}

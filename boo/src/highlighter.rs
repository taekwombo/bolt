mod config;
mod chunks;

use tree_sitter_highlight::{Highlighter, HighlightConfiguration, Highlight, HighlightEvent};
use config::HighlightColors;
use chunks::{Chunks, Chunk};

pub struct QueryHighlighter {
    colors: HighlightColors,
    highlighter: Highlighter,
    highlighter_configuration: HighlightConfiguration,
}

impl QueryHighlighter {
    pub fn new() -> Self {
        use tree_sitter_cypher::{language, HIGHLIGHTS_QUERY};

        let colors = HighlightColors::new();
        let highlighter = Highlighter::new();
        let mut highlighter_configuration = HighlightConfiguration::new(
            language(),
            HIGHLIGHTS_QUERY,
            "",
            "",
        ).expect("provide correct cypher queries");

        highlighter_configuration.configure(HighlightColors::groups());

        Self {
            colors,
            highlighter,
            highlighter_configuration,
        }
    }

    pub fn parse<'s: 'r, 'r>(&mut self, source: &'s str, width: u16) -> Chunks<'r> {
        let colors = &self.colors;
        let mut chunks: Chunks<'r> = Chunks::new(width);

        let hl_iterator = self.highlighter.highlight(
            &self.highlighter_configuration,
            source.as_bytes(),
            None,
            |_| None,
        ).expect("highlighter should be able to parse the query");

        let mut highlighting = false;
        let mut style: Option<&'static str> = None;
        let mut code: Option<&'s str> = None;

        for chunk in hl_iterator.filter_map(|item| item.ok()) {
            match chunk {
                HighlightEvent::HighlightStart(Highlight(group)) => {
                    style = Some(colors.get_color(group));
                    highlighting = true;
                },
                HighlightEvent::HighlightEnd => {
                    let s = style.take().unwrap();
                    let c = code.take().unwrap();

                    let chunk = Chunk::new(c, s);

                    chunks.push(chunk);
                    chunks.push(Chunk::new("", self.colors.reset()));
                    highlighting = false;
                },
                HighlightEvent::Source { start, end } => {
                    // If no builder is present then append style-less chunk.
                    let code_chunk = &source[start..end];

                    // If code contains new line character - split chunk.
                    let split = code_chunk.split('\n').collect::<Vec<&str>>();
                    let splits = split.len();

                    // TODO: Whoah, this should be cleaned up a bit :)
                    if highlighting {
                        if splits > 1 {
                            for (i, c) in split.iter().enumerate() {
                                if i + 1 == splits {
                                    code = Some(c);
                                    break;
                                }
                                chunks.push(Chunk::new(c, ""));
                                chunks.push(Chunk::new_line());
                            }
                        } else {
                            code = Some(code_chunk);
                        }
                    } else {
                        if splits > 1 {
                            for (i, c) in split.iter().enumerate() {
                                if i + 1 == splits {
                                    chunks.push(Chunk::new(c, ""));
                                    break;
                                }
                                chunks.push(Chunk::new(c, ""));
                                chunks.push(Chunk::new_line());
                            }
                        } else {
                            chunks.push(Chunk::new(code_chunk, ""));
                        }
                    }
                },
            }
        }

        chunks
    }
}


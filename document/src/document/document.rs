use derive_getters::Getters;
use rayon::prelude::IntoParallelRefIterator;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::{DocumentMeta, Node};

#[derive(Debug, Clone, TypedBuilder, Getters, Serialize, Deserialize)]
pub struct Document {
    meta: DocumentMeta,
    nodes: Vec<Node>,
}

impl Document {
    pub fn text(&self) -> String {
        let mut content = String::new();

        for text in self.all_texts() {
            content.push_str(&text);
        }
        content
    }

    pub fn all_texts(&self) -> Vec<String> {
        self.nodes
            .par_iter()
            .map(|x| extract_node_content(x))
            .collect::<Vec<String>>()
    }

    pub fn all_text_in_lines(&self) -> Vec<String> {
        let lines = self
            .all_texts()
            .par_iter()
            .map(|x| {
                x.split("\n")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
            })
            .flatten()
            .collect::<Vec<String>>();
        lines
    }
}

fn extract_node_content(node: &Node) -> String {
    let mut buf = String::new();

    match node {
        Node::Text(text) => buf.push_str(text.value()),

        Node::TextGroup(text_group) => {
            let mut last_column_end = 0;
            let texts = text_group
                .children()
                .iter()
                .map(|x| {
                    let mut text = String::new();
                    if let Some(position) = x.position() {
                        let start_column = *position.start().column();
                        if start_column > last_column_end {
                            text.push_str(&" ".repeat(start_column - last_column_end));
                        }
                        last_column_end = *position.end().column();
                    }

                    text.push_str(&extract_node_content(x));
                    text
                })
                .collect::<Vec<String>>();
            buf.push_str(&texts.join(""));
        }

        Node::Paragraph(paragraph) => {
            if let Some(position) = paragraph.position() {
                buf.push_str(&" ".repeat(*position.start().column()));
            }
            let texts = paragraph
                .children()
                .iter()
                .map(extract_node_content)
                .collect::<Vec<String>>();
            buf.push_str(&texts.join(""));
        }

        Node::List(list) => {
            if let Some(position) = list.position() {
                buf.push_str(&" ".repeat(*position.start().column()));
            }
            let texts = list
                .children()
                .iter()
                .map(extract_node_content)
                .collect::<Vec<String>>();
            buf.push_str(&texts.join("\n"));
        }

        Node::ListItem(list_item) => {
            let texts = list_item
                .children()
                .iter()
                .map(extract_node_content)
                .collect::<Vec<String>>();
            buf.push_str(&texts.join(" "));
        }

        Node::Table(table) => {
            if let Some(position) = table.position() {
                buf.push_str(&" ".repeat(*position.start().column()));
            }
            let texts = table
                .children()
                .iter()
                .map(extract_node_content)
                .collect::<Vec<String>>();
            buf.push_str(&texts.join(""));
        }

        Node::TableRow(table_row) => {
            let texts = table_row
                .children()
                .iter()
                .map(extract_node_content)
                .collect::<Vec<String>>();
            let indent = match table_row.position() {
                Some(position) => *position.start().column(),
                None => 1,
            };
            buf.push_str(&texts.join(&" ".repeat(indent)));
            buf.push_str("\n");
        }

        Node::TableCell(table_cell) => {
            let texts = table_cell
                .children()
                .iter()
                .map(extract_node_content)
                .collect::<Vec<String>>();
            buf.push_str(&texts.join(" "));
        }
        Node::Heading(heading) => {
            if let Some(position) = heading.position() {
                buf.push_str(&" ".repeat(*position.start().column()));
            }
            let texts = heading
                .children()
                .iter()
                .map(extract_node_content)
                .collect::<Vec<String>>();
            buf.push_str(&texts.join(""));
        }
        Node::LineBreak => {
            buf.push_str("\n");
        }
        _ => {}
    }
    buf
}

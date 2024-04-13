use chrono::DateTime;
use docx_rs::DocumentChild;

use super::{
    docx_meta::DocxMeta,
    extractor::{extract_paragraph, extract_table},
    helper::compute_position,
};
use crate::{
    document::{Document, DocumentMeta, ListItem, Node, Paragraph, Position, Table},
    utils, Result,
};

#[derive(Debug)]
pub struct DocxDcoumentPraser {}

impl DocxDcoumentPraser {
    pub fn new() -> Self {
        Self {}
    }
    pub fn parse(&self, data: Vec<u8>) -> Result<Document> {
        let doc = docx_rs::read_docx(&data)?;
        let docx_meta: DocxMeta = match serde_json::to_value(&doc.doc_props.core) {
            Ok(value) => serde_json::from_value(value).unwrap(),
            Err(_) => DocxMeta::default(),
        };
        let meta = extract_meta(&docx_meta);

        let mut last_position = Position::default();

        let nodes = doc
            .document
            .children
            .iter()
            .fold(vec![], |mut nodes, document_child| {
                match &document_child {
                    DocumentChild::Paragraph(paragraph) => {
                        last_position.reset_column();
                        let position = compute_position(&last_position, &document_child);
                        last_position = position.clone();
                        let children = extract_paragraph(paragraph);
                        let is_list = paragraph.has_numbering;
                        if is_list {
                            let content_node = Node::ListItem(
                                ListItem::builder()
                                    .children(children)
                                    .position(None)
                                    .spread(false)
                                    .checked(None)
                                    .build(),
                            );
                            nodes.push(content_node);
                        } else {
                            let content_node = Node::Paragraph(
                                Paragraph::builder()
                                    .children(children)
                                    .position(None)
                                    .build(),
                            );
                            nodes.push(content_node);
                        }
                    }

                    DocumentChild::Table(table) => {
                        last_position.reset_column();
                        let position = compute_position(&last_position, &document_child);
                        last_position = position.clone();
                        let table_children = extract_table(table);
                        nodes.push(Node::Table(
                            Table::builder()
                                .children(table_children)
                                .position(Some(position))
                                .build(),
                        ))
                    }
                    _ => {}
                }
                nodes
            });

        let result = utils::grouper::group_list_items(nodes);

        let document = Document::builder().meta(meta).nodes(result).build();
        Ok(document)
    }
}

fn extract_meta(docx_meta: &DocxMeta) -> DocumentMeta {
    let title = docx_meta.title().clone().unwrap_or("".to_string());
    let creator = match docx_meta.creator() {
        Some(s) => Some(s.to_string()),
        None => None,
    };
    let language = "en".to_string();
    let creation_date =
        match DateTime::parse_from_rfc3339(&docx_meta.created().clone().unwrap_or(String::new())) {
            Ok(dt) => Some(dt.timestamp_millis()),
            Err(_) => None,
        };
    let modification_date =
        match DateTime::parse_from_rfc3339(&docx_meta.created().clone().unwrap_or(String::new())) {
            Ok(dt) => Some(dt.timestamp_millis()),
            Err(_) => None,
        };

    let subject = match docx_meta.subject() {
        Some(s) => Some(s.to_string()),
        None => None,
    };
    let description = match docx_meta.description() {
        Some(s) => Some(s.to_string()),
        None => None,
    };

    DocumentMeta::builder()
        .title(title)
        .creator(creator.to_owned())
        .author(creator.to_owned())
        .producer(creator.to_owned())
        .language(Some(language))
        .creation_date(creation_date)
        .modification_date(modification_date)
        .subject(subject)
        .description(description)
        .keywords(None)
        .build()
}

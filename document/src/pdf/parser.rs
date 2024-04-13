use pdfium_render::prelude::*;

use super::{extractor, Analyser, Element, TextElement, TextElementGroupKind};
use crate::{
    document::{Document, Heading, ListItem, Node, Paragraph, Point, Position, Text, TextGroup},
    utils, Result,
};

pub struct PdfDocumentParser {
    pdfium: Pdfium,
    analyzer: Analyser,
}

impl PdfDocumentParser {
    pub fn new(pdfium_lib_path: &str) -> Result<Self> {
        let pdfium = Pdfium::new(
            Pdfium::bind_to_library(pdfium_lib_path)
                .or_else(|_| Pdfium::bind_to_system_library())?,
        );
        let analyzer = Analyser::new();
        Ok(Self { pdfium, analyzer })
    }

    pub fn parse(&self, bytes: Vec<u8>) -> Result<Document> {
        let pdfium_document = self.pdfium.load_pdf_from_byte_vec(bytes, None)?;
        let meta = extractor::extract_meta(pdfium_document.metadata());
        let elements = pdfium_document
            .pages()
            .iter()
            .enumerate()
            .map(|(page_num, pdf_page)| {
                extractor::extract_pdf_page(&pdf_page, page_num).unwrap_or(vec![])
            })
            .flatten()
            .collect::<Vec<Element>>();

        let groups = self.analyzer.analyse(elements);
        let mut last_position = Position::init();
        let mut nodes = vec![];

        for group in groups {
            match group.kind() {
                TextElementGroupKind::Heading => {
                    let children = group
                        .children()
                        .iter()
                        .map(|x| {
                            last_position.set_end_column(x.indent());
                            x.elements()
                                .iter()
                                .map(|y| {
                                    let text_node = create_text_node(y, last_position.clone());
                                    last_position = match text_node.position() {
                                        Some(position) => position,
                                        None => last_position.clone(),
                                    };
                                    text_node
                                })
                                .collect::<Vec<Node>>()
                        })
                        .flatten()
                        .collect::<Vec<Node>>();

                    let position = create_position_with_children(&children);

                    nodes.push(Node::Heading(
                        Heading::builder()
                            .children(children)
                            .depth(0)
                            .position(Some(position))
                            .build(),
                    ));
                    last_position.add_line();
                }
                TextElementGroupKind::BulletList | TextElementGroupKind::ReferenceList => {
                    let result = group
                        .children()
                        .iter()
                        .map(|x| {
                            last_position.set_end_column(x.indent());
                            let list_item_children = x
                                .elements()
                                .iter()
                                .map(|y| {
                                    let text_node = create_text_node(y, last_position.clone());
                                    last_position = match text_node.position() {
                                        Some(position) => position,
                                        None => last_position.clone(),
                                    };
                                    text_node
                                })
                                .collect::<Vec<Node>>();

                            let position = create_position_with_children(&list_item_children);

                            let list_item = Node::ListItem(
                                ListItem::builder()
                                    .children(list_item_children)
                                    .position(Some(position))
                                    .spread(false)
                                    .checked(None)
                                    .build(),
                            );
                            last_position.add_line();

                            list_item
                        })
                        .collect::<Vec<Node>>();

                    nodes.extend(result);
                    // let position = create_position_with_children(&children);

                    // nodes.push(Node::List(
                    //     List::builder()
                    //         .children(children)
                    //         .position(Some(position))
                    //         .spread(false)
                    //         .ordered(false)
                    //         .start(None)
                    //         .build(),
                    // ));
                }
                TextElementGroupKind::Paragraph => {
                    let children = group
                        .children()
                        .iter()
                        .map(|x| {
                            last_position.set_end_column(x.indent());
                            x.elements()
                                .iter()
                                .map(|y| {
                                    let text_node = create_text_node(y, last_position.clone());
                                    last_position = match text_node.position() {
                                        Some(position) => position,
                                        None => last_position.clone(),
                                    };
                                    text_node
                                })
                                .collect::<Vec<Node>>()
                        })
                        .flatten()
                        .collect::<Vec<Node>>();

                    let position = create_position_with_children(&children);

                    nodes.push(Node::Paragraph(
                        Paragraph::builder()
                            .children(children)
                            .position(Some(position))
                            .build(),
                    ));
                    last_position.add_line();
                }
                _ => {
                    let groups = group
                        .children()
                        .iter()
                        .map(|x| {
                            last_position.set_end_column(x.indent());

                            let children = x
                                .elements()
                                .iter()
                                .enumerate()
                                .map(|(idx, current)| {
                                    let text_node =
                                        create_text_node(&current, last_position.clone());

                                    last_position = match text_node.position() {
                                        Some(position) => position,
                                        None => last_position.clone(),
                                    };

                                    let next = x.elements().get(idx + 1);
                                    let spaces = match next {
                                        Some(next) => {
                                            ((next.bounds().x1() - current.bounds().x2())
                                                / current.font_size())
                                                as usize
                                        }
                                        None => 0,
                                    };

                                    last_position
                                        .set_end_column(last_position.end().column() + spaces);
                                    last_position.add_line();
                                    text_node
                                })
                                .collect::<Vec<Node>>();

                            let position = create_position_with_children(&children);

                            Node::TextGroup(
                                TextGroup::builder()
                                    .children(children)
                                    .position(Some(position))
                                    .build(),
                            )
                        })
                        .collect::<Vec<Node>>();

                    nodes.extend(groups);
                } // TextElementGroupKind::None => {}
                  // TextElementGroupKind::Table => {}
                  // TextElementGroupKind::PageNumber => {}
            }
        }
        let nodes = utils::grouper::group_list_items(nodes);
        let result = Document::builder().meta(meta).nodes(nodes).build();
        Ok(result)
    }
}

fn create_text_node(text_element: &TextElement, last_position: Position) -> Node {
    // println!("{:?} => {:?}", text_element.bounds(), text_element.text());
    let text = text_element.text().to_string();
    let start_line = *last_position.end().line();
    let start_column = *last_position.end().column();
    let start_offset = *last_position.end().offset();
    let position = Position::builder()
        .start(
            Point::builder()
                .line(start_line)
                .column(start_column)
                .offset(start_offset)
                .build(),
        )
        .end(
            Point::builder()
                .line(start_line)
                .column(start_column + text.len())
                .offset(start_offset + text.len())
                .build(),
        )
        .build();

    Node::Text(Text::builder().value(text).position(Some(position)).build())
}

fn create_position_with_children(children: &Vec<Node>) -> Position {
    let positional_nodes = children
        .iter()
        .filter(|x| x.is_positioal())
        .collect::<Vec<&Node>>();
    let start = positional_nodes
        .first()
        .unwrap()
        .position()
        .unwrap()
        .start()
        .to_owned();
    let end = positional_nodes
        .last()
        .unwrap()
        .position()
        .unwrap()
        .end()
        .to_owned();
    Position::builder().start(start).end(end).build()
}

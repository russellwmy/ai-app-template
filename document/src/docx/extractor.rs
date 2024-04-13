use docx_rs::{ParagraphChild, Run, RunChild, TableChild};

use crate::{
    document::{ListItem, Node, Table, TableCell, TableRow, Text},
    utils,
};

pub(super) fn extract_paragraph(paragraph: &docx_rs::Paragraph) -> Vec<Node> {
    paragraph.children.iter().fold(vec![], |mut acc, child| {
        match &child {
            ParagraphChild::Run(run) => {
                let value = extract_run(*run.to_owned());
                println!("{:?} => {:?}", run.run_property.sz_cs, value);
                println!();
                acc.push(Node::Text(
                    Text::builder().value(value).position(None).build(),
                ));
            }
            _ => {}
        };
        acc
    })
}

pub(super) fn extract_run(run: Run) -> String {
    let mut buf = String::new();
    for run_child in run.children {
        match run_child {
            RunChild::Text(text) => {
                buf.push_str(&text.text);
            }
            RunChild::Tab(tab) => {
                buf.push_str(&"\t".repeat(tab.pos.unwrap_or(0)));
            }
            _ => {}
        };
    }
    buf
}

pub(super) fn extract_table(table: &docx_rs::Table) -> Vec<Node> {
    table.rows.iter().fold(vec![], |mut rows, row_child| {
        match &row_child {
            TableChild::TableRow(table_row) => {
                let row_children = extract_table_row(table_row);
                rows.push(Node::TableRow(
                    TableRow::builder()
                        .children(row_children)
                        .position(None)
                        .build(),
                ));
            }
            _ => {}
        };
        rows
    })
}

fn extract_table_row(table_row: &docx_rs::TableRow) -> Vec<Node> {
    table_row
        .cells
        .iter()
        .fold(vec![], |mut cells, cell_child| {
            match cell_child {
                docx_rs::TableRowChild::TableCell(table_cell) => {
                    let cell_children = extract_table_cell(table_cell);

                    cells.push(Node::TableCell(
                        TableCell::builder()
                            .children(cell_children)
                            .position(None)
                            .build(),
                    ))
                }
            }
            cells
        })
}

fn extract_table_cell(table_cell: &docx_rs::TableCell) -> Vec<Node> {
    table_cell
        .children
        .iter()
        .fold(vec![], |mut cell_contents, cell_content| {
            match cell_content {
                docx_rs::TableCellContent::Paragraph(paragraph) => {
                    let children = extract_paragraph(paragraph);
                    match paragraph.has_numbering {
                        true => {
                            cell_contents.push(Node::ListItem(
                                ListItem::builder()
                                    .children(children)
                                    .position(None)
                                    .spread(false)
                                    .checked(None)
                                    .build(),
                            ));
                        }
                        false => {
                            cell_contents.extend(children);
                        }
                    };
                }
                docx_rs::TableCellContent::Table(table) => {
                    let table_children = extract_table(table);
                    cell_contents.push(Node::Table(
                        Table::builder()
                            .children(table_children.to_vec())
                            .position(None)
                            .build(),
                    ))
                }
                docx_rs::TableCellContent::StructuredDataTag(_) => todo!(),
                docx_rs::TableCellContent::TableOfContents(_) => todo!(),
            }

            utils::grouper::group_list_items(cell_contents)
        })
}

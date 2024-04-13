use docx_rs::DocumentChild;

use crate::document::{Point, Position};

pub(super) fn compute_position(
    last_position: &Position,
    document_child: &docx_rs::DocumentChild,
) -> Position {
    let start_line = *last_position.end().line() + 1;
    let start_offset = *last_position.end().offset();
    let mut start_column = 0;

    let mut column_change = 0;
    let mut line_change = 0;
    let mut offset_change = 0;

    match document_child {
        DocumentChild::Paragraph(paragraph) => {
            start_column = match &paragraph.property.indent {
                Some(indent) => indent.start_chars.unwrap_or(0) as usize,
                None => 0,
            };
            let text = paragraph.raw_text();
            let char_count = text.trim_start().chars().count();

            line_change = text.split("\n").count() - 1;
            column_change = char_count;
            offset_change = char_count;
        }
        DocumentChild::Table(table) => {
            line_change = table.rows.len();
        }
        _ => {}
    };

    let end_line = start_line + line_change;
    let end_column = start_column + column_change;
    let end_offset = start_offset + offset_change;

    let start = Point::builder()
        .line(start_line)
        .offset(start_offset)
        .column(start_column)
        .build();
    let end = Point::builder()
        .line(end_line)
        .offset(end_offset)
        .column(end_column)
        .build();

    Position::builder().start(start).end(end).build()
}

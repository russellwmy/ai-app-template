use std::collections::HashMap;

use super::{Rect, TextElement, TextElementGroup, TextElementGroupKind, TextLine};
use crate::utils;

pub(super) fn build_detect_bounds(element: &TextElement) -> Rect {
    Rect::builder()
        .x1(*element.bounds().x2())
        .y1(*element.bounds().y1())
        .x2(element.bounds().x2() + (element.page().width() - element.bounds().x2()))
        .y2(*element.bounds().y2())
        .build()
}

pub fn compute_paragraph_font_sizes(values: &Vec<f32>) -> Vec<f32> {
    let mut counter = HashMap::new();
    let count = values.len();
    for value in values {
        let key = value.to_string();
        counter
            .entry(key)
            .and_modify(|count| *count += 1)
            .or_insert(0);
    }
    counter
        .into_iter()
        .filter(|(_, v)| (*v as f32 / count as f32) > 0.4)
        .map(|(k, _)| k.parse::<f32>().unwrap())
        .collect()
}

pub(super) fn merge_same_style_elements(elements: &Vec<TextElement>) -> Vec<TextElement> {
    let mut result: Vec<TextElement> = vec![];
    for element in elements {
        let last = result.last();
        match last {
            Some(last_element) => match last_element.is_same_style(&element) {
                true => {
                    let mut new_element = result.pop().unwrap();
                    new_element.merge(&element);
                    result.push(new_element)
                }
                false => result.push(element.to_owned()),
            },
            None => result.push(element.to_owned()),
        }
    }

    result
}

pub(super) fn rebuild_bullet_list_group(group: &TextElementGroup) -> TextElementGroup {
    if group.kind().to_owned() == TextElementGroupKind::BulletList {
        let mut children = vec![];
        let mut buf = vec![];
        for text_line in group.children() {
            if utils::text::check_start_with_bullet(&text_line.text()) && !buf.is_empty() {
                let result = merge_same_style_elements(&buf);

                children.push(TextLine::builder().elements(result).build());
                buf = text_line.elements().to_owned();
            } else {
                buf.extend(text_line.elements().to_owned());
            }
        }

        if !buf.is_empty() {
            let result = merge_same_style_elements(&buf);
            children.push(TextLine::builder().elements(result).build());
        }

        TextElementGroup::builder()
            .children(children)
            .kind(*group.kind())
            .build()
    } else {
        group.to_owned()
    }
}

pub(super) fn rebuild_reference_list_group(group: &TextElementGroup) -> TextElementGroup {
    if group.kind().to_owned() == TextElementGroupKind::ReferenceList {
        let mut children = vec![];
        let mut buf = vec![];
        for text_line in group.children() {
            if utils::text::check_start_with_reference_number(&text_line.text()) && !buf.is_empty()
            {
                let result = merge_same_style_elements(&buf);
                children.push(TextLine::builder().elements(result).build());
                buf = text_line.elements().to_owned();
            } else {
                buf.extend(text_line.elements().to_owned());
            }
        }

        if !buf.is_empty() {
            let result = merge_same_style_elements(&buf);
            children.push(TextLine::builder().elements(result).build());
        }

        TextElementGroup::builder()
            .children(children)
            .kind(*group.kind())
            .build()
    } else {
        group.to_owned()
    }
}

pub(super) fn rebuild_paragraph_group(group: &TextElementGroup) -> TextElementGroup {
    if group.kind().to_owned() == TextElementGroupKind::Paragraph {
        let elements = group
            .children()
            .iter()
            .map(|x| x.elements().iter().map(|y| y.to_owned()))
            .flatten()
            .collect::<Vec<TextElement>>();

        let result = merge_same_style_elements(&elements);
        let text_line = TextLine::builder().elements(result).build();

        TextElementGroup::builder()
            .children(vec![text_line])
            .kind(*group.kind())
            .build()
    } else {
        group.to_owned()
    }
}

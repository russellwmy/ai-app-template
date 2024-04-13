use super::{helper, Rect, TextElement, TextElementGroup, TextElementGroupKind, TextLine};

pub(crate) fn group_text_elements(elements: &Vec<TextElement>) -> Vec<TextElement> {
    let mut result: Vec<TextElement> = vec![];
    for element in elements {
        let last = result.last();
        match last {
            Some(last_element) => {
                match last_element.is_same_style(&element) && last_element.is_same_line(&element) {
                    true => {
                        let mut new_element = result.pop().unwrap();
                        new_element.merge(element);
                        result.push(new_element)
                    }
                    false => result.push(element.to_owned()),
                }
            }
            None => result.push(element.to_owned()),
        }
    }

    result
        .into_iter()
        .filter(|x| !x.text().is_empty())
        .collect::<Vec<TextElement>>()
}

pub(crate) fn group_text_in_line(elements: &Vec<TextElement>) -> Vec<TextLine> {
    let mut result = vec![];
    let mut buf = vec![];
    let elements = group_text_elements(elements);

    let mut detect_bounds = match elements.first() {
        Some(first) => helper::build_detect_bounds(first),
        None => Rect::zero(),
    };

    for (index, current) in elements.iter().enumerate() {
        let next = elements.get(index + 1);

        detect_bounds = Rect::union(&vec![detect_bounds.clone(), current.bounds().to_owned()]);

        let is_overlap = match next {
            Some(next) => detect_bounds.overlap(&next.bounds()),
            None => false,
        };

        // println!(
        //     "{:?} => {:?}",
        //     (is_overlap, &detect_bounds, current.bounds()),
        //     current.text()
        // );

        buf.push(current.to_owned());

        if !is_overlap {
            result.push(TextLine::builder().elements(buf).build());
            buf = vec![];
            // Build new detection bounds
            detect_bounds = match next {
                Some(next) => helper::build_detect_bounds(next),
                None => Rect::zero(),
            };
        }
    }
    result
}

pub(crate) fn group_line_in_group(text_lines: &Vec<TextLine>) -> Vec<TextElementGroup> {
    let mut result = vec![];
    let mut buf = vec![];

    for (index, current) in text_lines.iter().enumerate() {
        let next = text_lines.get(index + 1);
        let distance = match next {
            Some(next) => (current.bounds().y2() - next.bounds().y1()).abs(),
            None => 0.0,
        };

        buf.push(current.to_owned());

        if distance >= 5.0 {
            result.push(
                TextElementGroup::builder()
                    .children(buf)
                    .kind(TextElementGroupKind::None)
                    .build(),
            );
            buf = vec![];
        }
    }
    result
}

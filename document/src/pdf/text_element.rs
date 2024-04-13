use derive_getters::Getters;
use typed_builder::TypedBuilder;

use super::{FontWeight, Page, Rect};

#[derive(Debug, Clone, PartialEq, TypedBuilder, Getters)]
pub struct TextElement {
    text: String,
    bounds: Rect,
    page: Page,
    font_name: String,
    font_size: f32,
    font_weight: FontWeight,
}

impl TextElement {
    pub fn merge(&mut self, other: &Self) {
        let mut buf = String::from(&self.text.to_owned());
        buf.push_str(&other.text);
        self.text = buf;

        self.bounds = Rect::union(&vec![self.bounds.to_owned(), other.bounds.to_owned()]);
    }

    pub fn line_bounds(&self) -> Rect {
        let y_padding = self.font_size;
        Rect::builder()
            .x1(0.0)
            .y1(*self.bounds.y1() + y_padding)
            .x2(*self.page.width())
            .y2(*self.bounds.y2() - y_padding)
            .build()
    }

    pub fn is_same_style(&self, other: &Self) -> bool {
        self.font_name == other.font_name
            && self.font_size == other.font_size
            && self.font_weight == other.font_weight
    }

    pub fn is_same_line(&self, other: &Self) -> bool {
        self.bounds
            .bottom_right()
            .distance(&other.bounds().bottom_left())
            < 3.0
    }
}

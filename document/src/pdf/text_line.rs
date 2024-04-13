use derive_getters::Getters;
use typed_builder::TypedBuilder;

use super::{constants, Aligment, Page, Rect, TextElement};
use crate::utils;

#[derive(Debug, Clone, PartialEq, TypedBuilder, Getters)]
pub struct TextLine {
    elements: Vec<TextElement>,
}

impl TextLine {
    pub fn text(&self) -> String {
        let mut buf = String::new();
        let font_size = self.font_size();

        for (idx, current) in self.elements.iter().enumerate() {
            buf.push_str(&current.text());

            let next = self.elements.get(idx + 1);
            if let Some(next) = next {
                let dist = next.bounds().x1() - current.bounds().x2();
                if font_size > 0.0 {
                    let spaces = " ".repeat((dist / font_size * self.ratio()).round() as usize);
                    buf.push_str(&spaces);
                } else {
                    buf.push_str(" ");
                }
            }
        }

        buf
    }

    pub fn bounds(&self) -> Rect {
        let rects = self
            .elements
            .iter()
            .map(|x| x.bounds().to_owned())
            .collect::<Vec<Rect>>();
        Rect::union(&rects)
    }

    pub fn font_size(&self) -> f32 {
        let values = self
            .elements
            .iter()
            .map(|x| *x.font_size())
            .collect::<Vec<f32>>();
        utils::stats::mode(&values)
    }

    pub fn indent(&self) -> usize {
        let head = *self.bounds().x1();
        match head > 0.0 {
            true => (head / constants::REFERENCE_CHAR_WIDTH).round() as usize,
            false => 0,
        }
    }

    pub fn alignment(&self) -> Aligment {
        let page = self.page();
        let bounds = self.bounds();
        let page_width = page.width();
        let content_width = bounds.width();
        let head = bounds.x1().round();
        let page_center = page_width / 2.0;
        let content_center = head + content_width / 2.0;
        let diff_center = page_center - content_center;
        let content_ratio = content_width / page_width;

        if diff_center >= -100.0 && diff_center <= 100.0 && content_ratio < 0.6 {
            Aligment::Center
        } else if diff_center < -100.0 {
            Aligment::Right
        } else {
            Aligment::Left
        }
    }

    pub fn page(&self) -> &Page {
        self.elements.first().unwrap().page()
    }

    fn ratio(&self) -> f32 {
        self.font_size() / constants::REFERENCE_CHAR_WIDTH
    }
}

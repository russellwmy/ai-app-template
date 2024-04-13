use derive_getters::Getters;
use typed_builder::TypedBuilder;

use super::{Rect, TextElementGroupKind, TextLine};
use crate::utils;

#[derive(Debug, Clone, PartialEq, TypedBuilder, Getters)]
pub struct TextElementGroup {
    kind: TextElementGroupKind,
    children: Vec<TextLine>,
}

impl TextElementGroup {
    pub fn text(&self) -> String {
        let texts = self
            .children
            .iter()
            .map(|x| x.text().to_owned())
            .collect::<Vec<String>>();
        texts.join("\n")
    }

    pub fn bounds(&self) -> Rect {
        let rects = self
            .children
            .iter()
            .map(|x| x.bounds().to_owned())
            .collect::<Vec<Rect>>();
        Rect::union(&rects)
    }

    pub fn font_size(&self) -> f32 {
        let values = self
            .children
            .iter()
            .map(|x| x.font_size())
            .collect::<Vec<f32>>();
        utils::stats::mode(&values)
    }

    pub fn line_count(&self) -> usize {
        self.text().split("\n").count()
    }

    pub fn element_count(&self) -> usize {
        self.children
            .iter()
            .map(|x| x.elements().len())
            .sum::<usize>()
    }

    pub fn line_distance_mean(&self) -> f32 {
        let distances = self
            .children
            .windows(2)
            .map(|w| w[0].bounds().y2() - w[1].bounds().y1())
            .sum::<f32>();
        distances / self.line_count() as f32
    }
}

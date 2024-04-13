use derive_getters::Getters;
use typed_builder::TypedBuilder;

use super::{grouper, helper, Element, TextElement, TextElementGroup, TextElementGroupKind};
use crate::utils;

#[derive(Debug, Clone, TypedBuilder, Getters)]
struct AnalyserState {
    paragraph_font_sizes: Vec<f32>,
}

pub struct Analyser {}

impl Analyser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn analyse(&self, elements: Vec<Element>) -> Vec<TextElementGroup> {
        let mut result = vec![];
        let text_elements = elements
            .iter()
            .filter(|x| x.as_text().is_ok())
            .map(|x| x.as_text().unwrap())
            .collect::<Vec<TextElement>>();
        let lines = grouper::group_text_in_line(&text_elements);
        let groups = grouper::group_line_in_group(&lines);
        let paragraph_font_sizes = helper::compute_paragraph_font_sizes(
            &groups
                .iter()
                .map(|x: &TextElementGroup| x.font_size())
                .collect(),
        );
        let state: AnalyserState = AnalyserState::builder()
            .paragraph_font_sizes(paragraph_font_sizes)
            .build();

        for (index, current) in groups.iter().enumerate() {
            let next = groups.get(index + 1);
            // println!("{:?} => {:?}", current.font_size(), current.text());

            if self.check_if_page_number(current) {
                let new_group = TextElementGroup::builder()
                    .children(current.children().clone())
                    .kind(TextElementGroupKind::PageNumber)
                    .build();
                result.push(new_group);
            } else if self.check_if_bullet_list(current) {
                let result_group = TextElementGroup::builder()
                    .children(current.children().clone())
                    .kind(TextElementGroupKind::BulletList)
                    .build();

                let new_group = helper::rebuild_bullet_list_group(&result_group);
                result.push(new_group);
            } else if self.check_if_reference_list(current) {
                let result_group = TextElementGroup::builder()
                    .children(current.children().clone())
                    .kind(TextElementGroupKind::ReferenceList)
                    .build();
                let new_group = helper::rebuild_reference_list_group(&result_group);

                result.push(new_group);
            } else if next.is_some() && self.check_if_heading(current, next.unwrap(), &state) {
                let new_group = TextElementGroup::builder()
                    .children(current.children().clone())
                    .kind(TextElementGroupKind::Heading)
                    .build();
                result.push(new_group);
            } else if self.check_if_paragraph(current, &state) {
                let result_group = TextElementGroup::builder()
                    .children(current.children().clone())
                    .kind(TextElementGroupKind::Paragraph)
                    .build();
                let new_group = helper::rebuild_paragraph_group(&result_group);
                result.push(new_group);
            } else {
                let new_group = TextElementGroup::builder()
                    .children(current.children().clone())
                    .kind(TextElementGroupKind::None)
                    .build();
                result.push(new_group);
            }
        }

        result
    }

    fn check_if_paragraph(&self, group: &TextElementGroup, state: &AnalyserState) -> bool {
        let text = group.text();
        let paragraph_font_sizes = state.paragraph_font_sizes.clone();
        let font_size = group.font_size();
        let min_paragraph_font_size = paragraph_font_sizes
            .iter()
            .min_by(|a, b| a.partial_cmp(&b).unwrap())
            .unwrap_or(&font_size);

        let is_paragraph_font_size =
            state.paragraph_font_sizes.contains(&font_size) || font_size < *min_paragraph_font_size;
        let word_count = utils::text::count_word(&text);

        let space_in_between = text
            .split("\n")
            .map(|s| utils::text::count_space(s.trim()) as f32)
            .collect::<Vec<f32>>();
        let space_in_between_mean = utils::stats::mean(&space_in_between) as f32;
        let space_ratio = space_in_between_mean / word_count as f32;
        let multi_space_in_between = text
            .split("\n")
            .map(|s| utils::text::count_multi_space(s.trim()) as f32)
            .collect::<Vec<f32>>();
        let mean_multi_space_in_between = utils::stats::mean(&multi_space_in_between);

        is_paragraph_font_size
            && group.line_distance_mean() < 3.0
            && space_ratio < 1.5
            && word_count > 4
            && mean_multi_space_in_between < 2.0
    }

    fn check_if_heading(
        &self,
        group: &TextElementGroup,
        next_group: &TextElementGroup,
        state: &AnalyserState,
    ) -> bool {
        let text = group.text();
        let font_size = group.font_size();
        let next_font_size = next_group.font_size();
        let next_is_paragraph = state.paragraph_font_sizes.contains(&next_font_size);
        let word_count = utils::text::count_word(&text);
        next_is_paragraph
            && font_size > next_font_size
            && group.line_count() <= 2
            && word_count > 0
            && text.len() >= 3
    }
    fn check_if_bullet_list(&self, group: &TextElementGroup) -> bool {
        let text = group.text();
        let word_count = utils::text::count_word(&text);

        utils::text::check_start_with_bullet(&text) && word_count > 0
    }
    fn check_if_reference_list(&self, group: &TextElementGroup) -> bool {
        let text = group.text();
        utils::text::check_start_with_reference_number(&text)
    }

    fn check_if_page_number(&self, group: &TextElementGroup) -> bool {
        let text = match group.text().is_ascii() {
            true => group.text().to_lowercase().replace("page", ""),
            false => group.text(),
        };
        let word_count = utils::text::count_word(&text);

        text.trim().chars().all(|c| c.is_numeric())
            && group.line_count() == 1
            && utils::text::count_space(&text) < 3
            && word_count < 3
    }
}

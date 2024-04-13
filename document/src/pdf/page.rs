use derive_getters::Getters;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, TypedBuilder, Getters)]
pub struct Page {
    page_num: usize,
    width: f32,
    height: f32,
}

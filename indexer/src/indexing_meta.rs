use derive_getters::Getters;
use typed_builder::TypedBuilder;

#[derive(Clone, Debug, TypedBuilder, Getters)]
pub struct IndexingMeta {
    id: String,
    title: String,
    external_link: String,
}

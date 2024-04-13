use super::{Page, Rect, TextElement};
use crate::Result;

#[derive(Debug, Clone, PartialEq)]
pub enum Element {
    Text(TextElement),
}

impl Element {
    pub fn page(&self) -> Page {
        match self {
            Element::Text(o) => o.page().to_owned(),
        }
    }

    pub fn bounds(&self) -> Rect {
        match self {
            Element::Text(o) => o.bounds().to_owned(),
        }
    }

    pub fn as_text(&self) -> Result<TextElement> {
        match self {
            Element::Text(o) => Ok(o.clone()),
        }
    }
}

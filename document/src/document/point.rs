use core::fmt;

use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Clone, Eq, Default, PartialEq, TypedBuilder, Getters, Serialize, Deserialize)]
pub struct Point {
    /// 1-indexed integer representing a line in a source file.
    line: usize,
    /// 1-indexed integer representing a column in a source file.
    column: usize,
    /// 0-indexed integer representing a character in a source file.
    offset: usize,
}
impl Point {
    pub(crate) fn init() -> Self {
        Self {
            line: 1,
            column: 0,
            offset: 0,
        }
    }

    pub(crate) fn set_line(&mut self, line: usize) {
        self.line = line;
    }
    pub(crate) fn set_column(&mut self, column: usize) {
        self.column = column;
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{} ({})", self.line, self.column, self.offset)
    }
}

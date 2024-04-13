use core::fmt;

use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::Point;

#[derive(Clone, Eq, Default, PartialEq, TypedBuilder, Getters, Serialize, Deserialize)]
pub struct Position {
    /// Represents the place of the first character of the parsed source region.
    start: Point,
    /// Represents the place of the first character after the parsed source
    /// region, whether it exists or not.
    end: Point,
}
impl Position {
    pub fn reset_column(&mut self) {
        self.end = Point::builder()
            .line(*self.end.line())
            .offset(*self.end.offset())
            .column(0)
            .build();
    }

    pub fn init() -> Self {
        Self {
            start: Point::init(),
            end: Point::init(),
        }
    }

    pub fn add_line(&mut self) {
        self.start.set_line(self.start.line() + 1);
        self.end.set_line(self.end.line() + 1);
    }

    pub fn set_end_column(&mut self, column: usize) {
        self.end.set_column(column);
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}-{}:{} ({}-{})",
            self.start.line(),
            self.start.column(),
            self.end.line(),
            self.end.column(),
            self.start.offset(),
            self.end.offset(),
        )
    }
}

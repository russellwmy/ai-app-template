use std::fmt::Debug;

use derive_getters::Getters;
use typed_builder::TypedBuilder;

#[derive(Clone, PartialEq, TypedBuilder, Getters)]
pub struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn distance(&self, other: &Self) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    pub fn angle(&self, other: &Self) -> f32 {
        f32::atan2(self.x - other.x, self.y - other.y)
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({x}, {y})", x = self.x, y = self.y,)
    }
}

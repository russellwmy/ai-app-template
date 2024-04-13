use std::fmt::Debug;

use derive_getters::Getters;
use typed_builder::TypedBuilder;

use super::Point;

#[derive(Clone, PartialEq, TypedBuilder, Getters)]
pub struct Rect {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
}
impl Rect {
    pub fn zero() -> Self {
        Self {
            x1: 0.0,
            y1: 0.0,
            x2: 0.0,
            y2: 0.0,
        }
    }
}
impl Rect {
    pub fn center(&self) -> Point {
        let x = self.x1 + self.width() / 2.0;
        let y = self.y1 + self.height() / 2.0;
        Point::builder().x(x).y(y).build()
    }

    pub fn left_center(&self) -> Point {
        let x = self.x1;
        let y = self.y1 + self.height() / 2.0;
        Point::builder().x(x).y(y).build()
    }

    pub fn right_center(&self) -> Point {
        let x = self.x2;
        let y = self.y1 + self.height() / 2.0;
        Point::builder().x(x).y(y).build()
    }
    pub fn top_center(&self) -> Point {
        let x = self.x1 + self.width() / 2.0;
        let y = self.y1;
        Point::builder().x(x).y(y).build()
    }

    pub fn bottom_center(&self) -> Point {
        let x = self.x1 + self.width() / 2.0;
        let y = self.y2;
        Point::builder().x(x).y(y).build()
    }

    pub fn top_left(&self) -> Point {
        let x = self.x1;
        let y = self.y1;
        Point::builder().x(x).y(y).build()
    }

    pub fn top_right(&self) -> Point {
        let x = self.x2;
        let y = self.y1;
        Point::builder().x(x).y(y).build()
    }

    pub fn bottom_left(&self) -> Point {
        let x = self.x1;
        let y = self.y2;
        Point::builder().x(x).y(y).build()
    }

    pub fn bottom_right(&self) -> Point {
        let x = self.x2;
        let y = self.y2;
        Point::builder().x(x).y(y).build()
    }

    pub fn height(&self) -> f32 {
        (self.y1 - self.y2).abs()
    }

    pub fn width(&self) -> f32 {
        (self.x1 - self.x2).abs()
    }

    pub(super) fn union(rects: &Vec<Rect>) -> Rect {
        let x1 = rects
            .iter()
            .map(|x| x.x1().to_owned())
            .min_by(|a, b| a.partial_cmp(&b).unwrap())
            .unwrap_or(0.0);
        let y1 = rects
            .iter()
            .map(|x| x.y1().to_owned())
            .max_by(|a, b| a.partial_cmp(&b).unwrap())
            .unwrap_or(0.0);
        let x2 = rects
            .iter()
            .map(|x| x.x2().to_owned())
            .max_by(|a, b| a.partial_cmp(&b).unwrap())
            .unwrap_or(0.0);
        let y2 = rects
            .iter()
            .map(|x| x.y2().to_owned())
            .min_by(|a, b| a.partial_cmp(&b).unwrap())
            .unwrap_or(0.0);

        Rect { x1, y1, x2, y2 }
    }

    pub fn overlap(&self, other: &Rect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 >= other.y2 && self.y2 <= other.y1
    }
}

impl Debug for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({x1}, {y1}, {x2}, {y2})",
            x1 = self.x1,
            x2 = self.x2,
            y1 = self.y1,
            y2 = self.y2
        )
    }
}

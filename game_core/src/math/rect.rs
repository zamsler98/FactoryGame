use std::ops::{Add, Sub};

use super::vec2::Vec2;

/// An axis-aligned rectangle, stored as its smallest corner plus a size.
///
/// Half-open on both axes: `min` is inside the rectangle and `max` is not, so
/// rectangles that share an edge do not overlap and a point on that edge
/// belongs to exactly one of them. Sizes are assumed non-negative.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Rect<T> {
    /// The corner with the smallest coordinates on both axes.
    pub position: Vec2<T>,
    pub size: Vec2<T>,
}

/// A `Rect` of whole units: tile and chunk regions.
pub type Recti = Rect<i32>;
/// A `Rect` of fractional units: screen and camera regions.
pub type Rectf = Rect<f32>;

impl<T> Rect<T> {
    pub const fn new(position: Vec2<T>, size: Vec2<T>) -> Self {
        Self { position, size }
    }

    /// The corner with the smallest coordinates. Inside the rectangle.
    pub fn min(self) -> Vec2<T> {
        self.position
    }
}

impl<T: Sub<Output = T> + Copy> Rect<T> {
    /// The rectangle spanning `min` up to `max`. `max` must not be smaller than
    /// `min` on either axis.
    pub fn from_corners(min: Vec2<T>, max: Vec2<T>) -> Self {
        Self::new(min, max - min)
    }
}

impl<T: Add<Output = T> + Copy> Rect<T> {
    /// The corner with the largest coordinates. Outside the rectangle.
    pub fn max(self) -> Vec2<T> {
        self.position + self.size
    }
}

impl<T: Add<Output = T> + PartialOrd + Copy> Rect<T> {
    /// Whether `point` lies inside, counting the `min` edges but not the `max`.
    pub fn contains(self, point: Vec2<T>) -> bool {
        let (min, max) = (self.min(), self.max());
        point.x >= min.x && point.x < max.x && point.y >= min.y && point.y < max.y
    }

    /// Whether the two rectangles share any interior. Touching edges do not
    /// count, so this agrees with `contains`.
    pub fn overlaps(self, other: Self) -> bool {
        let (min, max) = (self.min(), self.max());
        let (other_min, other_max) = (other.min(), other.max());
        min.x < other_max.x && other_min.x < max.x && min.y < other_max.y && other_min.y < max.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::{Vec2f, Vec2i};

    /// A 4x4 rect with its corner at (2, 3), so min is (2, 3) and max is (6, 7).
    fn rect() -> Recti {
        Recti::new(Vec2i::new(2, 3), Vec2i::new(4, 4))
    }

    #[test]
    fn min_and_max_are_the_two_corners() {
        assert_eq!(rect().min(), Vec2i::new(2, 3));
        assert_eq!(rect().max(), Vec2i::new(6, 7));
    }

    #[test]
    fn from_corners_round_trips_through_min_and_max() {
        let min = Vec2i::new(2, 3);
        let max = Vec2i::new(6, 7);
        let r = Recti::from_corners(min, max);

        assert_eq!(r, rect());
        assert_eq!(r.min(), min);
        assert_eq!(r.max(), max);
    }

    #[test]
    fn from_corners_handles_negative_coordinates() {
        let r = Rectf::from_corners(Vec2f::new(-3.0, -1.5), Vec2f::new(1.0, 0.5));

        assert_eq!(r.position, Vec2f::new(-3.0, -1.5));
        assert_eq!(r.size, Vec2f::new(4.0, 2.0));
    }

    #[test]
    fn contains_includes_the_min_edge_and_excludes_the_max_edge() {
        assert!(rect().contains(Vec2i::new(2, 3)));
        assert!(rect().contains(Vec2i::new(5, 6)));
        assert!(!rect().contains(Vec2i::new(6, 7)));
        assert!(!rect().contains(Vec2i::new(2, 7)));
        assert!(!rect().contains(Vec2i::new(6, 3)));
    }

    #[test]
    fn contains_rejects_points_outside() {
        assert!(!rect().contains(Vec2i::new(1, 5)));
        assert!(!rect().contains(Vec2i::new(5, 2)));
        assert!(!rect().contains(Vec2i::new(-4, -4)));
    }

    #[test]
    fn overlaps_is_true_for_a_partial_intersection() {
        let other = Recti::new(Vec2i::new(4, 5), Vec2i::new(4, 4));
        assert!(rect().overlaps(other));
        assert!(other.overlaps(rect()));
    }

    #[test]
    fn overlaps_is_true_for_containment_and_self() {
        let inner = Recti::new(Vec2i::new(3, 4), Vec2i::new(1, 1));
        assert!(rect().overlaps(inner));
        assert!(inner.overlaps(rect()));
        assert!(rect().overlaps(rect()));
    }

    #[test]
    fn overlaps_is_false_for_touching_edges() {
        // Sits flush against the right edge of `rect`, sharing x = 6.
        let flush = Recti::new(Vec2i::new(6, 3), Vec2i::new(4, 4));
        assert!(!rect().overlaps(flush));
        assert!(!flush.overlaps(rect()));
    }

    #[test]
    fn overlaps_is_false_for_disjoint_rects() {
        let far = Recti::new(Vec2i::new(20, 20), Vec2i::new(1, 1));
        assert!(!rect().overlaps(far));
        assert!(!far.overlaps(rect()));
    }
}

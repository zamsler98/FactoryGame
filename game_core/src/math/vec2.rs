use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

/// A 2D vector. The single 2D pair type used across the workspace: world
/// positions, chunk indices, camera offsets and screen coordinates are all
/// `Vec2`, differing only in their component type.
///
/// `Eq` and `Hash` are derived, so they apply to integer vectors (which are
/// used as map keys) but not to float ones, which cannot be keys anyway.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

/// A `Vec2` of whole units: world positions and chunk indices.
pub type Vec2i = Vec2<i32>;
/// A `Vec2` of fractional units: camera offsets and screen coordinates.
pub type Vec2f = Vec2<f32>;

impl<T> Vec2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl Vec2i {
    pub const ZERO: Self = Self::new(0, 0);

    /// Component-wise Euclidean division. Negative coordinates floor away from
    /// zero, so a position maps to the cell it sits in rather than toward the
    /// origin.
    pub const fn div_euclid(self, rhs: i32) -> Self {
        Self::new(self.x.div_euclid(rhs), self.y.div_euclid(rhs))
    }

    /// This vector in fractional units, for handing whole-unit game state to
    /// the float-based drawing APIs.
    pub fn as_vec2f(self) -> Vec2f {
        Vec2f::new(self.x as f32, self.y as f32)
    }
}

impl Vec2f {
    pub const ZERO: Self = Self::new(0.0, 0.0);
}

impl<T: Add<Output = T>> Add for Vec2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Sub<Output = T>> Sub for Vec2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: AddAssign> AddAssign for Vec2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: SubAssign> SubAssign for Vec2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Neg<Output = T>> Neg for Vec2<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

/// Scale by a scalar, e.g. chunk indices to the world position of their corner.
impl<T: Mul<Output = T> + Copy> Mul<T> for Vec2<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Vec2<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_sets_both_components() {
        let v = Vec2i::new(3, -4);
        assert_eq!(v.x, 3);
        assert_eq!(v.y, -4);
    }

    #[test]
    fn zero_is_the_origin() {
        assert_eq!(Vec2i::ZERO, Vec2i::new(0, 0));
        assert_eq!(Vec2f::ZERO, Vec2f::new(0.0, 0.0));
    }

    #[test]
    fn default_matches_zero() {
        assert_eq!(Vec2i::default(), Vec2i::ZERO);
    }

    #[test]
    fn add_and_sub() {
        let a = Vec2i::new(1, 2);
        let b = Vec2i::new(10, 20);
        assert_eq!(a + b, Vec2i::new(11, 22));
        assert_eq!(a - b, Vec2i::new(-9, -18));
    }

    #[test]
    fn add_assign_and_sub_assign() {
        let mut v = Vec2f::new(1.0, 2.0);
        v += Vec2f::new(0.5, 0.5);
        assert_eq!(v, Vec2f::new(1.5, 2.5));
        v -= Vec2f::new(1.5, 0.5);
        assert_eq!(v, Vec2f::new(0.0, 2.0));
    }

    #[test]
    fn neg_flips_both_components() {
        assert_eq!(-Vec2i::new(3, -4), Vec2i::new(-3, 4));
    }

    #[test]
    fn mul_and_div_by_scalar() {
        assert_eq!(Vec2i::new(2, -3) * 4, Vec2i::new(8, -12));
        assert_eq!(Vec2i::new(8, -12) / 4, Vec2i::new(2, -3));
    }

    #[test]
    fn div_euclid_floors_negatives_away_from_zero() {
        assert_eq!(Vec2i::new(0, 7).div_euclid(8), Vec2i::new(0, 0));
        assert_eq!(Vec2i::new(8, 16).div_euclid(8), Vec2i::new(1, 2));
        assert_eq!(Vec2i::new(-1, -8).div_euclid(8), Vec2i::new(-1, -1));
        assert_eq!(Vec2i::new(-9, -16).div_euclid(8), Vec2i::new(-2, -2));
    }

    #[test]
    fn as_vec2f_widens_components() {
        assert_eq!(Vec2i::new(-2, 5).as_vec2f(), Vec2f::new(-2.0, 5.0));
    }
}

use std::{
    fmt::{Display, Formatter},
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

// TODO(TmLev): generalise over inner type and size, i.e. Vec<5, u8>
#[derive(Copy, Clone, Default)]
pub struct Vec3 {
    coordinates: [f64; 3],
}

impl Vec3 {
    // Constructors

    pub fn new(c0: f64, c1: f64, c2: f64) -> Self {
        Self {
            coordinates: [c0, c1, c2],
        }
    }

    pub fn random() -> Self {
        Self {
            coordinates: [fastrand::f64(), fastrand::f64(), fastrand::f64()],
        }
    }

    pub fn random_rng(min: f64, max: f64) -> Self {
        Self {
            coordinates: [
                (max - min) * fastrand::f64() + min,
                (max - min) * fastrand::f64() + min,
                (max - min) * fastrand::f64() + min,
            ],
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let vec = Self::random_rng(-1.0, 1.0);
            if vec.length_squared() < 1.0 {
                return vec;
            }
        }
    }

    pub fn random_unit() -> Self {
        Self::random_in_unit_sphere().to_unit()
    }

    // Accessors

    pub fn x(&self) -> f64 {
        self[0]
    }
    pub fn y(&self) -> f64 {
        self[1]
    }
    pub fn z(&self) -> f64 {
        self[2]
    }

    // Meta

    pub fn length_squared(&self) -> f64 {
        self.coordinates.iter().fold(0f64, |acc, &x| acc + x * x)
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    // Operations

    pub fn to_unit(&self) -> Self {
        let mut unit = *self;
        unit /= unit.length();
        unit
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.coordinates
            .iter()
            .zip(other.coordinates.iter())
            .fold(0.0, |acc, (&s, &o)| acc + s * o)
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            coordinates: [
                self[1] * other[2] - self[2] * other[1],
                self[2] * other[0] - self[0] * other[2],
                self[0] * other[1] - self[1] * other[0],
            ],
        }
    }
}

// Indexing

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.coordinates[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.coordinates[index]
    }
}

// Arithmetic operations

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            coordinates: [-self[0], -self[1], -self[2]],
        }
    }
}

impl AddAssign<Self> for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        for (c, rc) in self.coordinates.iter_mut().zip(rhs.coordinates.iter()) {
            *c += *rc;
        }
    }
}

impl Add<Self> for Vec3 {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl SubAssign<Self> for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs;
    }
}

impl Sub<Self> for Vec3 {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self += -rhs;
        self
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, scalar: f64) {
        self.coordinates.iter_mut().for_each(|x| *x *= scalar);
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(mut self, scalar: f64) -> Self::Output {
        self *= scalar;
        self
    }
}

impl Mul<Self> for Vec3 {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        for (c, rc) in self.coordinates.iter_mut().zip(rhs.coordinates.iter()) {
            *c *= *rc;
        }
        self
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, scalar: f64) {
        *self *= 1f64 / scalar;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(mut self, scalar: f64) -> Self::Output {
        self /= scalar;
        self
    }
}

// Utility

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self[0], self[1], self[2])
    }
}

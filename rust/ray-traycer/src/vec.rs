use std::fmt::{Display, Formatter};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

// TODO(TmLev): generalise over inner type and size, i.e. Vec<5, u8>
#[derive(Clone, Copy)]
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
}

// Default constructor

impl Default for Vec3 {
    fn default() -> Self {
        Self {
            coordinates: Default::default(),
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

    fn div(mut self, rhs: f64) -> Self::Output {
        self /= rhs;
        self
    }
}

// Utility

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self[0], self[1], self[2])
    }
}

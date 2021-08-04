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

    pub fn random_unit() -> Self {
        Self::random_in_unit_sphere().to_unit()
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let vec = Self {
                coordinates: [
                    2.0 * fastrand::f64() - 1.0,
                    2.0 * fastrand::f64() - 1.0,
                    0.0,
                ],
            };
            if vec.length_squared() < 1.0 {
                return vec;
            }
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

    pub fn near_zero(&self) -> bool {
        const EPS: f64 = 1e-8;
        self[0].abs() < EPS && self[1].abs() < EPS && self[2].abs() < EPS
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

    pub fn reflect(&self, normal: &Self) -> Self {
        *self - 2.0 * self.dot(normal) * normal
    }

    pub fn refract(&self, normal: &Self, etai_over_etat: f64) -> Self {
        let cos_theta = (-self).dot(normal).min(1.0);
        let perpendicular = etai_over_etat * (self + cos_theta * normal);
        let parallel = -(1.0 - perpendicular.length_squared()).abs().sqrt() * normal;
        perpendicular + parallel
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

// Utility

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self[0], self[1], self[2])
    }
}

// Arithmetic operations

macro_rules! impl_bin_ops {
    // `impl Add<(Vec or f64)> for (f64 or Vec)`
    // with all possible combinations of references.
    ($Vec:ident $Op:ident $op_fn:ident $op_symbol:tt) => {
        // Base implementation for Vec + Vec.
        // Implementations below forward operations here.
        // e.g. a + b,
        // where
        //     a: &Vec,
        //     b: &Vec,
        impl<'a, 'other> $Op<&'other $Vec> for &'a $Vec {
            type Output = $Vec;

            fn $op_fn(self, other: &'other $Vec) -> Self::Output {
                $Vec {
                    coordinates: [
                        self[0] $op_symbol other[0],
                        self[1] $op_symbol other[1],
                        self[2] $op_symbol other[2],
                    ],
                }
            }
        }

        // e.g. a + b,
        // where
        //     a: Vec,
        //     b: &Vec,
        impl<'other> $Op<&'other $Vec> for $Vec {
            type Output = $Vec;

            fn $op_fn(self, other: &'other $Vec) -> Self::Output {
                &self $op_symbol other
            }
        }

        // e.g. a + b,
        // where
        //     a: &Vec,
        //     b: Vec,
        impl<'a> $Op<$Vec> for &'a $Vec {
            type Output = $Vec;

            fn $op_fn(self, other: $Vec) -> Self::Output {
                self $op_symbol &other
            }
        }

        // e.g. a + b,
        // where
        //     a: Vec,
        //     b: Vec,
        impl $Op<$Vec> for $Vec {
            type Output = $Vec;

            fn $op_fn(self, other: $Vec) -> Self::Output {
                &self $op_symbol &other
            }
        }

        // Base implementation for (Vec or f64) + (f64 or Vec).
        // Implementations below forward operations here.
        // e.g. a + b,
        // where
        //     a: &Vec,
        //     b: f64,
        impl<'a> $Op<f64> for &'a $Vec {
            type Output = $Vec;

            fn $op_fn(self, scalar: f64) -> Self::Output {
                $Vec {
                    coordinates: [
                        self[0] $op_symbol scalar,
                        self[1] $op_symbol scalar,
                        self[2] $op_symbol scalar,
                    ],
                }
            }
        }

        // e.g. a + b,
        // where
        //     a: f64,
        //     b: &Vec,
        impl<'other> $Op<&'other $Vec> for f64 {
            type Output = $Vec;

            fn $op_fn(self, other: &'other $Vec) -> Self::Output {
                other $op_symbol self
            }
        }

        // e.g. a + b,
        // where
        //     a: Vec,
        //     b: f64,
        impl $Op<f64> for $Vec {
            type Output = $Vec;

            fn $op_fn(self, scalar: f64) -> Self::Output {
                &self $op_symbol scalar
            }
        }

        // e.g. a + b,
        // where
        //     a: f64,
        //     b: Vec,
        impl $Op<$Vec> for f64 {
            type Output = $Vec;

            fn $op_fn(self, other: $Vec) -> Self::Output {
                &other $op_symbol self
            }
        }
    };
}

macro_rules! impl_assign_ops {
    // `impl AddAssign<Vec> for Vec`
    // with all possible combinations of references.
    ($Vec:ident $Op:ident $op_fn:ident $op_symbol:tt) => {
        // Base implementation for Vec += Vec.
        // Implementations below forward operations here.
        // e.g. a += b,
        // where
        //     a: &mut Vec,
        //     b: &Vec,
        impl<'other> $Op<&'other $Vec> for $Vec {
            fn $op_fn(&mut self, other: &'other $Vec) {
                *self = $Vec {
                    coordinates: [
                        self[0] $op_symbol other[0],
                        self[1] $op_symbol other[1],
                        self[2] $op_symbol other[2],
                    ],
                }
            }
        }

        // e.g. a += b,
        // where
        //     a: &mut Vec,
        //     b: Vec,
        impl $Op for $Vec {
            fn $op_fn(&mut self, other: $Vec) {
                *self = *self $op_symbol other
            }
        }

        // e.g. a += b,
        // where
        //     a: &mut Vec,
        //     b: f64,
        impl $Op<f64> for $Vec {
            fn $op_fn(&mut self, other: f64) {
                *self = *self $op_symbol other
            }
        }
    };
}

macro_rules! impl_un_ops {
    // `impl Neg for Vec`
    // with all possible combinations of references.
    ($Vec:ident $Op:ident $op_fn:ident $op_symbol:tt) => {
        // Base implementation for -Vec.
        // Implementations below forward operations here.
        // e.g. -a
        // where
        //     a: &Vec,
        impl<'a> $Op for &'a $Vec {
            type Output = $Vec;

            fn $op_fn(self) -> Self::Output {
                $Vec {
                    coordinates: [
                        $op_symbol self[0],
                        $op_symbol self[1],
                        $op_symbol self[2],
                    ],
                }
            }
        }

        // e.g. -a
        // where
        //     a: Vec,
        impl $Op for $Vec {
            type Output = $Vec;

            fn $op_fn(self) -> Self::Output {
                $op_symbol &self
            }
        }
    };
}

impl_bin_ops!(Vec3 Add add +);
impl_bin_ops!(Vec3 Sub sub -);
impl_bin_ops!(Vec3 Mul mul *);
impl_bin_ops!(Vec3 Div div /);

impl_assign_ops!(Vec3 AddAssign add_assign +);
impl_assign_ops!(Vec3 SubAssign sub_assign -);
impl_assign_ops!(Vec3 MulAssign mul_assign *);
impl_assign_ops!(Vec3 DivAssign div_assign /);

impl_un_ops!(Vec3 Neg neg -);

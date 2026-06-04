use std::ops::Add;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;

use nalgebra::Const;
use nalgebra::OPoint;
use nalgebra::Point;
use nalgebra::Vector3;

use num_traits::Zero;
use std::fmt::Debug;

// V is  Vec3 or DVec
#[derive(Debug)]
pub struct Open3DData<REAL, const D: usize>
where
    REAL: 'static + Clone + std::fmt::Debug + Default + PartialEq,
{
    pub normal: Point<REAL, D>,
    pub color: Point<REAL, D>,
}

impl<REAL> From<(Vector3<REAL>, Vector3<REAL>)> for Open3DData<REAL, 3>
where
    REAL: Clone + std::fmt::Debug + Default + PartialEq + Zero,
{
    fn from(values: (Vector3<REAL>, Vector3<REAL>)) -> Self {
        Self {
            normal: values.0.into(),
            color: values.1.into(),
        }
    }
}

impl<REAL, const D: usize> Default for Open3DData<REAL, D>
where
    REAL: Clone + std::fmt::Debug + Default + PartialEq + Zero,
{
    fn default() -> Self {
        Self {
            normal: Point::default(),
            color: Point::default(),
        }
    }
}

// F: f32 or f64
impl<REAL, const D: usize> Mul<REAL> for Open3DData<REAL, D>
where
    REAL: Clone + std::fmt::Debug + Default + PartialEq + Mul<Self>,
{
    type Output = Self;

    fn mul(self, s: REAL) -> Self::Output {
        // let normal = s * self.normal;
        // let color = s * self.color;
        // Self { normal, color }
        todo!();
    }
}

impl<const D: usize> Div<f64> for Open3DData<f64, D> {
    type Output = Self;

    fn div(self, d: f64) -> Self::Output {
        let normal = self.normal / d;
        let color = self.color / d;
        Self { normal, color }
    }
}

impl<REAL, const D: usize> MulAssign<REAL> for Open3DData<REAL, D>
where
    REAL: Clone + std::fmt::Debug + Default + PartialEq,
{
    fn mul_assign(&mut self, rhs: REAL) {
        todo!()
    }
}
impl<const D: usize> Add<Self> for Open3DData<f64, D>
where
    Point<f64, D>: Add<Point<f64, D>>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        todo!();
    }
}

impl<REAL, const D: usize> Open3DData<REAL, D>
where
    REAL: Clone + std::fmt::Debug + Default + PartialEq,
{
    const fn new<S>(normal: Point<REAL, D>, color: Point<REAL, D>) -> Self {
        Self { normal, color }
    }
}

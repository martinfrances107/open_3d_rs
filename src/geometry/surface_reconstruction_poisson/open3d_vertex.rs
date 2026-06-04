use nalgebra::Point;
use nalgebra::SVector;
use nalgebra::Vector;

struct Open3DVertex<REAL>
where
    REAL: 'static + Clone + std::fmt::Debug + PartialEq,
{
    point: Point<REAL, 3>,
    normal: Option<SVector<REAL, 3>>,
    color: Option<SVector<REAL, 3>>,
    w: f64,
}

impl<REAL> Open3DVertex<REAL>
where
    REAL: 'static + Clone + std::fmt::Debug + PartialEq,
{
    const fn new(point: Point<REAL, 3>) -> Self {
        Self {
            point,
            normal: None,
            color: None,
            w: 0_f64,
        }
    }
}

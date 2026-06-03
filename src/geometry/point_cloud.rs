use enum_map::Enum;
use nalgebra::Matrix4;
use nalgebra::Vector3;

use crate::geometry::Geometry3DTrait;
use crate::geometry::GeometryTrait;

#[derive(Debug, Enum, Hash, PartialEq, Eq)]
pub enum PointCloudKey {
    Color,
    Position,
    Normal,
}

#[derive(Debug, Hash, PartialEq)]
struct Tensor();

#[derive(Debug, Default, PartialEq, Eq)]
pub struct PointCloud<REAL> {
    pub points: Vec<Vector3<REAL>>,
    pub normals: Vec<Vector3<REAL>>,
    pub colors: Vec<Vector3<REAL>>,
    pub covariances: Vec<Matrix4<REAL>>,
}

impl<REAL> PointCloud<REAL> {
    #[must_use]
    pub const fn has_normals(&self) -> bool {
        !self.normals.is_empty()
    }

    #[must_use]
    pub const fn has_colors(&self) -> bool {
        !self.colors.is_empty()
    }
}

impl<REAL> GeometryTrait for PointCloud<REAL>
where
    REAL: 'static + Clone + std::fmt::Debug + PartialEq,
{
    fn is_empty(&self) -> bool {
        todo!()
    }

    fn clear(&self) -> Self {
        todo!()
    }

    fn geometry_type(&self) -> super::GeometryType {
        todo!()
    }

    fn dimension(&self) -> usize {
        todo!()
    }

    fn name(&self) -> String {
        todo!()
    }
}
impl<REAL> Geometry3DTrait<REAL> for PointCloud<REAL>
where
    REAL: 'static + Clone + std::fmt::Debug + PartialEq,
{
    fn min_bounds(&self) -> usize {
        todo!()
    }

    fn max_bounds(&self) -> usize {
        todo!()
    }

    fn axis_aligned_bounding_box(&self) -> super::AxisAlignedBoudingBox {
        todo!()
    }

    fn oriented_bounding_box(&self) -> super::OrientedBoundBox {
        todo!()
    }

    fn oriented_bounding_box_with_robust(
        &self,
        robust: bool,
    ) -> super::OrientedBoundBox {
        todo!()
    }

    fn minimal_oriented_bounding_box(&self) -> super::OrientedBoundBox {
        todo!()
    }

    fn transform(transformation: Matrix4<REAL>) {
        todo!()
    }

    fn translate_with_relative(
        translation: Vector3<REAL>,
        relative: super::Relative,
    ) {
        todo!()
    }

    fn scale(scale: f64, center: Vector3<REAL>) {
        todo!()
    }

    fn rotate(rotate: nalgebra::Matrix3<REAL>) {
        todo!()
    }

    fn rotate_with_center(r: nalgebra::Matrix3<REAL>, center: Vector3<REAL>) {
        todo!()
    }

    fn center<const DIM: usize>(&self) -> nalgebra::Point<REAL, DIM> {
        todo!()
    }

    fn get_center() -> nalgebra::Point3<REAL> {
        todo!()
    }
}

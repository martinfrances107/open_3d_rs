use nalgebra::Point3;
use nalgebra::Vector3;

use crate::geometry::Geometry;
use crate::geometry::Geometry3DTrait;
use crate::geometry::GeometryTrait;

pub trait MeshBaseTrait<REAL>: Geometry3DTrait<REAL>
where
    REAL: 'static + Clone + std::fmt::Debug + PartialEq,
{
}

struct MeshBase<REAL> {
    verticies: Vec<Vector3<REAL>>,
    normals: Vec<Vector3<REAL>>,
    vertex_colors: Vec<Vector3<REAL>>,
}

impl<REAL: 'static + Clone + std::fmt::Debug + PartialEq> MeshBaseTrait<REAL>
    for MeshBase<REAL>
{
}

impl<REAL> GeometryTrait for MeshBase<REAL> {
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
impl<REAL> Geometry3DTrait<REAL> for MeshBase<REAL>
where
    REAL: 'static + Clone + std::fmt::Debug + PartialEq,
{
    fn get_center() -> Point3<REAL> {
        todo!()
    }

    fn min_bounds(&self) -> usize {
        todo!()
    }

    fn max_bounds(&self) -> usize {
        todo!()
    }

    fn center<const DIM: usize>(&self) -> nalgebra::Point<REAL, DIM> {
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

    fn transform(transformation: nalgebra::Matrix4<REAL>) {
        todo!()
    }

    fn translate_with_relative(
        translation: nalgebra::Vector3<REAL>,
        relative: super::Relative,
    ) {
        todo!()
    }

    fn scale(scale: f64, center: nalgebra::Vector3<REAL>) {
        todo!()
    }

    fn rotate(rotate: nalgebra::Matrix3<REAL>) {
        todo!()
    }

    fn rotate_with_center(
        r: nalgebra::Matrix3<REAL>,
        center: nalgebra::Vector3<REAL>,
    ) {
        todo!()
    }

    fn translate(translation: Vector3<REAL>) {
        Self::translate_with_relative(translation, super::Relative::True);
    }
}

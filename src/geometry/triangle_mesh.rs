use std::collections::HashSet;

use log::error;
use nalgebra::SVector;
use nalgebra::Vector;
use nalgebra::Vector3;

use crate::geometry::Geometry3DTrait;
use crate::geometry::GeometryTrait;
use crate::geometry::mesh_base::MeshBaseTrait;
use crate::geometry::point_cloud::PointCloud;

#[derive(Debug)]
pub struct Mesh<REAL, const N: usize>([Vector3<REAL>; N]);

#[derive(Debug, Default)]
pub struct TriangleMesh<REAL> {
    triangles: Vector3<i32>,
    triangle_normals: Vec<Vector3<REAL>>,
    adjacency_list: HashSet<i32>,
    triangle_uvs: Vec<Vector3<REAL>>,
}

impl<REAL: 'static + Clone + std::fmt::Debug + PartialEq> MeshBaseTrait<REAL>
    for TriangleMesh<REAL>
{
}

impl<REAL> GeometryTrait for TriangleMesh<REAL>
where
    REAL: Clone + std::fmt::Debug + PartialEq,
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
impl<REAL> Geometry3DTrait<REAL> for TriangleMesh<REAL>
where
    REAL: 'static + Clone + std::fmt::Debug + PartialEq,
{
    fn get_center() -> nalgebra::Point3<REAL> {
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

    fn translate(translation: Vector3<REAL>) {
        Self::translate_with_relative(translation, super::Relative::True);
    }
}

///
/// # Errors
/// When the point cloud is empty.
///
fn create_from_point_cloud_poisson<REAL, const D: usize>(
    pcd: &PointCloud<REAL>,
) -> Result<(Mesh<REAL, D>, SVector<REAL, D>), bool> {
    if !pcd.has_normals() {
        error!("Point cloud has no normals");
    }

    todo!();
    // let mesh = [Vec3::default();N];
    // (mesh, densities)
}

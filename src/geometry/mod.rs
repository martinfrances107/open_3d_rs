pub mod mesh_base;
pub mod point_cloud;
pub mod surface_reconstruction_poisson;
pub mod triangle_mesh;

use std::borrow::Cow;
use std::marker::PhantomData;

use nalgebra::Matrix;
use nalgebra::Matrix3;
use nalgebra::Matrix4;
use nalgebra::Point;
use nalgebra::Point3;
use nalgebra::Vector3;

#[derive(Debug)]
pub struct AxisAlignedBoudingBox;
#[derive(Debug)]
pub struct OrientedBoundBox;
#[derive(Debug)]
pub struct OrientedBoundingEllipsoid;

// In Cpp version Geomtry3D inherits on Geomtry.
// Geomtry3D: Geomtry
//
// In rust use traits, for methods
// class poperties must have hand tuned.

// In cpp version GeomtryType is a private type inside a base Geometry class.
//
#[derive(Debug)]
pub enum GeometryType {
    // Unspecified geometry type.
    Unspecified = 0,
    // PointCloud
    PointCloud = 1,
    // VoxelGrid
    VoxelGrid = 2,
    // Octree
    Octree = 3,
    // LineSet
    LineSet = 4,
    // MeshBase
    MeshBase = 5,
    // TriangleMesh
    TriangleMesh = 6,
    // HalfEdgeTriangleMesh
    HalfEdgeTriangleMesh = 7,
    // Image
    Image = 8,
    // RGBDImage
    RGBDImage = 9,
    // TetraMesh
    TetraMesh = 10,
    // OrientedBoundingBox
    OrientedBoundingBox = 11,
    // AxisAlignedBoundingBox
    AxisAlignedBoundingBox = 12,
    // OrientedBoundingEllipsoid
    OrientedBoundingEllipsoid = 13,
}

pub trait GeometryTrait {
    fn is_empty(&self) -> bool;
    fn clear(&self) -> Self;
    fn geometry_type(&self) -> GeometryType;
    fn dimension(&self) -> usize;
    fn name(&self) -> String;
    // private method geometry constructor(type, dim)?
}
pub trait Geometry3DTrait<REAL>: GeometryTrait
where
    REAL: 'static + Clone + std::fmt::Debug + PartialEq,
{
    fn get_center() -> Point3<REAL>;

    fn min_bounds(&self) -> usize;
    fn max_bounds(&self) -> usize;
    fn center<const DIM: usize>(&self) -> Point<REAL, DIM>;
    fn axis_aligned_bounding_box(&self) -> AxisAlignedBoudingBox;
    fn oriented_bounding_box(&self) -> OrientedBoundBox;
    fn oriented_bounding_box_with_robust(
        &self,
        robust: bool,
    ) -> OrientedBoundBox;
    fn minimal_oriented_bounding_box(&self) -> OrientedBoundBox;
    fn transform(transformation: Matrix4<REAL>);
    fn translate_with_relative(translation: Vector3<REAL>, relative: Relative);
    fn translate(translation: Vector3<REAL>) {
        Self::translate_with_relative(translation, Relative::True);
    }

    fn scale(scale: f64, center: Vector3<REAL>);
    fn rotate(rotate: Matrix3<REAL>);
    fn rotate_with_center(r: Matrix3<REAL>, center: Vector3<REAL>);

    // TODO add many more.
}

#[derive(Debug)]
pub enum Relative {
    True,
    False,
}

struct Geometry<'a> {
    geometry_type: GeometryType,
    dimension: u8,
    name: Option<Cow<'a, str>>,
}

impl Default for Geometry<'_> {
    fn default() -> Self {
        Self {
            dimension: 3,
            name: None,
            geometry_type: GeometryType::Unspecified,
        }
    }
}
impl Geometry<'_> {
    const fn new(geometry_type: GeometryType, dimension: u8) -> Self {
        Self {
            geometry_type,
            dimension,
            name: None,
        }
    }
}
impl GeometryTrait for Geometry<'_> {
    fn is_empty(&self) -> bool {
        todo!()
    }

    fn clear(&self) -> Self {
        todo!()
    }

    fn geometry_type(&self) -> GeometryType {
        todo!()
    }

    fn dimension(&self) -> usize {
        todo!()
    }

    fn name(&self) -> String {
        todo!()
    }
}
impl<'a, REAL: 'static + Clone + std::fmt::Debug + PartialEq>
    Geometry3DTrait<REAL> for Geometry<'a>
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

    fn center<const DIM: usize>(&self) -> Point<REAL, DIM> {
        todo!()
    }

    fn axis_aligned_bounding_box(&self) -> AxisAlignedBoudingBox {
        todo!()
    }

    fn oriented_bounding_box(&self) -> OrientedBoundBox {
        todo!()
    }

    fn oriented_bounding_box_with_robust(
        &self,
        robust: bool,
    ) -> OrientedBoundBox {
        todo!()
    }

    fn minimal_oriented_bounding_box(&self) -> OrientedBoundBox {
        todo!()
    }

    fn transform(transformation: Matrix4<REAL>) {
        todo!()
    }

    fn translate_with_relative(translation: Vector3<REAL>, relative: Relative) {
        todo!()
    }

    fn scale(scale: f64, center: Vector3<REAL>) {
        todo!()
    }

    fn rotate(rotate: Matrix3<REAL>) {
        todo!()
    }

    fn rotate_with_center(r: Matrix3<REAL>, center: Vector3<REAL>) {
        todo!()
    }
}

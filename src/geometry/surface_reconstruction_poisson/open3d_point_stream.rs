use nalgebra::Point;
use num_traits::Zero;

use super::DIMENSION;
use crate::geometry::surface_reconstruction_poisson::PointCloud;
use crate::geometry::surface_reconstruction_poisson::open3d_data::Open3DData;
use crate::geometry::surface_reconstruction_poisson::xform::XForm;

// #[derive(Debug, Default)]
// pub struct InputPointStreamWidthData<REAL: Copy, const DIM: usize> {
//     pcd: PointCloud,
//     xform: XForm<REAL, DIM>,
//     current: usize,
// }

/// which is a third party device like the XBOX scanner.
// CPP version depends on InputPointStreamWithData
#[derive(Debug)]
pub struct Open3DPointStream<REAL>
where
    REAL: 'static + Clone + Copy + std::fmt::Debug + PartialEq + Zero,
{
    pcd: PointCloud<REAL>,
    pub xform: Option<XForm<REAL, 4>>,
    current: usize,
}

// #[derive(Debug)]
// struct Point<REAL: Copy, const N: usize>([REAL; N]);

// impl<REAL: Copy + Default, const N: usize> std::default::Default
//     for Point<REAL, N>
// {
//     fn default() -> Self {
//         Self([Default::default(); N])
//     }
// }

// impl<REAL: Copy, const N: usize> Point<REAL, N> {
//     fn add_column_vector() {
//         todo!();
//     }
// }

// impl<REAL: Copy, const N: usize> Subtract for Point<REAL, N> {
//     fn add_column_vector() {
//         todo!();
//     }
// }

impl<REAL> Open3DPointStream<REAL>
where
    REAL: Copy + std::fmt::Debug + Default + PartialEq + num_traits::Zero,
{
    pub const fn new(pcd: PointCloud<REAL>) -> Self {
        Self {
            pcd,
            xform: None,
            current: 0,
        }
    }

    const fn reset(&mut self) {
        self.current = 0;
    }

    fn next_point<const D: usize>(
        &mut self,
        p: &mut Point<REAL, 3>,
        d: &Open3DData<REAL, D>,
    ) -> bool {
        if self.current >= self.pcd.points.len() {
            return false;
        }

        // p.0[0] = self[self.current].x;

        // if self.pcd.has_normals() {
        //     todo!();
        // } else {
        //     todo!();
        // }

        // if self.pcd.has_colors() {
        //     todo!();
        // } else {
        //     todo!();
        // }

        self.current += 1;
        true
    }
}

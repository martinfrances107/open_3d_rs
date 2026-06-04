use std::sync::Arc;
use std::time::SystemTime;

use log::error;

// use crate::geometry::GeometryType::TriangleMesh;
use crate::geometry::point_cloud::PointCloud;
use crate::geometry::surface_reconstruction_poisson::open3d_data::Open3DData;
use crate::geometry::surface_reconstruction_poisson::open3d_point_stream::Open3DPointStream;
use crate::geometry::surface_reconstruction_poisson::xform::XForm;
use crate::geometry::triangle_mesh::Mesh;
use crate::geometry::triangle_mesh::TriangleMesh;

// The order of the B-Spline used to splat in data for color interpolation
const DATA_DEGREE: i16 = 0;
// The order of the B-Spline used to splat in the weights for density estimation
const WEIGHT_DEGREE: i32 = 2;
// The order of the B-Spline used to splat in the normals for constructing the
// Laplacian constraints
const NORMAL_DEGREE: i32 = 2;
// The default finite-element degree
const DEFAULT_FEM_DEGREE: i32 = 1;

// The dimension of the system
const DIMENSION: usize = 3;

pub mod extract_mesh;
pub mod open3d_data;
pub mod open3d_point_stream;
pub mod open3d_vertex;
pub mod xform;

#[derive(Debug)]
pub struct NoNormalError;

#[derive(Debug)]
pub enum Fit {
    Linear,
    NonLinear,
}

#[derive(Debug)]
pub struct PointCloudPoissonOptions {
    pub depth: usize,
    pub width: f32,
    pub scale: f32,
    pub fit: Fit,
    pub n_thread: i8,
}

impl Default for PointCloudPoissonOptions {
    fn default() -> Self {
        Self {
            depth: 8usize,
            width: 0_f32,
            scale: 1.1_f32,
            fit: Fit::NonLinear,
            n_thread: -1,
        }
    }
}

///
/// # Errors
/// When the point cloud has no normals.
///
pub fn create_from_point_cloud_poisson<REAL, MESH>(
    pcd: &PointCloud<REAL>,
) -> Result<(Arc<MESH>, Vec<f64>), NoNormalError>
where
    MESH: Default,
    REAL: 'static + Clone + Copy + std::fmt::Debug + Default + PartialEq,
{
    let options = PointCloudPoissonOptions::default();
    create_from_point_cloud_poisson_with_options(pcd, &options)
}

///
/// # Errors
/// When the point cloud has no normals.
///
pub fn create_from_point_cloud_poisson_with_options<REAL, MESH>(
    pcd: &PointCloud<REAL>,
    options: &PointCloudPoissonOptions,
) -> Result<(Arc<MESH>, Vec<f64>), NoNormalError>
where
    MESH: Default,
    REAL: 'static + Clone + Copy + std::fmt::Debug + Default + PartialEq,
{
    let PointCloudPoissonOptions {
        depth,
        width,
        scale,
        fit,
        n_thread,
    } = options;
    if !pcd.has_normals() {
        error!("Point clould has no normals");
        return Err(NoNormalError);
    }

    let mesh = Arc::new(MESH::default());
    let densities = vec![];

    execute(pcd, &mesh, &densities, *depth, *width, *scale, *fit);
    Ok((mesh, densities))
}

// IF F - f32 V must be Vec3
// if F - f64 V must be DVec3
fn execute<REAL, const D: usize, SAMPLEDATA, FEMSIGNS>(
    pcd: &PointCloud<REAL>,
    out_mesh: &Arc<TriangleMesh<REAL>>,
    out_densities: &[f64],
    depth: usize,
    width: f32,
    scale: f32,
    fit: Fit,
    // TODO what about
    //  UIntPack<FEMSigs...>
) where
    REAL: 'static + Clone + Copy + std::fmt::Debug + Default + PartialEq,
{
    let datax = 32_f32;
    let base_depth = 0;
    let base_v_cycles = 1;
    let confidence = 0_f32;
    let point_weight = 2_f64 * f64::from(DEFAULT_FEM_DEGREE);
    let confidence_bias = 0_f32;
    let samples_per_node: f32 = 1.5_f32;
    let cg_solver_accuracy = 1e-3f32;
    let full_depth = 5;
    let iters = 8;
    let exact_interpolation = false;
    let iso_value = 0;

    let start_time = SystemTime::now();
    //todo setup tree and profiler.

    let point_count: usize;
    let point_weight_sum = 0;

    let smaple_data: Vec<Open3DData<REAL, D>>;
    // TODO density estimatior;
    // SparseNodeData
    //let normal_info = None;
    // TODO Real targetValue = (Real)0.5;
    let terget_value = 0.5_f32;

    {
        let point_stream = Open3DPointStream::new(*pcd);
        if width > 0.0 {
            // let xform = XForm::get_point_x_form_with_scale(scale_factor)
            todo!();
        }
    }
}

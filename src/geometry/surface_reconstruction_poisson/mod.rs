use std::time::SystemTime;

use crate::geometry::point_cloud::PointCloud;
use crate::geometry::surface_reconstruction_poisson::open3d_data::Open3DData;
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

fn create_from_point_cloud_poisson<MESH, REAL>(
    pcd: &PointCloud<REAL>,
    depth: usize,
    width: f64,
    scale: f64,
    linear_fit: bool,
    n_thread: i32,
) -> (MESH, Vec<REAL>) {
    todo!();
}

// IF F - f32 V must be Vec3
// if F - f64 V must be DVec3
fn execute<REAL, const D: usize, SAMPLEDATA, FEMSIGNS>(
    pcd: &PointCloud<REAL>,
    out_mesh: &TriangleMesh<REAL>,
    out_densities: &[f64],
    depth: i32,
    width: f32,
    scale: f32,
    linear_fit: bool,
    // TODO what about
    //  UIntPack<FEMSigs...>
) where
    REAL: 'static + Clone + std::fmt::Debug + Default + PartialEq,
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
}

use std::path::Path;

use open_3d_rs::geometry::point_cloud::PointCloud;
use open_3d_rs::geometry::surface_reconstruction_poisson::PointCloudPoissonOptions;
use open_3d_rs::io::read_point_cloud_from_file;

// use std::path::Path;

/// Loads a eagle data set.
pub fn eagle_point_cloud(path: &Path) {
    todo!()
}

fn main() {
    let path = Path::new("../data/EaglePointCloud.ply");
    // let eagle = eagle_point_cloud(path);
    let mut pc = PointCloud::default();
    let options = PointCloudPoissonOptions {
        depth: 9,
        ..Default::default()
    };
    let success = read_point_cloud_from_file(&path, &mut pc);

    let options = PointCloudPoissonOptions {
        depth: 9,
        ..Default::default()
    };

    let create_from_point_cloud_poisson_with_options(pcd, options);
}

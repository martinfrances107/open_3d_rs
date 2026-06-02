use std::collections::HashMap;
use std::path::Path;
use std::sync::LazyLock;

use log::debug;
use log::warn;

use crate::geometry::point_cloud::PointCloud;

/// Load point cloud data from a source.
pub fn read_point_cloud(path: &Path) {
    todo!();
}

type ExtensionReaderFn<REAL> =
    fn(&Path, &mut PointCloud<REAL>, &ReadPointCloudOptions) -> bool;

// TODO make another global variable for f32?
static FILE_EXTENSION_TO_POINT_CLOUD_READ_FUNCTION_F64: LazyLock<
    HashMap<&'static str, ExtensionReaderFn<f64>>,
> = LazyLock::new(|| {
    let mut m: HashMap<&'static str, ExtensionReaderFn<f64>> = HashMap::new();
    // TODO add more reader here.
    // xyz,
    // pts
    m.insert("ply", read_point_cloud_from_plt);
    m
});

fn read_point_cloud_from_plt<REAL>(
    filename: &Path,
    point_cloud: &mut PointCloud<REAL>,
    options: &ReadPointCloudOptions,
) -> bool {
    todo!();
}
pub type Point<REAL, const DIM: usize> = [REAL; DIM];

#[derive(Clone, Default)]
struct ReadPointCloudOptions<'a> {
    format: &'a str,
    remove_nan_points: bool,
    remove_infiite_points: bool,
}

// impl <'a> ReadPointCloudOptions<'a> {
//     // TODO getters and setters here..
//     //
//     //
//     format: Cow<'a, str>,
// }

// TODO add formatter for ReadPointCloudOptions

pub fn read_point_cloud_from_file(
    filename: &Path,
    point_cloud: &mut PointCloud<f64>,
) -> bool {
    read_point_cloud_from_file_with_options(
        filename,
        point_cloud,
        &ReadPointCloudOptions::default(),
    )
}

fn read_point_cloud_from_file_with_options(
    filename: &Path,
    point_cloud: &mut PointCloud<f64>,
    params: &ReadPointCloudOptions,
) -> bool {
    let format = params.format;
    if format == "auto" {
        todo!("extension in to lower case");
    }
    debug!("Format {format} File {}", filename.display());

    let Some(map_iter) =
        FILE_EXTENSION_TO_POINT_CLOUD_READ_FUNCTION_F64.get(&format)
    else {
        warn!(
            "Read geometry: PointCloud failed unknown file extension for {} (format: {format}).",
            filename.display(),
        );
        return false;
    };

    let success = map_iter(filename, point_cloud, params);
    debug!(
        "Read geometry: Point Cloud {}, verticies",
        point_cloud.points.len()
    );

    if params.remove_nan_points || params.remove_infiite_points {
        // point_cloud.removed_non_finit_points(params.remove_nan_points, params.remove_infiite_points);
        unimplemented!("Must remove points.");
    }

    success
}

fn read_point_cloud_from_file_with_params<REAL>(
    filename: &Path,
    point_cloud: &mut PointCloud<REAL>,
    remove_nan_points: bool,
    remove_infinite_points: bool,
    print_progree: bool,
) -> bool {
    todo!();
}

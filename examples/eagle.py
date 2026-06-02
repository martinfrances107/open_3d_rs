import open3d as o3d

if __name__ == "__main__":
    # 1. Access the EaglePointCloud dataset (downloads automatically if missing)
    dataset = o3d.data.EaglePointCloud()

    # 2. Read the point cloud from the dataset path
    pcd = o3d.io.read_point_cloud(dataset.path)

    # 3. Visualize the point cloud
    o3d.visualization.draw(pcd)

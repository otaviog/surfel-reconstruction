use rstest::fixture;

use align3d::{pointcloud::PointCloud, transform::Transform, camera::PinholeCamera};

use super::{sample_range_img_ds1, TestRangeImageDataset};


pub struct TestPclDataset {
    dataset: TestRangeImageDataset,
}

impl TestPclDataset {
    pub fn get(&self, index: usize) -> PointCloud {
        let range_image = self
            .dataset
            .get(index)
            .expect("Error when loading range image to point cloud.");
        PointCloud::from(&range_image)
    }

    pub fn get_ground_truth(&self, source_index: usize, target_index: usize) -> Transform {
        self.dataset.get_ground_truth(source_index, target_index)
    }

    pub fn pinhole_camera(&self, index: usize) -> PinholeCamera {
        self.dataset.get_pinhole_camera(index)
    }
}

#[fixture]
pub fn sample_pcl_ds1() -> TestPclDataset {
    TestPclDataset {
        dataset: sample_range_img_ds1(),
    }
}

mod datasets;
mod point_clouds;
pub(crate) use point_clouds::{sample_pcl_ds1, TestPclDataset};
mod range_images;
pub(crate) use range_images::{sample_range_img_ds1, sample_range_img_ds2, TestRangeImageDataset};

use image::{flat::SampleLayout, RgbImage};
use nalgebra::Vector3;
use ndarray::{Array2, Array3, ShapeBuilder};

/// Trait to convert into ndarray::Array3, this is different than nshare version
/// because it uses the shape [height, width, channels] instead of [channels, height, width].
pub(crate) trait IntoArray3 {
    fn into_array3(self) -> Array3<u8>;
}

impl IntoArray3 for image::RgbImage {
    fn into_array3(self) -> Array3<u8> {
        let SampleLayout {
            channels,
            channel_stride,
            height,
            height_stride,
            width,
            width_stride,
        } = self.sample_layout();
        let shape = (height as usize, width as usize, channels as usize);
        let strides = (height_stride, width_stride, channel_stride);
        Array3::from_shape_vec(shape.strides(strides), self.into_raw()).unwrap()
    }
}

/// Trait to convert objects into image::RgbImage
pub trait IntoImageRgb8 {
    fn into_image_rgb8(self) -> RgbImage;
}

impl IntoImageRgb8 for Array3<u8> {
    fn into_image_rgb8(self) -> RgbImage {
        let (height, width, channels) = self.dim();
        if channels != 3 {
            panic!("Array3 must have 3 channels");
        }
        RgbImage::from_raw(width as u32, height as u32, self.into_raw_vec()).unwrap()
    }
}

pub trait ToImageRgb8 {
    fn to_image_rgb8(&self) -> RgbImage;
}

impl ToImageRgb8 for Array2<Vector3<u8>> {
    fn to_image_rgb8(&self) -> RgbImage {
        let (height, width) = self.dim();
        RgbImage::from_fn(width as u32, height as u32, |x, y| {
            let c = self[(y as usize, x as usize)];
            image::Rgb([c[0], c[1], c[2]])
        })
    }
}

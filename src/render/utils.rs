use bevy::{prelude::*, render::{render_resource::TextureFormat, texture::TextureFormatPixelInfo}};


pub fn set_f32_image_pixel_1d(
    im: &mut Image,
    index: usize,
    pixel: [f32; 4],
) {
    let px_size = TextureFormat::Rgba32Float.pixel_size();

    for i in 0..4 {
        let bytes = pixel[i].to_ne_bytes();
        for j in 0..4 {
            im.data[index * px_size + i * 4 + j] = bytes[j];
        }
    }
}
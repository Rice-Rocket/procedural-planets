use bevy::prelude::*;


use bevy::{window::PrimaryWindow, render::{render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages}, extract_resource::ExtractResource}};

use super::INIT_HEIGHTMAP_TEXTURE_SIZE;


pub fn setup_height_map_images(
    mut commands: Commands, 
    window_query: Query<&Window, With<PrimaryWindow>>, 
    mut images: ResMut<Assets<Image>>
) {
    let mut heightmap = Image::new_fill(
        Extent3d {
            width: INIT_HEIGHTMAP_TEXTURE_SIZE.0,
            height: INIT_HEIGHTMAP_TEXTURE_SIZE.1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0; 16],
        TextureFormat::Rgba32Float,
    );

    heightmap.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;
    let heightmap = images.add(heightmap);

    commands.insert_resource(PlanetHeightMapImages(heightmap, INIT_HEIGHTMAP_TEXTURE_SIZE));
}


#[derive(Resource, Clone, ExtractResource)]
pub struct PlanetHeightMapImages(pub Handle<Image>, pub (u32, u32));

impl PlanetHeightMapImages {
    pub fn fetch(&self, im: &Image, coord: [f32; 2]) -> f32 {
        let start = ((coord[1] * (self.1.0 - 1) as f32).round() as usize + coord[0].round() as usize) * 4;
        let end = start + 4;
        f32::from_ne_bytes(im.data[start..end].try_into().unwrap())
    }
    // pub fn fetch(&self, im: &Image, coord: [f32; 2], layer: usize) -> f32 {
    //     let start = ((layer * (self.1.1 - 1) as usize * (self.1.0 - 1) as usize) + (coord[1] * (self.1.0 - 1) as f32).round() as usize + coord[0].round() as usize) * 4;
    //     let end = start + 4;
    //     f32::from_ne_bytes(im.data[start..end].try_into().unwrap())
    // }
}
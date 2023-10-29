use bevy::{prelude::*, render::{render_resource::{ShaderType, StorageBuffer}, renderer::{RenderDevice, RenderQueue}}};
use bytemuck::{Pod, Zeroable};

use crate::gen::{shape::ShapeGenerator, compute::MAX_NOISE_LAYERS, noise::NoiseSimplex3d};



#[derive(Debug, Clone, Copy, ShaderType, Pod, Zeroable)]
#[repr(C, align(16))]
pub struct NoiseLayerStorage {
    pub simplex_random: [i32; NoiseSimplex3d::SIZE as usize * 2],
    pub filter_type: u32,

    pub num_octaves: i32,
    pub strength: f32,
    pub roughness: f32,
    pub lacunarity: f32,
    pub persistence: f32,
    pub offset: f32,
    pub floor: f32,

    pub center: Vec3,
    pub warp_target: i32,
    pub warp_offset: Vec3,
    pub first_layer_mask: i32,
}

impl Default for NoiseLayerStorage {
    fn default() -> Self {
        Self {
            simplex_random: [0; NoiseSimplex3d::SIZE as usize * 2],
            filter_type: 0,

            num_octaves: 0,
            strength: 0.0,
            roughness: 0.0,
            lacunarity: 0.0,
            persistence: 0.0,
            offset: 0.0,
            floor: 0.0,
            center: Vec3::ZERO,
            warp_offset: Vec3::ZERO,

            warp_target: 0,
            first_layer_mask: 0,
        }
    }
}


#[derive(Resource, Default)]
pub struct NoiseLayersBuffer {
    pub buffer: StorageBuffer<[NoiseLayerStorage; MAX_NOISE_LAYERS as usize]>,
    pub size: u64,
}


pub fn prepare_noise_layers_buffer(
    device: Res<RenderDevice>,
    queue: Res<RenderQueue>,
    mut noise_layers_buffer: ResMut<NoiseLayersBuffer>,
    shape_gen: Res<ShapeGenerator>,
) {
    let buf = noise_layers_buffer.buffer.get_mut();
    
    for i in 0..(shape_gen.num_layers as usize) {
        let layer = &mut buf[i];
        let shape_gen_layer = &shape_gen.noise_layers[i];

        layer.simplex_random = shape_gen_layer.filter.simplex_3d.random;
        layer.filter_type = shape_gen_layer.filter.ty.clone() as u32;

        layer.num_octaves = shape_gen_layer.filter.num_octaves;
        layer.strength = shape_gen_layer.filter.strength;
        layer.roughness = shape_gen_layer.filter.roughness;
        layer.lacunarity = shape_gen_layer.filter.lacunarity;
        layer.persistence = shape_gen_layer.filter.persistence;
        layer.offset = shape_gen_layer.filter.offset;
        layer.floor = shape_gen_layer.filter.floor;
        layer.center = shape_gen_layer.filter.center;
        layer.warp_offset = shape_gen_layer.filter.warp_offset;
        layer.warp_target = shape_gen_layer.warp_target as i32;
        layer.first_layer_mask = if shape_gen_layer.first_layer_mask { 1 } else { 0 };
    }

    noise_layers_buffer.buffer.write_buffer(&device, &queue);
}

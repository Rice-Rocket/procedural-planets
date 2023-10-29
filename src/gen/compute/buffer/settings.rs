use bevy::{prelude::*, render::{extract_resource::ExtractResource, render_resource::{ShaderType, UniformBuffer}, renderer::{RenderDevice, RenderQueue}}};

use crate::gen::compute::texture::PlanetHeightMapImages;



#[derive(Default, Clone, Resource, ExtractResource, Reflect, ShaderType)]
#[reflect(Resource)]
pub struct SettingsUniform {
    texture_size: IVec2,
}

#[derive(Resource, Default)]
pub struct SettingsBuffer {
    pub buffer: UniformBuffer<SettingsUniform>,
}

pub fn prepare_settings_buffer(
    device: Res<RenderDevice>,
    queue: Res<RenderQueue>,
    mut settings_buffer: ResMut<SettingsBuffer>,
    height_map_handles: Res<PlanetHeightMapImages>,
) {
    let buffer = settings_buffer.buffer.get_mut();
    buffer.texture_size = IVec2::new(height_map_handles.1.0 as i32, height_map_handles.1.1 as i32);

    settings_buffer.buffer.write_buffer(&device, &queue);
}
use std::borrow::Cow;

use bevy::{prelude::*, render::{render_resource::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindingResource, BindGroupLayout, CachedComputePipelineId, BindGroupLayoutDescriptor, BindGroupLayoutEntry, ShaderStages, BindingType, StorageTextureAccess, TextureFormat, TextureViewDimension, PipelineCache, ComputePipelineDescriptor, BufferBindingType}, render_asset::RenderAssets, renderer::RenderDevice}};

use super::{texture::PlanetHeightMapImages, buffer::SettingsBuffer};



#[derive(Resource)]
pub struct PlanetComputeBindGroup(pub BindGroup);

pub fn queue_bind_group(
    mut commands: Commands,
    pipeline: Res<PlanetComputePipeline>,
    gpu_images: Res<RenderAssets<Image>>,
    height_images: Res<PlanetHeightMapImages>,
    render_device: Res<RenderDevice>,

    settings_buf: Res<SettingsBuffer>,
) {
    let view_height_map_img = &gpu_images[&height_images.0];
    let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
        label: None,
        layout: &pipeline.bind_group_layout,
        entries: &[BindGroupEntry {
            binding: 0,
            resource: BindingResource::TextureView(&view_height_map_img.texture_view),
        }, BindGroupEntry {
            binding: 1,
            resource: settings_buf.buffer.binding().unwrap(),
        }],
    });
    commands.insert_resource(PlanetComputeBindGroup(bind_group));
}

#[derive(Resource)]
pub struct PlanetComputePipeline {
    pub bind_group_layout: BindGroupLayout,
    pub compute_pipeline: CachedComputePipelineId,
}

impl FromWorld for PlanetComputePipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let bind_group_layout =
            render_device
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::StorageTexture {
                            access: StorageTextureAccess::ReadWrite,
                            format: TextureFormat::Rgba32Float,
                            view_dimension: TextureViewDimension::D2,
                        },
                        count: None,
                    }, BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                });
        let shader = world
            .resource::<AssetServer>()
            .load("shaders/height.wgsl");
        let pipeline_cache = world.resource::<PipelineCache>();
        let compute_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: vec![bind_group_layout.clone()],
            push_constant_ranges: Vec::new(),
            shader: shader.clone(),
            shader_defs: vec![],
            entry_point: Cow::from("update"),
        });

        PlanetComputePipeline {
            bind_group_layout,
            compute_pipeline,
        }
    }
}
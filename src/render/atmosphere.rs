use bevy::{
    core_pipeline::{fullscreen_vertex_shader::fullscreen_shader_vertex_state, prepass::ViewPrepassTextures},
    prelude::*,
    render::{
        extract_component::{ComponentUniforms, ExtractComponent},
        render_graph::{NodeRunError, RenderGraphContext, ViewNode},
        render_resource::{
            BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
            BindGroupLayoutEntry, BindingResource, BindingType, CachedRenderPipelineId,
            ColorTargetState, ColorWrites, FragmentState, MultisampleState, Operations,
            PipelineCache, PrimitiveState, RenderPassColorAttachment, RenderPassDescriptor,
            RenderPipelineDescriptor, Sampler, SamplerBindingType, SamplerDescriptor, ShaderStages,
            ShaderType, TextureFormat, TextureSampleType, TextureViewDimension, TextureViewDescriptor, TextureAspect, BufferBindingType,
        },
        renderer::{RenderContext, RenderDevice},
        texture::BevyDefault,
        view::{ViewTarget, ViewUniforms, ViewUniform, ViewUniformOffset}
    },
    ecs::query::QueryItem, pbr::{GpuLights, LightMeta, ViewLightsUniformOffset},
};

use crate::ui::render::UiRenderSettings;


#[derive(Default)]
pub struct AtmospherePassPostProcessNode;
impl AtmospherePassPostProcessNode {
    pub const NAME: &str = "atmosphere_pass_post_process";
}

impl ViewNode for AtmospherePassPostProcessNode {
    type ViewQuery = (
        &'static ViewTarget, 
        &'static ViewPrepassTextures,
        bevy::ecs::system::lifetimeless::Read<ViewUniformOffset>,
        bevy::ecs::system::lifetimeless::Read<ViewLightsUniformOffset>
    );
    
    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        view_target: QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let post_process_pipeline = world.resource::<AtmospherePassPostProcessPipeline>();

        let pipeline_cache = world.resource::<PipelineCache>();

        let Some(pipeline) = pipeline_cache.get_render_pipeline(post_process_pipeline.pipeline_id) else {
            return Ok(());
        };

        let settings_uniforms = world.resource::<ComponentUniforms<AtmosphereSettings>>();
        let Some(settings_binding) = settings_uniforms.uniforms().binding() else {
            return Ok(());
        };

        let Some(depth_view) = view_target.1.depth.as_ref().map(|texture| texture.texture.create_view(&TextureViewDescriptor {
            aspect: TextureAspect::DepthOnly,
            ..default()
        })) else { return Ok(()); };

        let Some(view_binding) = world.resource::<ViewUniforms>().uniforms.binding() else {
            return Ok(());
        };

        let Some(lights_binding) = world.resource::<LightMeta>().view_gpu_lights.binding() else {
            return Ok(());
        };

        let post_process = view_target.0.post_process_write();

        let bind_group = render_context
            .render_device()
            .create_bind_group(&BindGroupDescriptor {
                label: Some("atmosphere_pass_post_process_bind_group"),
                layout: &post_process_pipeline.layout,
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: BindingResource::TextureView(post_process.source),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: BindingResource::TextureView(&depth_view),
                    },
                    BindGroupEntry {
                        binding: 2,
                        resource: BindingResource::Sampler(&post_process_pipeline.sampler),
                    },
                    BindGroupEntry {
                        binding: 3,
                        resource: settings_binding.clone(),
                    },
                    BindGroupEntry {
                        binding: 4,
                        resource: view_binding.clone(),
                    },
                    BindGroupEntry {
                        binding: 5,
                        resource: lights_binding.clone(),
                    },
                ],
            });

        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("atmosphere_pass_post_process_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: post_process.destination,
                resolve_target: None,
                ops: Operations::default(),
            })],
            depth_stencil_attachment: None,
        });

        render_pass.set_render_pipeline(pipeline);
        render_pass.set_bind_group(0, &bind_group, &[view_target.2.offset, view_target.3.offset]);
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}

#[derive(Resource)]
pub struct AtmospherePassPostProcessPipeline {
    layout: BindGroupLayout,
    sampler: Sampler,
    pipeline_id: CachedRenderPipelineId,
}

impl FromWorld for AtmospherePassPostProcessPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let layout = render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("atmosphere_pass_post_process_bind_group_layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Depth,
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 3,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: bevy::render::render_resource::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: Some(AtmosphereSettings::min_size()),
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 4,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: true,
                        min_binding_size: Some(ViewUniform::min_size()),
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 5,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: true,
                        min_binding_size: Some(GpuLights::min_size()),
                    },
                    count: None,
                },
            ],
        });

        let sampler = render_device.create_sampler(&SamplerDescriptor::default());

        let shader = world
            .resource::<AssetServer>()
            .load("shaders/atmosphere.wgsl");

        let pipeline_id = world
            .resource_mut::<PipelineCache>()
            .queue_render_pipeline(RenderPipelineDescriptor {
                label: Some("atmosphere_pass_post_process_pipeline".into()),
                layout: vec![layout.clone()],
                vertex: fullscreen_shader_vertex_state(),
                fragment: Some(FragmentState {
                    shader,
                    shader_defs: vec![],
                    entry_point: "fragment".into(),
                    targets: vec![Some(ColorTargetState {
                        format: TextureFormat::bevy_default(),
                        blend: None,
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                primitive: PrimitiveState::default(),
                depth_stencil: None,
                multisample: MultisampleState::default(),
                push_constant_ranges: vec![],
            });

        Self {
            layout,
            sampler,
            pipeline_id,
        }
    }
}


#[derive(Component, Debug, Clone, Copy, ExtractComponent, ShaderType, Reflect)]
#[reflect(Debug, Default)]
pub struct AtmosphereSettings {
    pub radius: f32,
    pub ocean_radius: f32,
    pub num_sample_points: u32,
    pub num_optical_depth_points: u32,
    pub density_falloff: f32,
    pub scattering_coeffs: Vec3,
}

impl Default for AtmosphereSettings {
    fn default() -> Self {
        Self {
            radius: 1.0,
            ocean_radius: 1.0,
            num_sample_points: 10,
            num_optical_depth_points: 10,
            density_falloff: 1.0,
            scattering_coeffs: Vec3::new(700.0, 530.0, 440.0),
        }
    }
}

pub fn update_atmosphere(
    mut atmospheres: Query<&mut AtmosphereSettings>,
    render_settings: Res<UiRenderSettings>,
) {
    for mut atmo in atmospheres.iter_mut() {
        atmo.radius = render_settings.atmosphere_radius;
        atmo.ocean_radius = render_settings.ocean_radius;
        atmo.num_sample_points = render_settings.atmosphere_sample_points;
        atmo.num_optical_depth_points = render_settings.atmosphere_optical_depth_points;
        atmo.density_falloff = render_settings.atmosphere_density_falloff;

        let mut scatter_coeffs = Vec3::from(render_settings.atmosphere_scatter_coeffs);
        scatter_coeffs = 400.0 / scatter_coeffs;
        scatter_coeffs = scatter_coeffs * scatter_coeffs * scatter_coeffs * scatter_coeffs;
        scatter_coeffs = scatter_coeffs * render_settings.atmosphere_scatter_strength;
        atmo.scattering_coeffs = scatter_coeffs;
    }
}
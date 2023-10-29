use bevy::{prelude::*, render::{extract_resource::{ExtractResourcePlugin, ExtractResource}, RenderApp, Render, render_graph::RenderGraph, RenderSet}};


use self::texture::PlanetHeightMapImages;

pub mod pipeline;
pub mod node;
pub mod texture;
pub mod buffer;

use pipeline::*;
use node::*;
use texture::*;
use buffer::*;

use super::shape::ShapeGenerator;


pub const INIT_HEIGHTMAP_TEXTURE_SIZE: (u32, u32) = (512, 512);
pub const WORKGROUP_SIZE: u32 = 8;
pub const MAX_NOISE_LAYERS: u32 = 16;


#[derive(ExtractResource, Resource, Default, Clone, PartialEq)]
pub struct PlanetComputeState(pub bool);


pub struct PlanetComputePlugin;

impl Plugin for PlanetComputePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlanetComputeState>();
        app.add_systems(Startup, setup_height_map_images);
        app.add_plugins(ExtractResourcePlugin::<PlanetComputeState>::default());
        app.add_plugins(ExtractResourcePlugin::<PlanetHeightMapImages>::default());
        app.add_plugins(ExtractResourcePlugin::<ShapeGenerator>::default());

        let render_app = app.sub_app_mut(RenderApp);
        render_app
            // .init_resource::<UISettings>()
            // .add_systems(ExtractSchedule, (extract_time, extract_ui_settings, extract_scene_data))
            .add_systems(Render, (
                prepare_noise_layers_buffer,
                prepare_settings_buffer,
            ).in_set(RenderSet::Prepare))
            .add_systems(Render, queue_bind_group.in_set(RenderSet::Queue));
        
        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();
        render_graph.add_node(PlanetComputeNode::NODE_NAME, PlanetComputeNode::default());
        render_graph.add_node_edge(
            PlanetComputeNode::NODE_NAME,
            bevy::render::main_graph::node::CAMERA_DRIVER,
        );
    }
    
    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<SettingsBuffer>();
        render_app.init_resource::<NoiseLayersBuffer>();
        render_app.init_resource::<PlanetComputePipeline>();
    }
}
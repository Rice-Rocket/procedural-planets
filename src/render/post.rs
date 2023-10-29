#![allow(dead_code)]

use bevy::{prelude::*, render::{extract_component::{ExtractComponentPlugin, UniformComponentPlugin}, RenderApp, render_graph::{ViewNodeRunner, RenderGraphApp}}, core_pipeline::core_3d};

use super::atmosphere::{AtmospherePassPostProcessPipeline, AtmosphereSettings, AtmospherePassPostProcessNode};


pub struct PostProcessPlugin;

impl Plugin for PostProcessPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ExtractComponentPlugin::<AtmosphereSettings>::default(),
            UniformComponentPlugin::<AtmosphereSettings>::default(),
        ));

        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_render_graph_node::<ViewNodeRunner<AtmospherePassPostProcessNode>>(
                core_3d::graph::NAME,
                AtmospherePassPostProcessNode::NAME,
            )
            .add_render_graph_edges(
                core_3d::graph::NAME,
                &[
                    core_3d::graph::node::TONEMAPPING,
                    AtmospherePassPostProcessNode::NAME,
                    core_3d::graph::node::END_MAIN_PASS_POST_PROCESSING,
                ],
            );
    }

    fn finish(&self, app: &mut App) {
        app.register_type::<AtmosphereSettings>();

        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .init_resource::<AtmospherePassPostProcessPipeline>();
    }
}
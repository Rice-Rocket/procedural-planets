use bevy::{prelude::*, render::{render_resource::{PipelineCache, ComputePassDescriptor}, render_graph, renderer::RenderContext}};

use super::{pipeline::{PlanetComputePipeline, PlanetComputeBindGroup}, texture::PlanetHeightMapImages, WORKGROUP_SIZE, PlanetComputeState};

enum ShaderState {
    Waiting,
    Update,
}

pub struct PlanetComputeNode {
    state: ShaderState,
}

impl PlanetComputeNode {
    pub const NODE_NAME: &'static str = "planet_compute";
}

impl Default for PlanetComputeNode {
    fn default() -> Self {
        Self {
            state: ShaderState::Update,
        }
    }
}

impl render_graph::Node for PlanetComputeNode {
    fn update(&mut self, world: &mut World) {
        if let Some(state) = world.get_resource::<PlanetComputeState>() {
            match self.state {
                ShaderState::Waiting => {
                    // if state.value {
                    //     println!("true");
                    //     self.state = ShaderState::Update;
                    // }
                }
                ShaderState::Update => {
                    // if !state.value {
                    //     println!("false");
                    //     self.state = ShaderState::Waiting;
                    // }
                }
            }
        }
    }

    fn run(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        let bind_group = &world.resource::<PlanetComputeBindGroup>().0;
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<PlanetComputePipeline>();
        let height_map_dims = world.resource::<PlanetHeightMapImages>().1;

        let encoder = render_context.command_encoder();
        {
            let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor::default());

            pass.set_bind_group(0, bind_group, &[]);

            match self.state {
                ShaderState::Waiting => {}
                ShaderState::Update => {
                    let update_pipeline = pipeline_cache
                        .get_compute_pipeline(pipeline.compute_pipeline)
                        .unwrap();
                    pass.set_pipeline(update_pipeline);
                    pass.dispatch_workgroups(height_map_dims.0 / WORKGROUP_SIZE, height_map_dims.1 / WORKGROUP_SIZE, 1);
                }
            }
        }

        Ok(())
    }
}
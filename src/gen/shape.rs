use bevy::{prelude::*, render::extract_resource::ExtractResource};
use serde::{Serialize, Deserialize};

use super::noise_filter::NoiseLayer;


#[derive(Resource, ExtractResource, Clone, Serialize, Deserialize)]
pub struct ShapeGenerator {
    pub radius: f32,
    pub sea_level: f32,
    pub num_layers: u32,
    pub noise_layers: Vec<NoiseLayer>,
}

impl Default for ShapeGenerator {
    fn default() -> Self {
        Self {
            radius: 1.0,
            sea_level: 1.0,
            num_layers: 1,
            noise_layers: vec![NoiseLayer::new(0, true)],
        }
    }
}

impl ShapeGenerator {
    pub fn get_point_and_elevation(&self, point_on_sphere: Vec3) -> (Vec3, f32) {
        let elevation = self.get_elevation(point_on_sphere);
        (point_on_sphere * elevation, elevation)
    }

    pub fn get_point(&self, point_on_sphere: Vec3) -> Vec3 {
        let elevation = self.get_elevation(point_on_sphere);
        point_on_sphere * elevation
    }

    pub fn get_elevation(&self, point_on_sphere: Vec3) -> f32 {
        let mut elevation = 0.0;
        let warp_targets: Vec<u32> = self.noise_layers.iter().map(|x| if x.is_warp && x.enabled { x.warp_target - 1 } else { self.num_layers }).collect();

        let first_layer = if warp_targets.contains(&0) {
            let warp_p = self.get_warped_pos(point_on_sphere, &self.noise_layers[warp_targets.iter().position(|x| *x == 0).unwrap()]);
            self.noise_layers[0].filter.evaluate(point_on_sphere + warp_p)
        } else {
            self.noise_layers[0].filter.evaluate(point_on_sphere)
        };
        if self.noise_layers[0].enabled {
            elevation = first_layer;
        }

        for i in 1..self.num_layers {
            let layer = &self.noise_layers[i as usize];
            if layer.enabled && !layer.is_warp {
                let mask = if layer.first_layer_mask { (first_layer - self.sea_level + 1.0).max(0.0) } else { 1.0 };
                let v = if warp_targets.contains(&i) {
                    let warp_p = self.get_warped_pos(point_on_sphere, &self.noise_layers[warp_targets.iter().position(|x| *x == i).unwrap()]);
                    layer.filter.evaluate(point_on_sphere + warp_p)
                } else {
                    layer.filter.evaluate(point_on_sphere)
                };
                elevation += v * mask;
            }
        }

        elevation = self.radius * (1.0 + elevation);
        elevation
    }

    pub fn get_warped_pos(&self, p: Vec3, warp_source: &NoiseLayer) -> Vec3 {
        let x = warp_source.filter.evaluate(p + warp_source.filter.warp_offset.x);
        let y = warp_source.filter.evaluate(p + warp_source.filter.warp_offset.y);
        let z = warp_source.filter.evaluate(p + warp_source.filter.warp_offset.z);
        Vec3::new(x, y, z)
    }
}
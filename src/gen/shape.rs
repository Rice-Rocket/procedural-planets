use bevy::prelude::*;
use serde::{Serialize, Deserialize};

use super::noise_filter::NoiseLayer;


#[derive(Resource, Clone, Serialize, Deserialize)]
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
    pub fn get_point_on_planet(&self, point_on_sphere: Vec3) -> (Vec3, f32) {
        let mut elevation = 0.0;
        let first_layer = self.noise_layers[0].filter.evaluate(point_on_sphere);
        if self.noise_layers[0].enabled {
            elevation = first_layer;
        }

        for i in 1..self.num_layers {
            if self.noise_layers[i as usize].enabled {
                let mask = if self.noise_layers[i as usize].first_layer_mask { (first_layer - self.sea_level + 1.0).max(0.0) } else { 1.0 };
                let v = self.noise_layers[i as usize].filter.evaluate(point_on_sphere);
                elevation += v * mask;
            }
        }

        elevation = self.radius * (1.0 + elevation);
        (point_on_sphere * elevation, elevation)
    }

    pub fn get_unscaled_elevation(&self, point_on_sphere: Vec3) -> f32 {
        let mut elevation = 0.0;
        let first_layer = self.noise_layers[0].filter.evaluate(point_on_sphere);
        if self.noise_layers[0].enabled {
            elevation = first_layer;
        }

        for i in 1..self.num_layers {
            if self.noise_layers[i as usize].enabled {
                let mask = if self.noise_layers[i as usize].first_layer_mask { first_layer } else { 1.0 };
                let v = self.noise_layers[i as usize].filter.evaluate(point_on_sphere);
                elevation += v * mask;
            }
        }

        elevation
    }

    pub fn scale_elevation(&self, unscaled: f32) -> f32 {
        let mut v = unscaled.max(0.0);
        v = self.radius * (1.0 + v);
        v
    }
}
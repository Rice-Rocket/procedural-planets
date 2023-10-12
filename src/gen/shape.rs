use bevy::prelude::*;

use super::noise_filter::NoiseLayer;


#[derive(Resource, Clone)]
pub struct ShapeGenerator {
    pub radius: f32,
    pub num_layers: u32,
    pub noise_layers: Vec<NoiseLayer>,
}

impl Default for ShapeGenerator {
    fn default() -> Self {
        Self {
            radius: 1.0,
            num_layers: 1,
            noise_layers: vec![NoiseLayer::new(0, true)],
        }
    }
}

impl ShapeGenerator {
    pub fn get_point_on_planet(&self, point_on_sphere: Vec3) -> Vec3 {
        let mut elevation = 0.0;

        for i in 0..self.num_layers {
            if self.noise_layers[i as usize].enabled {
                elevation += self.noise_layers[i as usize].filter.evaluate(point_on_sphere);
            }
        }

        point_on_sphere * self.radius * (1.0 + elevation)
    }
}
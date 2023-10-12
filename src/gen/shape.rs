use bevy::prelude::*;

use super::noise_filter::NoiseFilter;


#[derive(Resource, Clone)]
pub struct ShapeGenerator {
    pub radius: f32,
    pub noise: NoiseFilter,
}

impl Default for ShapeGenerator {
    fn default() -> Self {
        Self {
            radius: 1.0,
            noise: NoiseFilter::new(),
        }
    }
}

impl ShapeGenerator {
    pub fn get_point_on_planet(&self, point_on_sphere: Vec3) -> Vec3 {
        let elevation = self.noise.evaluate(point_on_sphere);
        point_on_sphere * self.radius * (1.0 + elevation)
    }
}
use bevy::prelude::*;

use super::noise::NoiseSimplex3d;

pub struct NoiseFilter {
    simplex_3d: NoiseSimplex3d,

    pub strength: f32,
    pub roughness: f32,
    pub center: Vec3,
}

impl NoiseFilter {
    pub fn new() -> Self {
        Self {
            simplex_3d: NoiseSimplex3d::new(0),

            strength: 1.0,
            roughness: 1.0,
            center: Vec3::ZERO,
        }
    }

    pub fn evaluate(&self, p: Vec3) -> f32 {
        let noise_val = (self.simplex_3d.evaluate(p * self.roughness + self.center) + 1.0) * 0.5;
        noise_val * self.strength
    }
}
use bevy::prelude::*;

use super::noise::NoiseSimplex3d;

#[derive(Clone)]
pub struct NoiseFilter {
    simplex_3d: NoiseSimplex3d,

    pub num_layers: i32,
    pub strength: f32,
    pub roughness: f32,
    pub frequency: f32,
    pub persistence: f32,
    pub center: Vec3,
}

impl NoiseFilter {
    pub fn new() -> Self {
        Self {
            simplex_3d: NoiseSimplex3d::new(0),

            num_layers: 1,
            strength: 1.0,
            roughness: 1.0,
            frequency: 2.0,
            persistence: 0.5,
            center: Vec3::ZERO,
        }
    }

    pub fn evaluate(&self, p: Vec3) -> f32 {
        let mut noise_val = 0.0;
        let mut f = self.roughness;
        let mut amp = 1.0;

        for _ in 0..self.num_layers {
            let v = self.simplex_3d.evaluate(p * f + self.center);
            noise_val += (v + 1.0) * 0.5 * amp;
            f *= self.frequency;
            amp *= self.persistence;
        }
        
        noise_val * self.strength
    }
}
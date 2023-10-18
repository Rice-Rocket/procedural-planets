use bevy::prelude::*;
use serde::{Serialize, Deserialize};

use super::noise::NoiseSimplex3d;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum NoiseFilterType {
    Standard,
    Rigid,
    Warp,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NoiseFilter {
    #[serde(skip)]
    pub simplex_3d: NoiseSimplex3d,
    pub noise_seed: u32,
    pub ty: NoiseFilterType,

    pub num_octaves: i32,
    pub strength: f32,
    pub roughness: f32,
    pub lacunarity: f32,
    pub persistence: f32,
    pub offset: f32,
    pub floor: f32,
    pub center: Vec3,
    pub warp_offset: Vec3,
}

impl NoiseFilter {
    pub fn new(seed: u32) -> Self {
        Self {
            simplex_3d: NoiseSimplex3d::new(seed),
            noise_seed: seed,
            ty: NoiseFilterType::Standard,

            num_octaves: 1,
            strength: 1.0,
            roughness: 1.0,
            lacunarity: 2.0,
            persistence: 0.5,
            offset: 0.0,
            floor: 0.0,
            center: Vec3::ZERO,
            warp_offset: Vec3::new(0.0, 100.0, -100.0),
        }
    }

    pub fn evaluate(&self, p: Vec3) -> f32 {
        match self.ty {
            NoiseFilterType::Standard => self.eval_standard(p),
            NoiseFilterType::Rigid => self.eval_rigid(p),
            NoiseFilterType::Warp => self.eval_standard(p),
        }
    }

    pub fn eval_standard(&self, p: Vec3) -> f32 {
        let mut noise_val = 0.0;
        let mut f = self.roughness;
        let mut amp = 1.0;

        for _ in 0..self.num_octaves {
            let v = self.simplex_3d.evaluate(p * f + self.center);
            noise_val += (v + 1.0) * 0.5 * amp;
            f *= self.lacunarity;
            amp *= self.persistence;
        }
        
        noise_val = noise_val;
        noise_val * self.strength - self.offset
    }

    pub fn eval_rigid(&self, p: Vec3) -> f32 {
        let mut noise_val = 0.0;
        let mut f = self.roughness;
        let mut amp = 1.0;
        let mut weight = 1.0;

        for _ in 0..self.num_octaves {
            let mut v = 1.0 - self.simplex_3d.evaluate(p * f + self.center).abs();
            v *= v;
            v *= weight;
            weight = v;

            noise_val += v * amp;
            f *= self.lacunarity;
            amp *= self.persistence;
        }
        
        noise_val = noise_val;
        noise_val * self.strength - self.offset
    }
}


#[derive(Serialize, Deserialize, Clone)]
pub struct NoiseLayer {
    pub filter: NoiseFilter,
    pub is_warp: bool,
    pub warp_target: u32,
    pub first_layer_mask: bool,
    pub enabled: bool,
}

impl NoiseLayer {
    pub fn new(i: u32, enabled: bool) -> Self {
        Self {
            filter: NoiseFilter::new(i),
            is_warp: false,
            warp_target: 1,
            first_layer_mask: false,
            enabled,
        }
    }
}
use bevy::{prelude::*, reflect::{TypeUuid, TypePath}, render::render_resource::{AsBindGroup, ShaderType}};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, ShaderType)]
pub struct ColorEntry {
    pub color: [f32; 3],
    pub elevation: f32,
    _padding: [f32; 3],
    pub steepness: f32,
}

impl Default for ColorEntry {
    fn default() -> Self {
        Self {
            color: [0.0; 3],
            elevation: 0.0,
            _padding: [0.0; 3],
            steepness: 0.0,
        }
    }
}

impl ColorEntry {
    pub fn new(elevation: f32, steepness: f32, color: [f32; 3]) -> Self {
        Self {
            elevation,
            color,
            steepness,
            _padding: [0.0; 3],
        }
    }
}


#[derive(AsBindGroup, Debug, Clone, TypeUuid, TypePath)]
#[uuid = "717f64fe-6844-4822-8926-e0ed374294c8"]
pub struct PlanetMaterial {
    #[uniform(0)]
    pub min_elevation: f32,
    #[uniform(0)]
    pub max_elevation: f32,
    #[uniform(0)]
    pub n_colors: u32,
    #[storage(1, read_only)]
    pub colors: [ColorEntry; ColorGradient::RESOLUTION as usize],
}

impl Material for PlanetMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/planet.wgsl".into()
    }
}


#[derive(Serialize, Deserialize, Clone)]
pub struct ColorGradient {
    key_points: Vec<(f32, f32, [f32; 3], bool)>,
}

impl ColorGradient {
    pub const RESOLUTION: u32 = 20;

    pub fn new() -> Self {
        Self {
            key_points: vec![(0.0, 0.0, [0.0; 3], true)],
        }
    }
    pub fn add(&mut self, color: [f32; 3], u: f32, v: f32) {
        self.key_points.push((u, v, color, true));
    }
    pub fn get_mut(&mut self, id: usize) -> &mut (f32, f32, [f32; 3], bool) {
        &mut self.key_points[id]
    }
    pub fn get_enabled_mut(&mut self, id: usize) -> &mut bool {
        &mut self.key_points[id].3
    }
    pub fn get_u_mut(&mut self, id: usize) -> &mut f32 {
        &mut self.key_points[id].0
    }
    pub fn get_v_mut(&mut self, id: usize) -> &mut f32 {
        &mut self.key_points[id].1
    }
    pub fn get_col_mut(&mut self, id: usize) -> &mut [f32; 3] {
        &mut self.key_points[id].2
    }
    pub fn get(&self, id: usize) -> &(f32, f32, [f32; 3], bool) {
        &self.key_points[id]
    }
    pub fn get_enabled(&self, id: usize) -> &bool {
        &self.key_points[id].3
    }
    pub fn get_u(&self, id: usize) -> &f32 {
        &self.key_points[id].0
    }
    pub fn get_v(&self, id: usize) -> &f32 {
        &self.key_points[id].1
    }
    pub fn get_col(&self, id: usize) -> &[f32; 3] {
        &self.key_points[id].2
    }
    pub fn pop(&mut self, id: usize) {
        self.key_points.remove(id);
    }
    pub fn count(&self) -> u32 {
        let filtered: Vec<_> = self.key_points.iter().filter(|x| x.3).collect();
        return filtered.len() as u32;
    }
    pub fn sorted(&self) -> Vec<(f32, f32, [f32; 3])> {
        let mut keys: Vec<_> = self.key_points.iter()
            .map(|x| *x)
            .filter(|x| x.3)
            .map(|x| (x.0, x.1, x.2))
            .collect();
        keys.sort_by(|(u1, _, _), (u2, _, _)| u1.partial_cmp(u2).unwrap());
        keys
    }
    pub fn interpolated(&self) -> [Color; Self::RESOLUTION as usize] {
        let mut colors = [Color::BLACK; Self::RESOLUTION as usize];
        let keys = self.sorted();

        for i in 0..Self::RESOLUTION {
            let t = i as f32 / (Self::RESOLUTION as f32 - 1.0);
            colors[i as usize] = self.evaluate(t, keys.clone());
        };

        return colors;
    }
    pub fn evaluate(&self, t: f32, keys: Vec<(f32, f32, [f32; 3])>) -> Color {
        if t <= keys[0].0 {
            return Color::from(keys[0].2);
        }
        if t >= keys[keys.len() - 1].0 {
            return Color::from(keys[keys.len() - 1].2);
        }

        for i in 0..(keys.len() - 1) {
            let start = keys[i];
            let end = keys[i + 1];

            if t < end.0 {
                let local_t = ((t - start.0) / (end.0 - start.0)).max(0.0).min(1.0);

                let r = start.2[0] + (end.2[0] - start.2[0]) * local_t;
                let g = start.2[1] + (end.2[1] - start.2[1]) * local_t;
                let b = start.2[2] + (end.2[2] - start.2[2]) * local_t;

                return Color::rgb(r, g, b);
            }
        };
        Color::BLACK
    }
}
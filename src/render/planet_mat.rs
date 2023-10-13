use bevy::{prelude::*, reflect::{TypeUuid, TypePath}, render::render_resource::AsBindGroup};


#[derive(AsBindGroup, Debug, Clone, TypeUuid, TypePath)]
#[uuid = "717f64fe-6844-4822-8926-e0ed374294c8"]
pub struct PlanetMaterial {
    #[uniform(0)]
    pub min_elevation: f32,
    #[uniform(0)]
    pub max_elevation: f32,
    #[texture(1)]
    #[sampler(2)]
    pub elevation_gradient: Handle<Image>,
}

impl Material for PlanetMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/planet.wgsl".into()
    }
}


pub struct ColorGradient {
    key_points: Vec<(f32, [f32; 3], bool)>,
}

impl ColorGradient {
    pub const RESOLUTION: u32 = 50;

    pub fn new() -> Self {
        Self {
            key_points: vec![(0.0, [0.0; 3], true)],
        }
    }
    pub fn add(&mut self, color: [f32; 3], t: f32) {
        self.key_points.push((t, color, true));
    }
    pub fn get_mut(&mut self, id: usize) -> &mut (f32, [f32; 3], bool) {
        &mut self.key_points[id]
    }
    pub fn get_enabled_mut(&mut self, id: usize) -> &mut bool {
        &mut self.key_points[id].2
    }
    pub fn get_t_mut(&mut self, id: usize) -> &mut f32 {
        &mut self.key_points[id].0
    }
    pub fn get_col_mut(&mut self, id: usize) -> &mut [f32; 3] {
        &mut self.key_points[id].1
    }
    pub fn get(&self, id: usize) -> &(f32, [f32; 3], bool) {
        &self.key_points[id]
    }
    pub fn get_enabled(&self, id: usize) -> &bool {
        &self.key_points[id].2
    }
    pub fn get_t(&self, id: usize) -> &f32 {
        &self.key_points[id].0
    }
    pub fn get_col(&self, id: usize) -> &[f32; 3] {
        &self.key_points[id].1
    }
    pub fn pop(&mut self, id: usize) {
        self.key_points.remove(id);
    }
    pub fn interpolated(&self) -> [Color; Self::RESOLUTION as usize] {
        let mut colors = [Color::BLACK; Self::RESOLUTION as usize];
        let mut keys: Vec<_> = self.key_points.iter().filter(|x| x.2).collect();
        keys.sort_by(|(t1, _, _), (t2, _, _)| t1.partial_cmp(t2).unwrap());

        for i in 0..Self::RESOLUTION {
            let t = i as f32 / (Self::RESOLUTION as f32 - 1.0);
            colors[i as usize] = self.evaluate(t, keys.clone());
        };

        return colors;
    }
    pub fn evaluate(&self, t: f32, keys: Vec<&(f32, [f32; 3], bool)>) -> Color {
        if t <= keys[0].0 {
            return Color::from(keys[0].1);
        }
        if t >= keys[keys.len() - 1].0 {
            return Color::from(keys[keys.len() - 1].1);
        }

        for i in 0..(keys.len() - 1) {
            let start = keys[i];
            let end = keys[i + 1];

            if t < end.0 {
                let local_t = ((t - start.0) / (end.0 - start.0)).max(0.0).min(1.0);

                let r = start.1[0] + (end.1[0] - start.1[0]) * local_t;
                let g = start.1[1] + (end.1[1] - start.1[1]) * local_t;
                let b = start.1[2] + (end.1[2] - start.1[2]) * local_t;

                return Color::rgb(r, g, b);
            }
        };
        Color::BLACK
    }
}
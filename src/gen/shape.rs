use bevy::prelude::*;


#[derive(Resource)]
pub struct ShapeGenerator {
    pub radius: f32,
}

impl Default for ShapeGenerator {
    fn default() -> Self {
        Self {
            radius: 1.0,
        }
    }
}

impl ShapeGenerator {
    pub fn get_point_on_planet(&self, point_on_sphere: Vec3) -> Vec3 {
        point_on_sphere * self.radius
    }
}
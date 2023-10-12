pub mod noise;
pub mod shape;

use bevy::prelude::*;

use shape::*;


pub struct GeneratorPlugin;

impl Plugin for GeneratorPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ShapeGenerator>()
        ;
    }
}
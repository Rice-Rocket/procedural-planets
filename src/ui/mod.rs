pub mod camera;
pub mod shape;
pub mod color;
pub mod render;

use bevy::prelude::*;

use camera::*;
use shape::*;
use color::*;
use render::*;


pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<UiRenderSettings>()
            .init_resource::<UiColorSettings>()
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, (
                pan_orbit_camera,
                
                render_settings,
                shape_settings,
                color_settings,

                update_shape,
                update_color,
            ))
        ;
    }
}
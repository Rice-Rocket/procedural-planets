pub mod camera;
pub mod shape;
pub mod color;
pub mod render;
pub mod save;

use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;

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
            .init_resource::<UiVisibility>()
            .add_plugins(PanOrbitCameraPlugin)
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, (
                // pan_orbit_camera,

                render_settings,
                shape_settings,
                color_settings,
            ))
        ;
    }
}
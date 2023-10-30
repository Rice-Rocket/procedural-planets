pub mod camera;
pub mod shape;
pub mod color;
pub mod render;
pub mod save;
pub mod controller;

use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use super::controller::FpsControllerPlugin;
// use bevy_fps_controller::controller::FpsControllerPlugin;

use bevy_rapier3d::{prelude::{RapierPhysicsPlugin, NoUserData}, render::RapierDebugRenderPlugin};
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
            .init_resource::<CameraMode>()
            .add_plugins(PanOrbitCameraPlugin)
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            // .add_plugins(RapierDebugRenderPlugin::default())
            .add_plugins(FpsControllerPlugin)
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, (
                update_camera_mode,
                camera_cursor_grab,
                update_camera_local_up,

                render_settings,
                shape_settings,
                color_settings,
            ))
        ;
    }
}
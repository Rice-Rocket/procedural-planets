use bevy::{prelude::*, core_pipeline::{clear_color::ClearColorConfig, prepass::DepthPrepass}};
use bevy_panorbit_camera::PanOrbitCamera;

use crate::render::atmosphere::AtmosphereSettings;


pub fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn((
        Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::default(), Vec3::Y),
            ..default()
        }, 
        PanOrbitCamera::default(), 
        AtmosphereSettings::default(),
        DepthPrepass,
    ));
}
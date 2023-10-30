use bevy::{prelude::*, core_pipeline::{clear_color::ClearColorConfig, prepass::DepthPrepass}, window::{CursorGrabMode, PrimaryWindow}};
use bevy_panorbit_camera::PanOrbitCamera;
use bevy_rapier3d::prelude::*;
use super::controller::*;
// use bevy_fps_controller::controller::*;

use crate::render::{atmosphere::AtmosphereSettings, planet::TerrainFace};

use super::render::UiVisibility;


#[derive(Resource, PartialEq, Default, Copy, Clone)]
pub enum CameraMode {
    #[default]
    Edit,
    Explore,
}

pub fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn((
        Camera3dBundle {
            projection: Projection::Perspective(PerspectiveProjection {
                near: 0.0001,
                far: 100.0,
                ..default()
            }),
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


pub fn update_camera_mode(
    mut commands: Commands,
    camera_entities: Query<Entity, With<Camera>>,
    camera_mode: Res<CameraMode>,
    terrain_faces: Query<&Handle<Mesh>, With<TerrainFace>>,
    meshes: Res<Assets<Mesh>>,
    explore_cams: Query<Entity, With<FpsController>>,
    mut old_cam_mode: Local<CameraMode>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
    mut ui_visibility: ResMut<UiVisibility>,
) {
    let mut window = primary_window.single_mut();

    if *camera_mode != *old_cam_mode {
        for entity in camera_entities.iter() {
            if *camera_mode == CameraMode::Edit {
                commands.entity(entity)
                    .remove::<RenderPlayer>()
                    .insert(Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y))
                    .insert(PanOrbitCamera::default());

                for explore_cam in explore_cams.iter() {
                    commands.entity(explore_cam).despawn();
                }

                window.cursor.grab_mode = CursorGrabMode::None;
                window.cursor.visible = true;
                *ui_visibility = UiVisibility::Visible;
            } else {
                let logical_entity = commands.spawn((
                    TransformBundle::from_transform(Transform::from_xyz(0.0, 2.0, 0.0)),
                    Collider::capsule(Vec3::Y * 0.005, Vec3::Y * 0.015, 0.005),
                    Friction {
                        coefficient: 0.0,
                        combine_rule: CoefficientCombineRule::Min,
                    },
                    Restitution {
                        coefficient: 0.0,
                        combine_rule: CoefficientCombineRule::Min,
                    },
                    ActiveEvents::COLLISION_EVENTS,
                    Velocity::zero(),
                    RigidBody::Dynamic,
                    Sleeping::disabled(),
                    LockedAxes::ROTATION_LOCKED,
                    AdditionalMassProperties::Mass(1.0),
                    GravityScale(0.0),
                    Ccd { enabled: true }, // Prevent clipping when going fast
                    LogicalPlayer(0),
                    FpsControllerInput {
                        // pitch: -std::f32::consts::PI / 6.0,
                        // yaw: std::f32::consts::PI * 5.0 / 4.0,
                        pitch: -std::f32::consts::PI,
                        ..default()
                    },
                    FpsController {
                        local_up: Vec3::Y,
                        ..default()
                    },
                )).insert(CameraConfig {
                    height_offset: -0.05,
                    radius_scale: 0.0075,
                }).id();

                commands.entity(entity)
                    .remove::<PanOrbitCamera>()
                    .insert(RenderPlayer { logical_entity });

                for mesh_handle in terrain_faces.iter() {
                    let mesh = meshes.get(mesh_handle).unwrap();
                    let vertices = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap().as_float3().unwrap().iter().map(|x| Vec3::from(*x)).collect();
                    let indices = mesh.indices().unwrap().iter().collect::<Vec<usize>>().chunks(3).map(|x| [x[0] as u32, x[1] as u32, x[2] as u32]).collect();
                    commands.spawn(Collider::trimesh(vertices, indices));
                }

                window.cursor.grab_mode = CursorGrabMode::Confined;
                window.cursor.visible = false;
                *ui_visibility = UiVisibility::Hidden;
            }
        }
        *old_cam_mode = *camera_mode;
    }
}

pub fn toggle_grab_cursor(window: &mut Window) {
    match window.cursor.grab_mode {
        CursorGrabMode::None => {
            window.cursor.grab_mode = CursorGrabMode::Confined;
            window.cursor.visible = false;
        },
        _ => {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    }
}

pub fn camera_cursor_grab(
    keys: Res<Input<KeyCode>>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = primary_window.single_mut();
    if keys.just_pressed(KeyCode::Escape) {
        toggle_grab_cursor(&mut window);
    }
}

pub fn update_camera_local_up(
    mut controllers: Query<(&mut FpsController, &Transform), Without<RenderPlayer>>,
    mut cameras: Query<&mut Transform, With<RenderPlayer>>,
) {
    for (mut controller, transform) in controllers.iter_mut() {
        controller.local_up = (transform.translation - Vec3::ZERO).normalize();
    }
    for mut transform in cameras.iter_mut() {
        let forward = transform.forward();
        let up = (transform.translation - Vec3::ZERO).normalize();
        transform.look_to(forward, up);
    }
}
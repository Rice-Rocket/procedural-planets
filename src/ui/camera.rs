use bevy::{prelude::*, window::PrimaryWindow, input::mouse::{MouseMotion, MouseWheel}, core_pipeline::clear_color::ClearColorConfig};
use bevy_egui::EguiContext;



#[derive(Component)]
pub struct PanOrbitCamera {
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            upside_down: false,
        }
    }
}

impl PanOrbitCamera {
    pub fn from_radius(radius: f32) -> Self {
        Self {
            radius,
            ..default()
        }
    }
    pub fn from_translation(pos: Vec3) -> Self {
        Self {
            radius: pos.length(),
            ..default()
        }
    }
}

pub fn pan_orbit_camera(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut motion_evr: EventReader<MouseMotion>,
    mut scroll_evr: EventReader<MouseWheel>,
    mouse_input: Res<Input<MouseButton>>,
    mut query: Query<(&mut PanOrbitCamera, &mut Transform, &Projection)>,
    mut egui_ctxs: Query<&mut EguiContext>,
) {
    for mut ctx in egui_ctxs.iter_mut() {
        if ctx.get_mut().wants_pointer_input() {
            return;
        }
    }

    let orbit_button = MouseButton::Left;
    let pan_button = MouseButton::Right;

    let mut pan = Vec2::ZERO;
    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.0;
    let mut orbit_button_changed = false;

    if mouse_input.pressed(orbit_button) {
        for ev in motion_evr.iter() {
            rotation_move += ev.delta;
        }
    } else if mouse_input.pressed(pan_button) {
        for ev in motion_evr.iter() {
            pan += ev.delta;
        }
    }

    for ev in scroll_evr.iter() {
        scroll += ev.y * 0.25;
    }
    if mouse_input.just_released(orbit_button) || mouse_input.just_pressed(orbit_button) {
        orbit_button_changed = true;
    }

    for (mut pan_orbit, mut transform, projection) in query.iter_mut() {
        if orbit_button_changed {
            let up = transform.rotation * Vec3::Y;
            pan_orbit.upside_down = up.y <= 0.0;
        }

        let mut any = false;
        if rotation_move.length_squared() > 0.0 {
            any = true;
            let window = window_query.get_single().unwrap();
            let delta_x = {
                let delta = rotation_move.x / window.width() * std::f32::consts::PI * 2.0;
                if pan_orbit.upside_down { -delta } else { delta }
            };
            let delta_y = rotation_move.y / window.height() * std::f32::consts::PI;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation;
            transform.rotation = transform.rotation * pitch;
        } else if pan.length_squared() > 0.0 {
            any = true;
            let window = window_query.get_single().unwrap();
            if let Projection::Perspective(projection) = projection {
                pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / Vec2::new(window.width(), window.height());
            }
            let right = transform.rotation * Vec3::X * -pan.x;
            let up = transform.rotation * Vec3::Y * pan.y;
            let translation = (right + up) * pan_orbit.radius;
            pan_orbit.focus += translation;
        } else if scroll.abs() > 0.0 {
            any = true;
            pan_orbit.radius -= scroll * pan_orbit.radius * 0.2;
            pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);
        }

        if any {
            let rot_mat = Mat3::from_quat(transform.rotation);
            transform.translation = pan_orbit.focus + rot_mat.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
        }
    }

    motion_evr.clear();
}


pub fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn((Camera3dBundle {
        camera_3d: Camera3d {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::default(), Vec3::Y),
        ..default()
    }, PanOrbitCamera::default()));
}
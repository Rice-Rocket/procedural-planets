use bevy::prelude::*;

use crate::ui::render::UiRenderSettings;


pub fn spawn_directional_light(
    mut commands: Commands,
) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::rgb(1.0, 1.0, 1.0),
            illuminance: 10000.0,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::YXZ, 0.0, 0.0, 0.0)),
        ..default()
    });
}

pub fn update_directional_light(
    mut directional_light_transforms: Query<&mut Transform, With<DirectionalLight>>,
    render_settings: Res<UiRenderSettings>,
) {
    if !render_settings.is_changed() { return; }
    for mut transform in directional_light_transforms.iter_mut() {
        let rot = render_settings.light_euler_rot * std::f32::consts::PI / 180.0;
        transform.rotation = Quat::from_euler(EulerRot::YXZ, rot.x, rot.y, rot.z);
    }
}
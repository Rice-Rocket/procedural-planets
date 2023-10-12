use bevy::{prelude::*, pbr::wireframe::WireframeConfig};
use bevy_egui::{egui, EguiContexts};

use crate::render::planet::{UpdatePlanetMesh, Planet};

#[derive(Resource)]
pub struct UiRenderSettings {
    pub planet_resolution: u32,
    pub light_euler_rot: Vec3,
}

impl Default for UiRenderSettings {
    fn default() -> Self {
        Self {
            planet_resolution: 10,
            light_euler_rot: Vec3::ZERO,
        }
    }
}

pub fn render_settings(
    mut contexts: EguiContexts,
    mut settings: ResMut<UiRenderSettings>,
    mut planet: ResMut<Planet>,
    mut update_planet_mesh_evw: EventWriter<UpdatePlanetMesh>,

    time: Res<Time>,
    mut wireframe_config: ResMut<WireframeConfig>,
    mut last_fps_update: Local<(f32, f32)>,
) {
    egui::Window::new("Render Settings").show(contexts.ctx_mut(), |ui| {
        let mut fps_value = last_fps_update.0;
        if last_fps_update.1 > 0.25 {
            fps_value = 1.0 / time.delta_seconds();
            last_fps_update.1 = 0.0;
            last_fps_update.0 = fps_value;
        }
        ui.label(format!("FPS: {:.1}", fps_value));
        last_fps_update.1 += time.delta_seconds();
        ui.label("Press [TAB] to Toggle UI");

        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Mesh Resolution:");
            let old_res = settings.planet_resolution;
            ui.add(egui::widgets::DragValue::new(&mut settings.planet_resolution).clamp_range(2..=256));
            if old_res != settings.planet_resolution {
                planet.resolution = settings.planet_resolution;
                update_planet_mesh_evw.send(UpdatePlanetMesh {});
            }
        });

        ui.horizontal(|ui| {
            ui.label("Enable Wireframe:");
            ui.add(egui::widgets::Checkbox::without_text(&mut wireframe_config.global));
        });

        ui.horizontal(|ui| {
            ui.label("Light Direction:");
            ui.add(egui::DragValue::new(&mut settings.light_euler_rot.y).prefix("Pitch: ").clamp_range(0..=360).speed(2));
            ui.add(egui::DragValue::new(&mut settings.light_euler_rot.x).prefix("Yaw: ").clamp_range(0..=360).speed(2));
        });
    });
}
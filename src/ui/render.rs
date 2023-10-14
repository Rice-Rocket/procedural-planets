use bevy::{prelude::*, pbr::wireframe::WireframeConfig};
use bevy_egui::{egui, EguiContexts};

use crate::render::planet::{UpdatePlanetMesh, Planet};

#[derive(Resource)]
pub struct UiRenderSettings {
    pub planet_resolution: u32,
    pub light_euler_rot: Vec3,

    pub ocean_radius: f32,
    pub ocean_depth_mul: f32,
    pub ocean_alpha_mul: f32,
    pub ocean_color_1: [f32; 3],
    pub ocean_color_2: [f32; 3],
}

impl Default for UiRenderSettings {
    fn default() -> Self {
        Self {
            planet_resolution: 10,
            light_euler_rot: Vec3::ZERO,

            ocean_radius: 1.0,
            ocean_depth_mul: 1.0,
            ocean_alpha_mul: 1.0,
            ocean_color_1: [0.0; 3],
            ocean_color_2: [1.0; 3],
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

        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Ocean Elevation:");
            ui.add(egui::DragValue::new(&mut settings.ocean_radius).speed(0.025).min_decimals(2).clamp_range(0f32..=100f32));
        });

        ui.horizontal(|ui| {
            ui.label("Ocean Depth Multiplier:");
            ui.add(egui::DragValue::new(&mut settings.ocean_depth_mul).speed(0.05).min_decimals(2).clamp_range(0f32..=100f32));
        });

        ui.horizontal(|ui| {
            ui.label("Ocean Alpha Multiplier:");
            ui.add(egui::DragValue::new(&mut settings.ocean_alpha_mul).speed(0.05).min_decimals(2).clamp_range(0f32..=100f32));
        });

        ui.horizontal(|ui| {
            ui.label("Ocean Deep Color:");
            egui::color_picker::color_edit_button_rgb(ui, &mut settings.ocean_color_1);
        });

        ui.horizontal(|ui| {
            ui.label("Ocean Shallow Color:");
            egui::color_picker::color_edit_button_rgb(ui, &mut settings.ocean_color_2);
        });
    });
}
use bevy::prelude::*;
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
) {
    egui::Window::new("Render Settings").show(contexts.ctx_mut(), |ui| {
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
            ui.label("Light Direction:");
            ui.add(egui::DragValue::new(&mut settings.light_euler_rot.y).prefix("Pitch: ").clamp_range(0..=360).speed(2));
            ui.add(egui::DragValue::new(&mut settings.light_euler_rot.x).prefix("Yaw: ").clamp_range(0..=360).speed(2));
        });
    });
}
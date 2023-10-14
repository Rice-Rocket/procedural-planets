use bevy::{prelude::*, pbr::wireframe::WireframeConfig};
use bevy_egui::{egui, EguiContexts};
use serde::{Serialize, Deserialize};

use crate::{render::planet::{UpdatePlanetMesh, Planet, UpdatePlanetMaterials}, gen::shape::ShapeGenerator};

use super::{save::{SaveState, restore_save}, color::UiColorSettings};

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct UiRenderSettings {
    pub planet_resolution: u32,
    pub light_euler_rot: Vec3,

    pub ocean_radius: f32,
    pub ocean_depth_mul: f32,
    pub ocean_alpha_mul: f32,
    pub ocean_color_1: [f32; 3],
    pub ocean_color_2: [f32; 3],

    pub load_path: String,
    pub save_path: String,
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
            load_path: String::from(""),
            save_path: String::from(""),
        }
    }
}

pub fn render_settings(
    mut contexts: EguiContexts,
    mut settings: ResMut<UiRenderSettings>,
    mut planet: ResMut<Planet>,
    mut update_planet_mesh_evw: EventWriter<UpdatePlanetMesh>,
    mut update_planet_materials_evw: EventWriter<UpdatePlanetMaterials>,
    mut shape_gen: ResMut<ShapeGenerator>,
    mut colors: ResMut<UiColorSettings>,

    time: Res<Time>,
    mut wireframe_config: ResMut<WireframeConfig>,
    mut last_fps_update: Local<(f32, f32)>,
) {
    egui::Window::new("Settings").show(contexts.ctx_mut(), |ui| {
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
            if ui.button("Load Planet:").clicked() {
                if let Ok(file_contents) = std::fs::read(format!("assets/saves/{}.ron", settings.load_path)) {
                    let deserialized: SaveState = ron::de::from_bytes(&file_contents).unwrap();
                    restore_save(deserialized, settings.as_mut(), shape_gen.as_mut(), colors.as_mut());

                    planet.resolution = settings.planet_resolution;
                    update_planet_mesh_evw.send(UpdatePlanetMesh {});
                    update_planet_materials_evw.send(UpdatePlanetMaterials {});
                }
            }
            ui.text_edit_singleline(&mut settings.load_path);
        });
        ui.horizontal(|ui| {
            if ui.button("Save Scene:").clicked() {
                let savestate = SaveState {
                    shape_gen: shape_gen.clone(),
                    colors: colors.clone(),
                    settings: settings.clone(),
                };

                let serialized = ron::ser::to_string_pretty(&savestate, ron::ser::PrettyConfig::default()).unwrap();
                std::fs::write(format!("assets/saves/{}.ron", settings.save_path), serialized).unwrap();
            }
            ui.text_edit_singleline(&mut settings.save_path);
        });

        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Mesh Resolution:");
            let old_res = settings.planet_resolution;
            ui.add(egui::widgets::DragValue::new(&mut settings.planet_resolution).clamp_range(3..=512));
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
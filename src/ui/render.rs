use bevy::{prelude::*, pbr::wireframe::WireframeConfig};
use bevy_egui::{egui, EguiContexts};
use serde::{Serialize, Deserialize};

use crate::{render::planet::{UpdatePlanetMesh, Planet, UpdatePlanetMaterials}, gen::shape::ShapeGenerator};

use super::{save::{SaveState, restore_save}, color::UiColorSettings, camera::CameraMode};

#[derive(Resource, Default, PartialEq)]
pub enum UiVisibility {
    Hidden,
    #[default]
    Visible,
}


#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct UiRenderSettings {
    pub planet_resolution: u32,
    pub light_euler_rot: Vec3,

    pub ocean_radius: f32,
    pub ocean_depth_mul: f32,
    pub ocean_alpha_mul: f32,
    pub ocean_smoothness: f32,
    pub ocean_color_1: [f32; 3],
    pub ocean_color_2: [f32; 3],

    pub atmosphere_radius: f32,
    pub atmosphere_sample_points: u32,
    pub atmosphere_optical_depth_points: u32,
    pub atmosphere_density_falloff: f32,
    pub atmosphere_scatter_strength: f32,
    pub atmosphere_scatter_coeffs: [f32; 3],

    pub waves_normal_map_1: u32,
    pub waves_normal_map_2: u32,
    pub wave_strength: f32,
    pub wave_scale: f32,
    pub wave_speed: f32,

    pub surface_normal_map: u32,
    pub surface_strength: f32,
    pub surface_scale: f32,

    #[serde(skip)]
    pub load_path: String,
    #[serde(skip)]
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
            ocean_smoothness: 1.0,
            ocean_color_1: [0.0; 3],
            ocean_color_2: [1.0; 3],

            atmosphere_radius: 1.5,
            atmosphere_sample_points: 10,
            atmosphere_optical_depth_points: 10,
            atmosphere_density_falloff: 4.0,
            atmosphere_scatter_strength: 20.0,
            atmosphere_scatter_coeffs: [700.0, 530.0, 440.0],

            wave_strength: 0.3,
            wave_scale: 2.0,
            wave_speed: 0.1,
            waves_normal_map_1: 1,
            waves_normal_map_2: 2,

            surface_normal_map: 1,
            surface_strength: 0.1,
            surface_scale: 1.0,

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
    mut camera_mode: ResMut<CameraMode>,

    time: Res<Time>,
    mut wireframe_config: ResMut<WireframeConfig>,
    mut last_fps_update: Local<(f32, f32)>,
    mut ui_visibility: ResMut<UiVisibility>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        *ui_visibility = match *ui_visibility {
            UiVisibility::Hidden => UiVisibility::Visible,
            UiVisibility::Visible => UiVisibility::Hidden,
        };
    }

    if *ui_visibility != UiVisibility::Visible { return };

    egui::Window::new("Settings").show(contexts.ctx_mut(), |ui| {
        let mut fps_value = last_fps_update.0;
        if last_fps_update.1 > 0.25 {
            fps_value = 1.0 / time.delta_seconds();
            last_fps_update.1 = 0.0;
            last_fps_update.0 = fps_value;
        }
        ui.label(format!("FPS: {:.1}", fps_value));
        last_fps_update.1 += time.delta_seconds();
        ui.label("Press [ESC] to Toggle UI");

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
            if ui.button("Save Planet:").clicked() {
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
            ui.label("Camera Mode:");
            ui.selectable_value(camera_mode.as_mut(), CameraMode::Edit, "Edit");
            ui.selectable_value(camera_mode.as_mut(), CameraMode::Explore, "Explore");
        });

        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Mesh Resolution:");
            ui.add(egui::widgets::DragValue::new(&mut settings.planet_resolution).clamp_range(3..=512));
            if ui.button("Update").clicked() {
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

        ui.collapsing("Ocean", |ui| {
            ui.horizontal(|ui| {
                ui.label("Elevation:");
                ui.add(egui::DragValue::new(&mut settings.ocean_radius).speed(0.025).min_decimals(2).clamp_range(0f32..=100f32));
                if shape_gen.sea_level != settings.ocean_radius {
                    shape_gen.sea_level = settings.ocean_radius;
                    update_planet_mesh_evw.send(UpdatePlanetMesh {});
                }
            });

            ui.horizontal(|ui| {
                ui.label("Depth Multiplier:");
                ui.add(egui::DragValue::new(&mut settings.ocean_depth_mul).speed(0.05).min_decimals(2).clamp_range(0f32..=100f32));
            });

            ui.horizontal(|ui| {
                ui.label("Alpha Multiplier:");
                ui.add(egui::DragValue::new(&mut settings.ocean_alpha_mul).speed(0.05).min_decimals(2).clamp_range(0f32..=100f32));
            });

            ui.horizontal(|ui| {
                ui.label("Smoothness:");
                ui.add(egui::DragValue::new(&mut settings.ocean_smoothness).speed(0.01).min_decimals(2).clamp_range(0f32..=0.99f32));
            });

            ui.horizontal(|ui| {
                ui.label("Deep Color:");
                egui::color_picker::color_edit_button_rgb(ui, &mut settings.ocean_color_1);
            });

            ui.horizontal(|ui| {
                ui.label("Shallow Color:");
                egui::color_picker::color_edit_button_rgb(ui, &mut settings.ocean_color_2);
            });
        });

        ui.separator();

        ui.collapsing("Atmosphere", |ui| {
            ui.horizontal(|ui| {
                ui.label("Radius:");
                ui.add(egui::DragValue::new(&mut settings.atmosphere_radius).speed(0.025).min_decimals(2).clamp_range(0f32..=100f32));
            });
            
            ui.horizontal(|ui| {
                ui.label("Num Sample Points:");
                ui.add(egui::DragValue::new(&mut settings.atmosphere_sample_points).speed(0.025));
            });
            
            ui.horizontal(|ui| {
                ui.label("Num Optical Depth Samples:");
                ui.add(egui::DragValue::new(&mut settings.atmosphere_optical_depth_points).speed(0.025));
            });

            ui.horizontal(|ui| {
                ui.label("Density Falloff:");
                ui.add(egui::DragValue::new(&mut settings.atmosphere_density_falloff).speed(0.025).min_decimals(2).clamp_range(0f32..=100f32));
            });

            ui.horizontal(|ui| {
                ui.label("Scattering Strength:");
                ui.add(egui::DragValue::new(&mut settings.atmosphere_scatter_strength).speed(0.025).min_decimals(2).clamp_range(0f32..=100f32));
            });

            ui.label("Scattering Coefficients:");
            ui.indent(1, |ui| {
                ui.add(egui::DragValue::new(&mut settings.atmosphere_scatter_coeffs[0]).speed(0.25).max_decimals(1).clamp_range(0f32..=1000f32).prefix("Red: "));
                ui.add(egui::DragValue::new(&mut settings.atmosphere_scatter_coeffs[1]).speed(0.25).max_decimals(1).clamp_range(0f32..=1000f32).prefix("Green: "));
                ui.add(egui::DragValue::new(&mut settings.atmosphere_scatter_coeffs[2]).speed(0.25).max_decimals(1).clamp_range(0f32..=1000f32).prefix("Blue: "));
            });
        });

        ui.separator();

        ui.collapsing("Normal Maps", |ui| {
            ui.add(egui::DragValue::new(&mut settings.waves_normal_map_1).clamp_range(1..=3).max_decimals(0).speed(0.05).prefix("Wave Normal Map 1: "));
            ui.add(egui::DragValue::new(&mut settings.waves_normal_map_2).clamp_range(1..=3).max_decimals(0).speed(0.05).prefix("Wave Normal Map 2: "));
            
            ui.add(egui::DragValue::new(&mut settings.wave_strength).clamp_range(0f32..=1f32).min_decimals(2).speed(0.025).prefix("Wave Strength: "));
            ui.add(egui::DragValue::new(&mut settings.wave_scale).clamp_range(0f32..=100f32).min_decimals(2).speed(0.025).prefix("Wave Scale: "));
            ui.add(egui::DragValue::new(&mut settings.wave_speed).clamp_range(0f32..=100f32).min_decimals(2).speed(0.025).prefix("Wave Speed: "));
            
            ui.separator();
            
            ui.add(egui::DragValue::new(&mut settings.surface_normal_map).clamp_range(1..=5).max_decimals(0).speed(0.05).prefix("Surface Normal Map: "));
            ui.add(egui::DragValue::new(&mut settings.surface_strength).clamp_range(0f32..=1f32).min_decimals(2).speed(0.025).prefix("Surface Strength: "));
            ui.add(egui::DragValue::new(&mut settings.surface_scale).clamp_range(0f32..=100f32).min_decimals(2).speed(0.025).prefix("Surface Scale: "));
        });
    });
}
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use serde::{Serialize, Deserialize};

use crate::render::{planet::UpdatePlanetMaterials, planet_mat::ColorGradient};

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct UiColorSettings {
    pub num_colors: usize,
    pub colors: ColorGradient,
}

impl Default for UiColorSettings {
    fn default() -> Self {
        Self {
            num_colors: 1,
            colors: ColorGradient::new(),
        }
    }
}

pub fn color_settings(
    mut contexts: EguiContexts,
    mut settings: ResMut<UiColorSettings>,
    mut update_planet_mats_evw: EventWriter<UpdatePlanetMaterials>,
) {
    let mut changed = false;

    egui::Window::new("Color Settings").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("Elevation Key Colors:");
            if ui.small_button("-").clicked() && settings.num_colors > 1 {
                settings.num_colors -= 1;
                let num_colors = settings.num_colors;
                settings.colors.pop(num_colors);
            }
            ui.label(format!("{}", settings.num_colors));
            if ui.small_button("+").clicked() {
                settings.num_colors += 1;
                settings.colors.add([0.0; 3], 0.0, 0.0);
            }
        });

        for i in 0..settings.num_colors {
            let old = settings.colors.get(i).clone();
            ui.horizontal(|ui| {
                ui.collapsing(format!("Point {}", i + 1), |ui| {
                    ui.add(egui::Checkbox::without_text(settings.colors.get_enabled_mut(i)));
                    ui.add_enabled_ui(*settings.colors.get_enabled(i), |ui| {
                        ui.add(egui::DragValue::new(settings.colors.get_u_mut(i)).speed(0.005).clamp_range(0f32..=1f32).prefix("Elevation:"));
                        ui.add(egui::DragValue::new(settings.colors.get_v_mut(i)).speed(0.005).clamp_range(0f32..=1f32).prefix("Steepness:"));
                        egui::color_picker::color_edit_button_rgb(ui, settings.colors.get_col_mut(i));
                    });
                });
            });
            changed = changed || (old != *settings.colors.get(i));
        }
    });

    if changed {
        update_planet_mats_evw.send(UpdatePlanetMaterials {});
    }
}
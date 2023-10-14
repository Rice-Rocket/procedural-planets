use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use serde::{Serialize, Deserialize};

use crate::render::{planet::UpdatePlanetMaterials, planet_mat::ColorGradient};

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct UiColorSettings {
    pub num_elevation_colors: usize,
    pub elevation_colors: ColorGradient,
}

impl Default for UiColorSettings {
    fn default() -> Self {
        Self {
            num_elevation_colors: 1,
            elevation_colors: ColorGradient::new(),
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
            if ui.small_button("-").clicked() && settings.num_elevation_colors > 1 {
                settings.num_elevation_colors -= 1;
                let num_colors = settings.num_elevation_colors;
                settings.elevation_colors.pop(num_colors);
            }
            ui.label(format!("{}", settings.num_elevation_colors));
            if ui.small_button("+").clicked() {
                settings.num_elevation_colors += 1;
                settings.elevation_colors.add([0.0; 3], 0.0);
            }
        });

        for i in 0..settings.num_elevation_colors {
            ui.horizontal(|ui| {
                ui.label(format!("Point {}", i + 1));
                let old = settings.elevation_colors.get(i).clone();
                ui.add(egui::Checkbox::without_text(settings.elevation_colors.get_enabled_mut(i)));
                ui.add_enabled_ui(*settings.elevation_colors.get_enabled(i), |ui| {
                    ui.add(egui::DragValue::new(settings.elevation_colors.get_t_mut(i)).speed(0.005).clamp_range(0f32..=1f32).prefix("t: "));
                    egui::color_picker::color_edit_button_rgb(ui, settings.elevation_colors.get_col_mut(i));
                });
                changed = changed || (old != *settings.elevation_colors.get(i));
            });
        }
    });

    if changed {
        update_planet_mats_evw.send(UpdatePlanetMaterials {});
    }
}
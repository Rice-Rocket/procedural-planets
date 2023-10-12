use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::render::planet::UpdatePlanetMaterials;

#[derive(Resource)]
pub struct UiColorSettings {
    pub planet_color: [f32; 3],
}

impl Default for UiColorSettings {
    fn default() -> Self {
        Self {
            planet_color: [1.0; 3],
        }
    }
}

pub fn color_settings(
    mut contexts: EguiContexts,
    mut settings: ResMut<UiColorSettings>,
) {
    egui::Window::new("Color Settings").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("Color:");
            egui::color_picker::color_edit_button_rgb(ui, &mut settings.planet_color);
        });
    });
}

pub fn update_color(
    settings: Res<UiColorSettings>,
    mut update_planet_mats_evw: EventWriter<UpdatePlanetMaterials>,
) {
    if settings.is_changed() {
        update_planet_mats_evw.send(UpdatePlanetMaterials {});
    }
}
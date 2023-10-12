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
    mut update_planet_mats_evw: EventWriter<UpdatePlanetMaterials>,
) {
    let mut changed = false;

    egui::Window::new("Color Settings").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("Color:");
            let old = settings.planet_color;
            egui::color_picker::color_edit_button_rgb(ui, &mut settings.planet_color);
            changed = changed || (old != settings.planet_color);
        });
    });

    if changed {
        update_planet_mats_evw.send(UpdatePlanetMaterials {});
    }
}
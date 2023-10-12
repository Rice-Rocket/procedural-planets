use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{gen::shape::ShapeGenerator, render::planet::UpdatePlanetMesh};


pub fn shape_settings(
    mut contexts: EguiContexts,
    mut noise_filter: ResMut<ShapeGenerator>,
    mut update_planet_mesh_evw: EventWriter<UpdatePlanetMesh>,
) {
    egui::Window::new("Noise Settings").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("Radius:");
            let old = shape_gen.radius;
            ui.add(egui::widgets::DragValue::new(&mut shape_gen.radius).clamp_range(0f32..=100f32).min_decimals(1).speed(0.2));
            if old != shape_gen.radius {
                update_planet_mesh_evw.send(UpdatePlanetMesh {});
            }
        });
    });
}
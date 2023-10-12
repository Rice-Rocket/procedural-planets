use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{gen::shape::ShapeGenerator, render::planet::UpdatePlanetMesh};


pub fn shape_settings(
    mut contexts: EguiContexts,
    mut shape_gen: ResMut<ShapeGenerator>,
    mut update_planet_mesh_evw: EventWriter<UpdatePlanetMesh>,
) {
    egui::Window::new("Shape Settings").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("Radius:");
            ui.add(egui::widgets::DragValue::new(&mut shape_gen.radius).clamp_range(0f32..=100f32).min_decimals(1).speed(0.2));
        });
        
        ui.horizontal(|ui| {
            ui.label("Noise Strength:");
            ui.add(egui::widgets::DragValue::new(&mut shape_gen.noise.strength).clamp_range(0f32..=100f32).min_decimals(2).speed(0.025));
        });
        
        ui.horizontal(|ui| {
            ui.label("Noise Roughness:");
            ui.add(egui::widgets::DragValue::new(&mut shape_gen.noise.roughness).clamp_range(0f32..=100f32).min_decimals(2).speed(0.025));
        });

        ui.horizontal(|ui| {
            ui.label("Noise Center:");
            ui.add(egui::widgets::DragValue::new(&mut shape_gen.noise.center.x).prefix("X: ").clamp_range(0f32..=100f32).min_decimals(2).speed(0.025));
            ui.add(egui::widgets::DragValue::new(&mut shape_gen.noise.center.y).prefix("Y: ").clamp_range(0f32..=100f32).min_decimals(2).speed(0.025));
            ui.add(egui::widgets::DragValue::new(&mut shape_gen.noise.center.z).prefix("Z: ").clamp_range(0f32..=100f32).min_decimals(2).speed(0.025));
        });
    });
}

pub fn update_shape(
    shape_gen: Res<ShapeGenerator>,
    mut update_planet_mesh_evw: EventWriter<UpdatePlanetMesh>,
) {
    if shape_gen.is_changed() {
        update_planet_mesh_evw.send(UpdatePlanetMesh {});
    }
}
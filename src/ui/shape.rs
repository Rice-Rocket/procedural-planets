use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{gen::shape::ShapeGenerator, render::planet::UpdatePlanetMesh};


pub fn shape_settings(
    mut contexts: EguiContexts,
    mut shape_gen: ResMut<ShapeGenerator>,
    mut update_planet_mesh_evw: EventWriter<UpdatePlanetMesh>,
) {
    let mut changed = false;

    egui::Window::new("Shape Settings").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("Radius:");
            let old = shape_gen.radius;
            ui.add(egui::widgets::DragValue::new(&mut shape_gen.radius).clamp_range(0f32..=100f32).min_decimals(1).speed(0.2));
            changed = changed || (old != shape_gen.radius);
        });
        
        ui.horizontal(|ui| {
            ui.label("FBM Layers:");
            let old = shape_gen.noise.num_layers;
            ui.add(egui::widgets::DragValue::new(&mut shape_gen.noise.num_layers).clamp_range(0..=8).min_decimals(2).speed(0.025));
            changed = changed || (old != shape_gen.noise.num_layers);
        });
        
        ui.horizontal(|ui| {
            ui.label("Noise Strength:");
            let old = shape_gen.noise.strength;
            ui.add(egui::widgets::DragValue::new(&mut shape_gen.noise.strength).clamp_range(0f32..=100f32).min_decimals(2).speed(0.025));
            changed = changed || (old != shape_gen.noise.strength);
        });
        
        ui.horizontal(|ui| {
            ui.label("Noise Roughness:");
            let old = shape_gen.noise.roughness;
            ui.add(egui::widgets::DragValue::new(&mut shape_gen.noise.roughness).clamp_range(0f32..=100f32).min_decimals(2).speed(0.025));
            changed = changed || (old != shape_gen.noise.roughness);
        });
        
        ui.horizontal(|ui| {
            ui.label("Noise Frequency:");
            let old = shape_gen.noise.frequency;
            ui.add(egui::widgets::DragValue::new(&mut shape_gen.noise.frequency).clamp_range(0f32..=100f32).min_decimals(2).speed(0.025));
            changed = changed || (old != shape_gen.noise.frequency);
        });
        
        ui.horizontal(|ui| {
            ui.label("Noise Persistence:");
            let old = shape_gen.noise.persistence;
            ui.add(egui::widgets::DragValue::new(&mut shape_gen.noise.persistence).clamp_range(0f32..=100f32).min_decimals(2).speed(0.025));
            changed = changed || (old != shape_gen.noise.persistence);
        });

        ui.horizontal(|ui| {
            ui.label("Noise Center:");
            let old = shape_gen.noise.center;
            ui.add(egui::widgets::DragValue::new(&mut shape_gen.noise.center.x).prefix("X: ").clamp_range(0f32..=100f32).min_decimals(2).speed(0.025));
            ui.add(egui::widgets::DragValue::new(&mut shape_gen.noise.center.y).prefix("Y: ").clamp_range(0f32..=100f32).min_decimals(2).speed(0.025));
            ui.add(egui::widgets::DragValue::new(&mut shape_gen.noise.center.z).prefix("Z: ").clamp_range(0f32..=100f32).min_decimals(2).speed(0.025));
            changed = changed || (old != shape_gen.noise.center);
        });
    });

    if changed {
        update_planet_mesh_evw.send(UpdatePlanetMesh {});
    }
}
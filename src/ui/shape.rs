use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{gen::{shape::ShapeGenerator, noise_filter::{NoiseLayer, NoiseFilterType}}, render::planet::UpdatePlanetMesh};


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
            ui.add(egui::widgets::DragValue::new(&mut shape_gen.radius).clamp_range(0f32..=100f32).min_decimals(2).speed(0.025));
            changed = changed || (old != shape_gen.radius);
        });

        ui.horizontal(|ui| {
            ui.label("Noise Layers:");
            if ui.small_button("-").clicked() && shape_gen.num_layers > 1 {
                shape_gen.num_layers -= 1;
                shape_gen.noise_layers.pop();
            }
            ui.label(format!("{}", shape_gen.num_layers));
            if ui.small_button("+").clicked() {
                shape_gen.num_layers += 1;
                let num_layers = shape_gen.num_layers;
                shape_gen.noise_layers.push(NoiseLayer::new(num_layers, false));
            }
        });
        
        let num_layers = shape_gen.num_layers;
        for i in 0..num_layers {
            egui::containers::CollapsingHeader::new(format!("Layer {}", i + 1)).show(ui, |ui| {
                let layer = &mut shape_gen.noise_layers[i as usize];

                ui.horizontal(|ui| {
                    ui.label("Filter Type:");
                    let old = layer.filter.ty.clone();
                    egui::ComboBox::from_id_source(0)
                        .selected_text(format!("{:?}", layer.filter.ty))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut layer.filter.ty, NoiseFilterType::Standard, "Standard");
                            ui.selectable_value(&mut layer.filter.ty, NoiseFilterType::Rigid, "Rigid");
                            ui.selectable_value(&mut layer.filter.ty, NoiseFilterType::Warp, "Warp");
                        });
                    layer.is_warp = layer.filter.ty == NoiseFilterType::Warp;
                    changed = changed || (old != layer.filter.ty);
                });

                ui.horizontal(|ui| {
                    ui.label("Enabled:");
                    let old = layer.enabled;
                    ui.add(egui::widgets::Checkbox::without_text(&mut layer.enabled));
                    changed = changed || (old != layer.enabled);
                });

                ui.horizontal(|ui| {
                    ui.label("Use First Layer As Mask:");
                    let old = layer.first_layer_mask;
                    ui.add(egui::widgets::Checkbox::without_text(&mut layer.first_layer_mask));
                    changed = changed || (old != layer.first_layer_mask);
                });

                if layer.is_warp {
                    ui.horizontal(|ui| {
                        ui.label("Warp Target Layer:");
                        let old = layer.warp_target;
                        ui.add(egui::DragValue::new(&mut layer.warp_target).clamp_range(1..=num_layers).max_decimals(0).speed(0.05));
                        changed = changed || (old != layer.warp_target);
                    });

                    ui.horizontal(|ui| {
                        ui.label("Warp Offset:");
                        let old = layer.filter.warp_offset;
                        ui.add(egui::widgets::DragValue::new(&mut layer.filter.warp_offset.x).prefix("X: ").clamp_range(0f32..=100f32).min_decimals(2).speed(0.025));
                        ui.add(egui::widgets::DragValue::new(&mut layer.filter.warp_offset.y).prefix("Y: ").clamp_range(0f32..=100f32).min_decimals(2).speed(0.025));
                        ui.add(egui::widgets::DragValue::new(&mut layer.filter.warp_offset.z).prefix("Z: ").clamp_range(0f32..=100f32).min_decimals(2).speed(0.025));
                        changed = changed || (old != layer.filter.warp_offset);
                    });
                }

                ui.horizontal(|ui| {
                    ui.label("Noise Octaves:");
                    let old = layer.filter.num_octaves;
                    ui.add(egui::widgets::DragValue::new(&mut layer.filter.num_octaves).clamp_range(0..=8).max_decimals(0).speed(0.05));
                    changed = changed || (old != layer.filter.num_octaves);
                });
                
                ui.horizontal(|ui| {
                    ui.label("Noise Strength:");
                    let old = layer.filter.strength;
                    ui.add(egui::widgets::DragValue::new(&mut layer.filter.strength).clamp_range(0f32..=100f32).min_decimals(2).speed(0.025));
                    changed = changed || (old != layer.filter.strength);
                });
                
                ui.horizontal(|ui| {
                    ui.label("Noise Roughness:");
                    let old = layer.filter.roughness;
                    ui.add(egui::widgets::DragValue::new(&mut layer.filter.roughness).clamp_range(0f32..=100f32).min_decimals(2).speed(0.025));
                    changed = changed || (old != layer.filter.roughness);
                });
                
                ui.horizontal(|ui| {
                    ui.label("Noise Lacunarity:");
                    let old = layer.filter.lacunarity;
                    ui.add(egui::widgets::DragValue::new(&mut layer.filter.lacunarity).clamp_range(0f32..=100f32).min_decimals(2).speed(0.025));
                    changed = changed || (old != layer.filter.lacunarity);
                });
                
                ui.horizontal(|ui| {
                    ui.label("Noise Persistence:");
                    let old = layer.filter.persistence;
                    ui.add(egui::widgets::DragValue::new(&mut layer.filter.persistence).clamp_range(0f32..=100f32).min_decimals(2).speed(0.025));
                    changed = changed || (old != layer.filter.persistence);
                });
                
                ui.horizontal(|ui| {
                    ui.label("Vertical Offset:");
                    let old = layer.filter.offset;
                    ui.add(egui::widgets::DragValue::new(&mut layer.filter.offset).clamp_range(0f32..=1f32).min_decimals(2).speed(0.025));
                    changed = changed || (old != layer.filter.offset);
                });
                
                ui.horizontal(|ui| {
                    ui.label("Floor:");
                    let old = layer.filter.floor;
                    ui.add(egui::widgets::DragValue::new(&mut layer.filter.floor).clamp_range(0f32..=1f32).min_decimals(2).speed(0.025));
                    changed = changed || (old != layer.filter.floor);
                });
        
                ui.horizontal(|ui| {
                    ui.label("Noise Center:");
                    let old = layer.filter.center;
                    ui.add(egui::widgets::DragValue::new(&mut layer.filter.center.x).prefix("X: ").clamp_range(0f32..=100f32).min_decimals(2).speed(0.025));
                    ui.add(egui::widgets::DragValue::new(&mut layer.filter.center.y).prefix("Y: ").clamp_range(0f32..=100f32).min_decimals(2).speed(0.025));
                    ui.add(egui::widgets::DragValue::new(&mut layer.filter.center.z).prefix("Z: ").clamp_range(0f32..=100f32).min_decimals(2).speed(0.025));
                    changed = changed || (old != layer.filter.center);
                });
            });
        }
    });

    if changed {
        update_planet_mesh_evw.send(UpdatePlanetMesh {});
    }
}
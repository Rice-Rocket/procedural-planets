use serde::{Serialize, Deserialize};

use crate::gen::{shape::ShapeGenerator, noise::NoiseSimplex3d};

use super::{color::UiColorSettings, render::UiRenderSettings};


#[derive(Serialize, Deserialize)]
pub struct SaveState {
    pub shape_gen: ShapeGenerator,
    pub colors: UiColorSettings,
    pub settings: UiRenderSettings,
}


pub fn restore_save(
    save: SaveState,
    settings: &mut UiRenderSettings,
    shape_gen: &mut ShapeGenerator,
    colors: &mut UiColorSettings,
) {
    *settings = save.settings;
    *colors = save.colors;
    *shape_gen = save.shape_gen;

    for layer in shape_gen.noise_layers.iter_mut() {
        layer.filter.simplex_3d = NoiseSimplex3d::new(layer.filter.noise_seed);
    }
}
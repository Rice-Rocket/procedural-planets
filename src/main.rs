use bevy::prelude::*;

pub mod gen;
pub mod render;
pub mod ui;

use bevy_egui::EguiPlugin;
use gen::*;
use render::*;
use ui::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: bevy::window::PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        .add_plugins((GeneratorPlugin, RenderPlugin, UIPlugin))
        .run();
}

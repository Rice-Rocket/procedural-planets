use bevy::{prelude::*, pbr::wireframe::WireframePlugin, core_pipeline::experimental::taa::TemporalAntiAliasPlugin};

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
        .add_plugins(WireframePlugin)
        .add_plugins(EguiPlugin)
        .add_plugins((GeneratorPlugin, RenderPlugin, UIPlugin))
        .insert_resource(Msaa::Off)
        .add_plugins(TemporalAntiAliasPlugin)
        .run();
}

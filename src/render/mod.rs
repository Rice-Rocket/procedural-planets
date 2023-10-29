pub mod planet;
pub mod light;
pub mod planet_mat;
// pub mod ocean_mat;
pub mod ocean;
pub mod utils;
pub mod atmosphere;
pub mod post;

use bevy::prelude::*;

use planet::*;
use light::*;
use planet_mat::*;
// use ocean_mat::*;
use ocean::*;
use atmosphere::*;
use post::*;


pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Planet>()
            .add_event::<UpdatePlanetMesh>()
            .add_event::<UpdatePlanetMaterials>()
            .add_plugins(PostProcessPlugin)
            .add_plugins(MaterialPlugin::<PlanetMaterial>::default())
            .add_plugins(MaterialPlugin::<OceanMaterial> {
                prepass_enabled: false,
                ..default()
            })
            .add_systems(Startup, (
                spawn_planet,
                spawn_directional_light,
                spawn_ocean,
            ))
            .add_systems(Update, (
                generate_mesh,
                generate_materials,
                update_directional_light,
                update_ocean,
                update_atmosphere,
                update_planet_material,
            ))
        ;
    }
}
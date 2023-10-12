pub mod planet;
pub mod light;

use bevy::prelude::*;

use planet::*;
use light::*;


pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Planet>()
            .add_event::<UpdatePlanetMesh>()
            .add_event::<UpdatePlanetMaterials>()
            .add_systems(Startup, (
                spawn_planet,
                spawn_directional_light
            ))
            .add_systems(Update, (
                generate_mesh,
                generate_colors,
                update_directional_light,
            ))
        ;
    }
}
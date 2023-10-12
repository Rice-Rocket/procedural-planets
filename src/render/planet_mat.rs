use bevy::{prelude::*, reflect::{TypeUuid, TypePath}, render::render_resource::AsBindGroup};


#[derive(AsBindGroup, Debug, Clone, TypeUuid, TypePath)]
#[uuid = "717f64fe-6844-4822-8926-e0ed374294c8"]
pub struct PlanetMaterial {
    #[uniform(0)]
    pub min_elevation: f32,
    #[uniform(0)]
    pub max_elevation: f32,
}

impl Material for PlanetMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/planet.wgsl".into()
    }
}

// #[derive(Component)]
// pub struct PlanetMaterialData {
//     pub min_elevation: f32,
//     pub max_elevation: f32,
// }


// pub fn prepare_planet_material(
//     material: Res<RenderMaterials<PlanetMaterial>>,
    
// ) {

// }
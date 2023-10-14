use bevy::{prelude::*, reflect::{TypeUuid, TypePath}, render::render_resource::AsBindGroup};

use crate::ui::render::UiRenderSettings;

// ! IDEA: to get depth prepass in post processing shader, just pass in the depth
// ! texture manually, following bevy's source code



#[derive(AsBindGroup, Debug, Clone, TypeUuid, TypePath)]
#[uuid = "4c5eb867-07fc-4e23-b30c-2169f38ba1de"]
pub struct OceanMaterial {
    #[uniform(0)]
    pub radius: f32,
    #[uniform(0)]
    pub depth_mul: f32,
    #[uniform(0)]
    pub alpha_mul: f32,
    #[uniform(0)]
    pub color_1: Vec4,
    #[uniform(0)]
    pub color_2: Vec4,
}

impl Material for OceanMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/water.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

impl Default for OceanMaterial {
    fn default() -> Self {
        Self {
            radius: 1.0,
            depth_mul: 1.0, 
            alpha_mul: 1.0,
            color_1: Vec4::ZERO,
            color_2: Vec4::ONE,
        }
    }
}


pub fn spawn_ocean(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut ocean_materials: ResMut<Assets<OceanMaterial>>,
) {
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::try_from(shape::Icosphere { radius: 1.0, subdivisions: 5 }).unwrap()),
        material: ocean_materials.add(OceanMaterial::default()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}

pub fn update_ocean(
    mut ocean_transforms: Query<(&mut Transform, &Handle<OceanMaterial>)>,
    mut ocean_materials: ResMut<Assets<OceanMaterial>>,
    render_settings: Res<UiRenderSettings>,
) {
    for (mut transform, mat_handle) in ocean_transforms.iter_mut() {
        transform.scale.x = render_settings.ocean_radius;
        transform.scale.y = render_settings.ocean_radius;
        transform.scale.z = render_settings.ocean_radius;

        let mat = ocean_materials.get_mut(mat_handle).unwrap();
        mat.radius = render_settings.ocean_radius;
        mat.depth_mul = render_settings.ocean_depth_mul;
        mat.alpha_mul = render_settings.ocean_alpha_mul;
        mat.color_1 = Vec4::new(render_settings.ocean_color_1[0], render_settings.ocean_color_1[1], render_settings.ocean_color_1[2], 1.0);
        mat.color_2 = Vec4::new(render_settings.ocean_color_2[0], render_settings.ocean_color_2[1], render_settings.ocean_color_2[2], 1.0);
    }
}
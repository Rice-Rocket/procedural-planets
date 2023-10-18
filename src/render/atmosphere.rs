use bevy::{prelude::*, reflect::{TypeUuid, TypePath}, render::render_resource::AsBindGroup};

use crate::ui::render::UiRenderSettings;

// ! IDEA: to get depth prepass in post processing shader, just pass in the depth
// ! texture manually, following bevy's source code



#[derive(AsBindGroup, Debug, Clone, TypeUuid, TypePath)]
#[uuid = "c6ec9a0b-d50f-495f-b753-09ef16f81c2d"]
pub struct AtmosphereMaterial {
    #[uniform(0)]
    pub radius: f32,
    #[uniform(0)]
    pub ocean_radius: f32,
    #[uniform(0)]
    pub num_sample_points: u32,
    #[uniform(0)]
    pub num_optical_depth_points: u32,
    #[uniform(0)]
    pub density_falloff: f32,
    #[uniform(0)]
    pub scattering_coeffs: Vec3,
}

impl Material for AtmosphereMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/atmosphere.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

impl Default for AtmosphereMaterial {
    fn default() -> Self {
        Self {
            radius: 1.0,
            ocean_radius: 1.0,
            num_sample_points: 10,
            num_optical_depth_points: 10,
            density_falloff: 1.0,
            scattering_coeffs: Vec3::new(700.0, 530.0, 440.0),
        }
    }
}


pub fn spawn_atmosphere(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut atmosphere_materials: ResMut<Assets<AtmosphereMaterial>>,
) {
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::try_from(shape::Icosphere { radius: 1.0, subdivisions: 6 }).unwrap()),
        material: atmosphere_materials.add(AtmosphereMaterial::default()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}

pub fn update_atmosphere(
    mut atmosphere_transforms: Query<(&mut Transform, &Handle<AtmosphereMaterial>)>,
    mut atmosphere_materials: ResMut<Assets<AtmosphereMaterial>>,
    render_settings: Res<UiRenderSettings>,
) {
    for (mut transform, mat_handle) in atmosphere_transforms.iter_mut() {
        transform.scale.x = render_settings.atmosphere_radius * 1.0;
        transform.scale.y = render_settings.atmosphere_radius * 1.0;
        transform.scale.z = render_settings.atmosphere_radius * 1.0;

        let mat = atmosphere_materials.get_mut(mat_handle).unwrap();
        
        mat.radius = render_settings.atmosphere_radius;
        mat.ocean_radius = render_settings.ocean_radius;
        mat.num_sample_points = render_settings.atmosphere_sample_points;
        mat.num_optical_depth_points = render_settings.atmosphere_optical_depth_points;
        mat.density_falloff = render_settings.atmosphere_density_falloff;

        let mut scatter_coeffs = Vec3::from(render_settings.atmosphere_scatter_coeffs);
        scatter_coeffs = 400.0 / scatter_coeffs;
        scatter_coeffs = scatter_coeffs * scatter_coeffs * scatter_coeffs * scatter_coeffs;
        scatter_coeffs = scatter_coeffs * render_settings.atmosphere_scatter_strength;
        mat.scattering_coeffs = scatter_coeffs;
    }
}
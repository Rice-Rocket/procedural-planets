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
    pub smoothness: f32,
    #[uniform(0)]
    pub color_1: Vec4,
    #[uniform(0)]
    pub color_2: Vec4,
    #[uniform(0)]
    pub time: f32,
    #[uniform(0)]
    pub wave_strength: f32,
    #[uniform(0)]
    pub wave_speed: f32,
    #[uniform(0)]
    pub wave_scale: f32,
    
    #[texture(1)]
    #[sampler(2)]
    pub wave_normals_1: Option<Handle<Image>>,
    #[texture(3)]
    #[sampler(4)]
    pub wave_normals_2: Option<Handle<Image>>,
    selected_normal_map_1: u32,
    selected_normal_map_2: u32,
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
            smoothness: 1.0,
            color_1: Vec4::ZERO,
            color_2: Vec4::ONE,
            wave_strength: 0.1,
            time: 0.0,
            wave_speed: 1.0,
            wave_scale: 1.0,
            wave_normals_1: None,
            wave_normals_2: None,
            selected_normal_map_1: 1,
            selected_normal_map_2: 1,
        }
    }
}


pub fn spawn_ocean(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut ocean_materials: ResMut<Assets<OceanMaterial>>,
) {
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::try_from(shape::Icosphere { radius: 1.0, subdivisions: 6 }).unwrap()),
        material: ocean_materials.add(OceanMaterial::default()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}

pub fn update_ocean(
    mut ocean_transforms: Query<(&mut Transform, &Handle<OceanMaterial>)>,
    mut ocean_materials: ResMut<Assets<OceanMaterial>>,
    asset_server: Res<AssetServer>,
    render_settings: Res<UiRenderSettings>,
    time: Res<Time>,
) {
    for (mut transform, mat_handle) in ocean_transforms.iter_mut() {
        transform.scale.x = render_settings.ocean_radius * 1.0;
        transform.scale.y = render_settings.ocean_radius * 1.0;
        transform.scale.z = render_settings.ocean_radius * 1.0;

        let mat = ocean_materials.get_mut(mat_handle).unwrap();
        mat.time = time.elapsed_seconds();
        mat.wave_speed = render_settings.wave_speed;
        mat.wave_scale = render_settings.wave_scale;
        mat.wave_strength = render_settings.wave_strength;
        mat.radius = render_settings.ocean_radius;
        mat.depth_mul = render_settings.ocean_depth_mul;
        mat.alpha_mul = render_settings.ocean_alpha_mul;
        mat.smoothness = render_settings.ocean_smoothness;
        mat.color_1 = Vec4::new(render_settings.ocean_color_1[0], render_settings.ocean_color_1[1], render_settings.ocean_color_1[2], 1.0);
        mat.color_2 = Vec4::new(render_settings.ocean_color_2[0], render_settings.ocean_color_2[1], render_settings.ocean_color_2[2], 1.0);

        if mat.wave_normals_1.is_none() || mat.selected_normal_map_1 != render_settings.waves_normal_map_1 {
            mat.selected_normal_map_1 = render_settings.waves_normal_map_1;
            mat.wave_normals_1 = Some(asset_server.load(format!("textures/normals/waves_{}.png", render_settings.waves_normal_map_1)));
        }
        if mat.wave_normals_2.is_none() || mat.selected_normal_map_2 != render_settings.waves_normal_map_2 {
            mat.selected_normal_map_2 = render_settings.waves_normal_map_2;
            mat.wave_normals_2 = Some(asset_server.load(format!("textures/normals/waves_{}.png", render_settings.waves_normal_map_2)));
        }
    }
}
use bevy::{prelude::*, render::{mesh::Indices, render_resource::{PrimitiveTopology, Extent3d, TextureDimension, TextureFormat}, texture::TextureFormatPixelInfo}};

use crate::{ui::color::UiColorSettings, gen::shape::ShapeGenerator};

use super::planet_mat::{PlanetMaterial, ColorGradient};


#[derive(Resource)]
pub struct Planet {
    pub resolution: u32,
    pub position: Vec3,
    terrain_faces: [Entity; 6],
}

impl Default for Planet {
    fn default() -> Self {
        Self {
            resolution: 10,
            position: Vec3::ZERO,
            terrain_faces: [Entity::PLACEHOLDER; 6],
        }
    }
}

#[derive(Component)]
pub struct TerrainFace {
    local_up: Vec3,
    axis_a: Vec3,
    axis_b: Vec3,
}

impl TerrainFace {
    pub fn new(local_up: Vec3) -> Self {
        let axis_a = Vec3::new(local_up.y, local_up.z, local_up.x);
        let axis_b = local_up.cross(axis_a);

        Self {
            local_up,
            axis_a,
            axis_b,
        }
    }
}

#[derive(Event)]
pub struct UpdatePlanetMesh {}

#[derive(Event)]
pub struct UpdatePlanetMaterials {}


pub fn spawn_planet(
    mut commands: Commands,
    mut planet: ResMut<Planet>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PlanetMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut update_planet_mesh_evw: EventWriter<UpdatePlanetMesh>
) {
    let directions = [Vec3::Y, Vec3::NEG_Y, Vec3::X, Vec3::NEG_X, Vec3::Z, Vec3::NEG_Z];
    for i in 0..6 {
        let mesh = meshes.add(Mesh::new(PrimitiveTopology::TriangleList));
        let mat = PlanetMaterial {
            min_elevation: 0.0,
            max_elevation: 0.0,
            elevation_gradient: images.add(Image::new_fill(
                Extent3d {
                    width: ColorGradient::RESOLUTION,
                    height: 1,
                    depth_or_array_layers: 1,
                },
                TextureDimension::D2,
                &[0; 4],
                TextureFormat::Rgba8Unorm,
            ))
        };
        planet.terrain_faces[i] = commands.spawn((MaterialMeshBundle {
            mesh: mesh.clone(),
            material: materials.add(mat),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        }, TerrainFace::new(
            directions[i]
        ))).id();
    }
    update_planet_mesh_evw.send(UpdatePlanetMesh {});
}


pub fn generate_mesh(
    terrain_faces: Query<(&TerrainFace, &Handle<Mesh>)>,
    face_materials: Query<&Handle<PlanetMaterial>, With<TerrainFace>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PlanetMaterial>>,
    planet: Res<Planet>,
    shape_gen: Res<ShapeGenerator>,
    mut update_planet_mesh_evr: EventReader<UpdatePlanetMesh>,
) {
    for _update_planet_mesh_ev in update_planet_mesh_evr.iter() {
        let mut min_elevation = f32::MAX;
        let mut max_elevation = f32::MIN;

        for (face, mesh_handle) in terrain_faces.iter() {
            let mut vertices = vec![Vec3::ZERO; (planet.resolution * planet.resolution) as usize];
            let mut uvs = vec![Vec2::ZERO; (planet.resolution * planet.resolution) as usize];
            let mut triangles = vec![0u32; ((planet.resolution - 1) * (planet.resolution - 1) * 6) as usize];
            let mut tri_index = 0;
    
            for y in 0u32..planet.resolution {
                for x in 0u32..planet.resolution {
                    let i = y * planet.resolution + x;
                    let uv = Vec2::new(x as f32, y as f32) / (planet.resolution as f32 - 1.0);

                    let point_on_cube = face.local_up + (uv.x - 0.5) * 2.0 * face.axis_a + (uv.y - 0.5) * 2.0 * face.axis_b;
                    let point_on_sphere = point_on_cube.normalize();
                    let (position, elevation) = shape_gen.get_point_on_planet(point_on_sphere);

                    if elevation > max_elevation {
                        max_elevation = elevation;
                    }
                    if elevation < min_elevation {
                        min_elevation = elevation;
                    }

                    vertices[i as usize] = position;
                    uvs[i as usize] = uv;
    
                    if x != planet.resolution - 1 && y != planet.resolution - 1 {
                        triangles[tri_index] = i;
                        triangles[tri_index + 1] = i + planet.resolution + 1;
                        triangles[tri_index + 2] = i + planet.resolution;
    
                        triangles[tri_index + 3] = i;
                        triangles[tri_index + 4] = i + 1;
                        triangles[tri_index + 5] = i + planet.resolution + 1;
    
                        tri_index += 6;
                    }
                }
            }
    
            let mesh = meshes.get_mut(&mesh_handle).unwrap();
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
            mesh.set_indices(Some(Indices::U32(triangles)));
            mesh.duplicate_vertices();
            mesh.compute_flat_normals();
        }

        for mat_handle in face_materials.iter() {
            let mat = materials.get_mut(mat_handle).unwrap();
            mat.min_elevation = min_elevation;
            mat.max_elevation = max_elevation;
        }
    }
}

pub fn generate_materials(
    terrain_faces: Query<&Handle<PlanetMaterial>, With<TerrainFace>>,
    mut materials: ResMut<Assets<PlanetMaterial>>,
    mut images: ResMut<Assets<Image>>,
    color_settings: Res<UiColorSettings>,
    mut update_planet_mats_evr: EventReader<UpdatePlanetMaterials>,
) {
    for _update_planet_mats_ev in update_planet_mats_evr.iter() {
        for mat_handle in terrain_faces.iter() {
            let mat = materials.get_mut(mat_handle).unwrap();
            let elevation_grad_img = images.get_mut(&mat.elevation_gradient).unwrap();

            for (i, col) in color_settings.elevation_colors.interpolated().iter().enumerate() {
                elevation_grad_img.data[i * TextureFormat::Rgba8Unorm.pixel_size() + 0] = (col.r() * 255.0) as u8;
                elevation_grad_img.data[i * TextureFormat::Rgba8Unorm.pixel_size() + 1] = (col.g() * 255.0) as u8;
                elevation_grad_img.data[i * TextureFormat::Rgba8Unorm.pixel_size() + 2] = (col.b() * 255.0) as u8;
            }
        }
    }
}
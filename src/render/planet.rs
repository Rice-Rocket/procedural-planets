use bevy::{prelude::*, render::{mesh::Indices, render_resource::PrimitiveTopology}};

use crate::{ui::color::UiColorSettings, gen::shape::ShapeGenerator};

use super::planet_mat::{PlanetMaterial, ColorEntry};


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
    mut update_planet_mesh_evw: EventWriter<UpdatePlanetMesh>
) {
    let directions = [Vec3::Y, Vec3::NEG_Y, Vec3::X, Vec3::NEG_X, Vec3::Z, Vec3::NEG_Z];
    for i in 0..6 {
        let mesh = meshes.add(Mesh::new(PrimitiveTopology::TriangleList));
        let mat = PlanetMaterial::default();
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
            let mesh = meshes.get_mut(&mesh_handle).unwrap();
            mesh.remove_attribute(Mesh::ATTRIBUTE_POSITION);
            mesh.remove_attribute(Mesh::ATTRIBUTE_NORMAL);
            mesh.remove_attribute(Mesh::ATTRIBUTE_UV_0);
            mesh.set_indices(None);

            let num_triangles = ((planet.resolution - 1) * (planet.resolution - 1) * 2) as usize;
            let mut positions = vec![Vec3::ZERO; (planet.resolution * planet.resolution) as usize];
            let mut uvs = vec![Vec2::ZERO; (planet.resolution * planet.resolution) as usize];
            let mut normals = vec![Vec3::ZERO; (planet.resolution * planet.resolution) as usize];
            let mut indices = vec![0u32; num_triangles * 3];
            let mut tri_index = 0;

            for y in 0u32..planet.resolution {
                for x in 0u32..planet.resolution {
                    let i = y * planet.resolution + x;
                    let uv = Vec2::new(x as f32, y as f32) / (planet.resolution as f32 - 1.0);

                    let point_on_cube = face.local_up + (uv.x - 0.5) * 2.0 * face.axis_a + (uv.y - 0.5) * 2.0 * face.axis_b;
                    let point_on_sphere = point_on_cube.normalize();
                    let (position, elevation) = shape_gen.get_point_and_elevation(point_on_sphere);

                    if elevation > max_elevation {
                        max_elevation = elevation;
                    }
                    if elevation < min_elevation {
                        min_elevation = elevation;
                    }

                    positions[i as usize] = position;
                    uvs[i as usize] = uv;
    
                    if x != planet.resolution - 1 && y != planet.resolution - 1 {
                        indices[tri_index] = i;
                        indices[tri_index + 1] = i + planet.resolution + 1;
                        indices[tri_index + 2] = i + planet.resolution;
    
                        indices[tri_index + 3] = i;
                        indices[tri_index + 4] = i + 1;
                        indices[tri_index + 5] = i + planet.resolution + 1;
    
                        tri_index += 6;
                    }
                }
            }

            for i in 0..num_triangles {
                let i0 = indices[i * 3 + 0] as usize;
                let i1 = indices[i * 3 + 1] as usize;
                let i2 = indices[i * 3 + 2] as usize;

                let p0 = positions[i0];
                let p1 = positions[i1];
                let p2 = positions[i2];

                let face_normal = (p1 - p0).cross(p2 - p0);
                normals[i0] += face_normal;
                normals[i1] += face_normal;
                normals[i2] += face_normal;
            }

            normals = normals.iter().map(|x| x.normalize()).collect();
    
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
            mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
            mesh.set_indices(Some(Indices::U32(indices)));
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
    color_settings: Res<UiColorSettings>,
    mut update_planet_mats_evr: EventReader<UpdatePlanetMaterials>,
) {
    for _update_planet_mats_ev in update_planet_mats_evr.iter() {
        for mat_handle in terrain_faces.iter() {
            let mat = materials.get_mut(mat_handle).unwrap();
            mat.n_colors = color_settings.colors.count();

            for (i, col) in color_settings.colors.sorted().iter().enumerate() {
                mat.colors[i] = ColorEntry::new(col.0, col.1, col.2);
            }
        }
    }
}

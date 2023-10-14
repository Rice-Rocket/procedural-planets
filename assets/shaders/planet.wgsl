#import bevy_pbr::pbr_functions as pbr_functions
#import bevy_pbr::pbr_bindings as pbr_bindings
#import bevy_pbr::pbr_types as pbr_types

#import bevy_core_pipeline::tonemapping as tonemapping

#import bevy_pbr::mesh_types MESH_FLAGS_SHADOW_RECEIVER_BIT

#import bevy_pbr::mesh_vertex_output MeshVertexOutput
#import bevy_pbr::mesh_view_bindings view

struct PlanetMaterial {
    min_elevation: f32,
    max_elevation: f32,
};

@group(1) @binding(0)
var<uniform> planet: PlanetMaterial;
@group(1) @binding(1)
var elevation_gradient_texture: texture_2d<f32>;
@group(1) @binding(2)
var elevation_gradient_sampler: sampler;


fn lerp(a: vec3<f32>, b: vec3<f32>, t: f32) -> vec3<f32> {
    return a + (b - a) * t;
}


@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {

    // * Get planet color
    
    let elevation = length(in.world_position.xyz);
    let norm_elevation = (elevation - planet.min_elevation) / (planet.max_elevation - planet.min_elevation);
    let planet_col = textureSample(elevation_gradient_texture, elevation_gradient_sampler, vec2(norm_elevation, 0.0)).xyz;

    // * Get Built in Bevy PBR Result

    var uv = in.uv;
    let is_orthographic = false;
    var pbr_in = pbr_functions::pbr_input_new();
    pbr_in.flags = MESH_FLAGS_SHADOW_RECEIVER_BIT;

    pbr_in.material.base_color = vec4(planet_col, 1.0);
    pbr_in.material.emissive = vec4(0., 0., 0., 1.);

    pbr_in.material.metallic = 0.0;
    pbr_in.material.perceptual_roughness = 0.5;
    // pbr_in.material.perceptual_roughness = 0.045;
    pbr_in.material.reflectance = 0.5;

    pbr_in.frag_coord = in.position;
    pbr_in.world_position = in.world_position;
    pbr_in.world_normal = pbr_functions::prepare_world_normal(
        in.world_normal,
        (pbr_in.material.flags & pbr_types::STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT) != 0u,
        true, 
    );

    pbr_in.N = pbr_functions::apply_normal_mapping(pbr_in.material.flags, pbr_in.world_normal, in.uv, view.mip_bias);
    pbr_in.V = pbr_functions::calculate_view(in.world_position, is_orthographic);

    var pbr_result = pbr_functions::pbr(pbr_in);

    pbr_result = tonemapping::tone_mapping(pbr_result, view.color_grading);

    return pbr_result;
}
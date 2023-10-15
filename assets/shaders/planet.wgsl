#import bevy_pbr::pbr_functions as pbr_functions
#import bevy_pbr::pbr_bindings as pbr_bindings
#import bevy_pbr::pbr_types as pbr_types

#import bevy_core_pipeline::tonemapping as tonemapping

#import bevy_pbr::mesh_types MESH_FLAGS_SHADOW_RECEIVER_BIT

#import bevy_pbr::mesh_vertex_output MeshVertexOutput
#import bevy_pbr::mesh_view_bindings view

struct ColorEntry {
    color: vec3<f32>,
    elevation: f32,
    _padding: vec3<f32>,
    steepness: f32,
}

struct PlanetMaterial {
    min_elevation: f32,
    max_elevation: f32,
    n_colors: u32,
    #ifdef SIXTEEN_BYTE_ALIGNMENT
    _webgl2_padding: f32,
    #endif
};

@group(1) @binding(0)
var<uniform> planet: PlanetMaterial;
@group(1) @binding(1)
var<storage, read> colors: array<ColorEntry>;


fn lerp3(a: vec3<f32>, b: vec3<f32>, t: f32) -> vec3<f32> {
    return a + (b - a) * t;
}

fn inv_lerp(v: f32, a: f32, b: f32) -> f32 {
    return saturate((v - a) / (b - a));
}


@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {

    // * Get planet color

    // ! Consider steepness/elevation as 2d space. 
    // ! Basically, make the thing interpolate in a 2d texture
    // ! ^^^ Just an idea
    
    let elevation = length(in.world_position.xyz);
    let norm_elevation = inv_lerp(elevation, planet.min_elevation, planet.max_elevation);

    let local_up = in.world_position.xyz / elevation;
    let steepness = 1.0 - dot(in.world_normal, local_up);

    let color_pos = vec2(norm_elevation, steepness);
    var planet_col = vec3(0.0);
    var amount = 0.0;
    for (var i = 0u; i < planet.n_colors; i++) {
        let col_entry = colors[i];
        let position = vec2(col_entry.elevation, col_entry.steepness);
        let dist = distance(color_pos, position) / 1.5 + 0.000001;
        let gauss_dist = 1.0 - exp(-dist * dist * 10.0);
        let strength = 1.0 - min(0.000001, 1.0 * log(gauss_dist)) - 1.0;

        planet_col = (col_entry.color * strength + planet_col * amount) / (amount + strength);
        amount += strength;
    }

    // * Get Built in Bevy PBR Result

    var uv = in.uv;
    let is_orthographic = false;
    var pbr_in = pbr_functions::pbr_input_new();
    pbr_in.flags = MESH_FLAGS_SHADOW_RECEIVER_BIT;

    pbr_in.material.base_color = vec4(planet_col, 1.0);
    pbr_in.material.emissive = vec4(0., 0., 0., 1.);

    pbr_in.material.metallic = 0.0;
    pbr_in.material.perceptual_roughness = 0.8;
    pbr_in.material.reflectance = 0.3;

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
#import bevy_pbr::mesh_vertex_output MeshVertexOutput
#import bevy_pbr::mesh_view_bindings as view_bindings
#import bevy_pbr::shadows as shadows

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
    normal_strength: f32,
    normal_scale: f32,
    #ifdef SIXTEEN_BYTE_ALIGNMENT
    _webgl2_padding: vec3<f32>,
    #endif
};

@group(1) @binding(0) var<uniform> planet: PlanetMaterial;
@group(1) @binding(1) var<storage, read> colors: array<ColorEntry>;
@group(1) @binding(2) var surface_normals_texture: texture_2d<f32>;
@group(1) @binding(3) var surface_normals_sampler: sampler;

fn inv_lerp(v: f32, a: f32, b: f32) -> f32 {
    return saturate((v - a) / (b - a));
}

fn unpack_normal(normal: vec4<f32>) -> vec3<f32> {
    return normal.xyz * 2.0 - 0.5;
}

fn blend_rnm(a: vec3<f32>, b: vec3<f32>) -> vec3<f32> {
    var n1 = a;
    var n2 = b;

    n1.z += 1.0;
    n2 *= vec3(-1.0, -1.0, 1.0);

    return n1 * dot(n1, n2) / n1.z - n2;
}

fn triplanar_normal(pos: vec3<f32>, normal: vec3<f32>, scale: f32, offset: vec2<f32>, map_texture: texture_2d<f32>, map_sampler: sampler) -> vec3<f32> {
    let abs_normal = abs(normal);

    var blend_weight = saturate(normal * normal * normal * normal);
    blend_weight /= dot(blend_weight, vec3(1.0));

    let uv_x = pos.zy * scale + offset;
    let uv_y = pos.xz * scale + offset;
    let uv_z = pos.xy * scale + offset;

    var tan_normal_x = unpack_normal(textureSample(map_texture, map_sampler, fract(uv_x)));
    var tan_normal_y = unpack_normal(textureSample(map_texture, map_sampler, fract(uv_y)));
    var tan_normal_z = unpack_normal(textureSample(map_texture, map_sampler, fract(uv_z)));

    tan_normal_x = blend_rnm(vec3(normal.zy, abs_normal.x), tan_normal_x);
    tan_normal_y = blend_rnm(vec3(normal.xz, abs_normal.y), tan_normal_y);
    tan_normal_z = blend_rnm(vec3(normal.xy, abs_normal.z), tan_normal_z);

    let axis_sign = sign(normal);
    tan_normal_x.z *= axis_sign.x;
    tan_normal_y.z *= axis_sign.y;
    tan_normal_z.z *= axis_sign.z;

    return normalize(
        tan_normal_x.zyx * blend_weight.x +
        tan_normal_y.xzy * blend_weight.y +
        tan_normal_z.xyz * blend_weight.z
    );
}


@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
    let view_z = dot(vec4(
        view_bindings::view.inverse_view[0].z,
        view_bindings::view.inverse_view[1].z,
        view_bindings::view.inverse_view[2].z,
        view_bindings::view.inverse_view[3].z,
    ), in.world_position);

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

    var surface_normal = in.world_normal.xyz;
    let surface_bumps = triplanar_normal(in.world_position.xyz, surface_normal, planet.normal_scale, vec2(0.0), surface_normals_texture, surface_normals_sampler);
    surface_normal = normalize(mix(surface_normal, surface_bumps, planet.normal_strength));

    for (var i = 0u; i < view_bindings::lights.n_directional_lights; i++) {
        let directional_light = view_bindings::lights.directional_lights[i];
        let to_light = directional_light.direction_to_light;
        let shadow = shadows::fetch_directional_shadow(i, in.world_position, surface_normal, view_z);
        let diffuse = saturate(dot(surface_normal, to_light));
        planet_col *= diffuse * shadow;
    }

    return vec4(planet_col, 1.0);
}
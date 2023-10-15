#import bevy_pbr::utils
#import bevy_pbr::mesh_vertex_output MeshVertexOutput
#import bevy_pbr::mesh_view_bindings as view_bindings
#import bevy_pbr::prepass_utils as prepass_utils
#import bevy_pbr::pbr_functions as pbr_functions
#import bevy_pbr::mesh_view_types

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct OceanMaterial {
    radius: f32,
    depth_mul: f32,
    alpha_mul: f32,
    smoothness: f32,
    color_1: vec4<f32>,
    color_2: vec4<f32>,
}

@group(1) @binding(0) var<uniform> ocean: OceanMaterial;


fn lerp3(a: vec3<f32>, b: vec3<f32>, t: f32) -> vec3<f32> {
    return a + (b - a) * t;
}

fn lerp4(a: vec4<f32>, b: vec4<f32>, t: f32) -> vec4<f32> {
    return a + (b - a) * t;
}

fn linearize_depth(depth: f32) -> f32 {
    return view_bindings::view.projection[3][2] / depth;
}

fn ray_sphere_intersection(center: vec3<f32>, radius: f32, ro: vec3<f32>, rd: vec3<f32>) -> vec2<f32> {
    let offset = ro - center;
    let a = dot(rd, rd);
    let half_b = dot(offset, rd);
    let c = dot(offset, offset) - radius * radius;

    let discriminant = half_b * half_b - a * c;
    if (discriminant >= 0.0) {
        let s = sqrt(discriminant);
        let dst_near = max(0.0, (-half_b - s) / a);
        let dst_far = (-half_b + s) / a;

        if (dst_far >= 0.0) {
            return vec2(dst_near, dst_far - dst_near);
        }
    }
    return vec2(0.0, 0.0);
}


@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
    let view_vector = in.world_position.xyz - view_bindings::view.world_position.xyz;

    let nonlinear_depth = prepass_utils::prepass_depth(in.position, 0u);
    let scene_depth = linearize_depth(nonlinear_depth);

    let ray_pos = view_bindings::view.world_position.xyz;
    let ray_dir = normalize(view_vector);
    let hit_info = ray_sphere_intersection(vec3(0.0), ocean.radius, ray_pos, ray_dir);
    let dst_to_ocean = hit_info.x;
    let dst_thru_ocean = hit_info.y;
    let ocean_view_depth = min(dst_thru_ocean, scene_depth - linearize_depth(in.position.z));

    if (ocean_view_depth > 0.0) {
        let ocean_normal = normalize(ray_pos + ray_dir * dst_to_ocean);

        var incoming_light = 1.0;
        for (var i = 0u; i < view_bindings::lights.n_directional_lights; i++) {
            let to_light = view_bindings::lights.directional_lights[i].direction_to_light;
            let spec_angle = acos(dot(normalize(to_light - ray_dir), ocean_normal));
            let spec_exponent = spec_angle / (1.0 - ocean.smoothness);
            let spec_highlight = exp(-spec_exponent * spec_exponent);
            let diffuse = saturate(dot(ocean_normal, to_light));
            incoming_light *= diffuse;
            incoming_light += spec_highlight;
        }

        let optical_depth = 1.0 - exp(-ocean_view_depth * ocean.depth_mul);
        let alpha = 1.0 - exp(-ocean_view_depth * ocean.alpha_mul);
        let ocean_col = lerp3(ocean.color_2.xyz, ocean.color_1.xyz, optical_depth) * max(0.01, incoming_light);

        return vec4(ocean_col, alpha);
    }

    return vec4(0.0);
}
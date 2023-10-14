#import bevy_pbr::utils
#import bevy_pbr::mesh_vertex_output MeshVertexOutput
#import bevy_pbr::mesh_view_bindings view
#import bevy_pbr::prepass_utils as prepass_utils
#import bevy_pbr::pbr_functions as pbr_functions

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct OceanMaterial {
    radius: f32,
    depth_mul: f32,
    alpha_mul: f32,
#ifdef SIXTEEN_BYTE_ALIGNMENT
    // WebGL2 structs must be 16 byte aligned.
    _webgl2_padding: f32
#endif
    color_1: vec4<f32>,
    color_2: vec4<f32>,
}

@group(1) @binding(0) var<uniform> ocean: OceanMaterial;


fn lerp(a: vec3<f32>, b: vec3<f32>, t: f32) -> vec3<f32> {
    return a + (b - a) * t;
}

fn linearize_depth(depth: f32) -> f32 {
    return view.projection[3][2] / depth;
}

fn ray_sphere_intersection(center: vec3<f32>, radius: f32, ro: vec3<f32>, rd: vec3<f32>) -> vec2<f32> {
    let offset = ro - center;
    let b = 2.0 * dot(offset, rd);
    let c = dot(offset, offset) - radius * radius;

    let discriminant = b * b - 4.0 * c;
    if (discriminant > 0.0) {
        let s = sqrt(discriminant);
        let dst_near = max(0.0, (-b - s) / 2.0);
        let dst_far = (-b + s) / 2.0;

        if (dst_far >= 0.0) {
            return vec2(dst_near, dst_far - dst_near);
        }
    }
    return vec2(0.0, 0.0);
}


@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
    let view_vector = in.world_position.xyz - view.world_position.xyz;
    let nonlinear_depth = prepass_utils::prepass_depth(in.position, 0u);
    let scene_depth = linearize_depth(nonlinear_depth);// * length(view_vector);

    let ray_pos = view.world_position.xyz;
    let ray_dir = normalize(view_vector);
    let hit_info = ray_sphere_intersection(vec3(0.0), ocean.radius, ray_pos, ray_dir.xyz);
    let ocean_view_depth = min(hit_info.y, scene_depth - hit_info.x);

    if (ocean_view_depth > 0.0) {
        let optical_depth = 1.0 - exp(-ocean_view_depth * ocean.depth_mul);
        let alpha = 1.0 - exp(-ocean_view_depth * ocean.alpha_mul);
        let ocean_col = lerp(ocean.color_2.xyz, ocean.color_1.xyz, optical_depth);
        return vec4(ocean_col, alpha);
    }

    return vec4(0.0);
}
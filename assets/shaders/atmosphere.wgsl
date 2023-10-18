#import bevy_pbr::mesh_vertex_output MeshVertexOutput
#import bevy_pbr::mesh_view_bindings as view_bindings
#import bevy_pbr::prepass_utils as prepass_utils
#import bevy_pbr::mesh_view_types

const PI: f32 = 3.1415927;

struct AtmosphereMaterial {
    radius: f32,
    ocean_radius: f32,
    num_sample_points: u32,
    num_optical_depth_points: u32,
    density_falloff: f32,
    scattering_coeffs: vec3<f32>,
    #ifdef SIXTEEN_BYTE_ALIGNMENT
    _webgl2_padding: vec3<f32>,
    #endif
}

@group(1) @binding(0) var<uniform> atmosphere: AtmosphereMaterial;

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    return a + (b - a) * t;
}

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
    return vec2(10000000000000.0, 0.0);
}

fn point_density(pos: vec3<f32>) -> f32 {
    let height = (length(pos) - atmosphere.ocean_radius) / (atmosphere.radius - atmosphere.ocean_radius);
    let local_density = exp(-height * atmosphere.density_falloff) * (1.0 - height);
    return local_density;
}

fn optical_depth(ray_pos: vec3<f32>, ray_dir: vec3<f32>, ray_length: f32) -> f32 {
    var sample_point = ray_pos;
    var depth = 0.0;
    let step_size = ray_length / (f32(atmosphere.num_optical_depth_points) - 1.0);

    for (var i = 0u; i < atmosphere.num_optical_depth_points; i++) {
        let local_density = point_density(sample_point);
        depth += local_density * step_size;
        sample_point += ray_dir * step_size;
    }

    return depth;
}

fn get_light(ray_pos: vec3<f32>, ray_dir: vec3<f32>, ray_length: f32) -> vec4<f32> {
    var in_scatter_pos = ray_pos;
    var in_scattered_light = vec3(0.0);
    var scatter_light_alpha = 0.0;
    let step_size = ray_length / (f32(atmosphere.num_sample_points) - 1.0);
    let dir_to_sun = view_bindings::lights.directional_lights[0].direction_to_light;

    for (var i = 0u; i < atmosphere.num_sample_points; i++) {
        let sun_ray_length = ray_sphere_intersection(vec3(0.0), atmosphere.radius, in_scatter_pos, dir_to_sun).y;
        let sun_ray_depth = optical_depth(in_scatter_pos, dir_to_sun, sun_ray_length);
        let view_ray_depth = optical_depth(in_scatter_pos, -ray_dir, step_size * f32(i));
        let transmittance = exp(-(sun_ray_depth + view_ray_depth));
        let transmittance_scattered = pow(vec3(transmittance), atmosphere.scattering_coeffs);
        let local_density = point_density(in_scatter_pos);

        scatter_light_alpha += local_density * transmittance * step_size;
        in_scattered_light += local_density * transmittance_scattered * atmosphere.scattering_coeffs * step_size;
        in_scatter_pos += ray_dir * step_size;
    }

    return vec4(in_scattered_light, scatter_light_alpha);
}


@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
    let view_vector = in.world_position.xyz - view_bindings::view.world_position.xyz;

    let nonlinear_depth = prepass_utils::prepass_depth(in.position, 0u);
    let scene_depth = linearize_depth(nonlinear_depth);

    let ray_pos = view_bindings::view.world_position.xyz;
    let ray_dir = normalize(view_vector);

    let dst_to_ocean = ray_sphere_intersection(vec3(0.0), atmosphere.ocean_radius, ray_pos, ray_dir).x;
    let dst_to_surface = min(scene_depth, dst_to_ocean);

    let atmosphere_hit_info = ray_sphere_intersection(vec3(0.0), atmosphere.radius, ray_pos, ray_dir);
    let dst_to_atmosphere = atmosphere_hit_info.x;
    let dst_thru_atmosphere = min(atmosphere_hit_info.y, dst_to_surface - dst_to_atmosphere);

    if (dst_thru_atmosphere > 0.0) {
        let hit_pos = ray_pos + ray_dir * dst_to_atmosphere;
        let light = get_light(hit_pos, ray_dir, dst_thru_atmosphere);
        return light;
    }

    return vec4(vec3(dst_thru_atmosphere / (atmosphere.radius * 2.0)), 1.0);
}
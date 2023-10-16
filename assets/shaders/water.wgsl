#import bevy_pbr::utils
#import bevy_pbr::mesh_vertex_output MeshVertexOutput
#import bevy_pbr::mesh_view_bindings as view_bindings
#import bevy_pbr::prepass_utils as prepass_utils
#import bevy_pbr::pbr_functions as pbr_functions
#import bevy_pbr::mesh_view_types

const PI: f32 = 3.1415927;

struct OceanMaterial {
    radius: f32,
    depth_mul: f32,
    alpha_mul: f32,
    smoothness: f32,
    color_1: vec4<f32>,
    color_2: vec4<f32>,
    time: f32,
    wave_strength: f32,
    wave_speed: f32,
    wave_scale: f32,
}

@group(1) @binding(0) var<uniform> ocean: OceanMaterial;
@group(1) @binding(1) var wave_normals_texture_1: texture_2d<f32>;
@group(1) @binding(2) var wave_normals_sampler_1: sampler;
@group(1) @binding(3) var wave_normals_texture_2: texture_2d<f32>;
@group(1) @binding(4) var wave_normals_sampler_2: sampler;

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
    return vec2(0.0, 0.0);
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

fn fresnel(normal: vec3<f32>, incident: vec3<f32>) -> f32 {
    let n1 = 1.0; // air index of refraction
    let n2 = 1.33; // water index of refraction

    let r0 = ((n1 - n2) / (n1 + n2)) * ((n1 - n2) / (n1 + n2));
    let cos_x = -dot(normal, incident);
    let x = 1.0 - cos_x;
    return lerp(r0, 1.0, x * x * x * x * x);
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
        let ocean_hit_pos = ray_pos + ray_dir * dst_to_ocean;
        let ocean_sphere_normal = normalize(ocean_hit_pos);

        let wave_offset_1 = vec2(ocean.time * ocean.wave_speed, ocean.time * ocean.wave_speed * 0.8);
        let wave_offset_2 = vec2(ocean.time * ocean.wave_speed * -0.8, ocean.time * ocean.wave_speed * -0.3);
        var wave_normal = triplanar_normal(ocean_hit_pos, ocean_sphere_normal, ocean.wave_scale, wave_offset_1, wave_normals_texture_1, wave_normals_sampler_1);
        wave_normal = triplanar_normal(ocean_hit_pos, wave_normal, ocean.wave_scale, wave_offset_2, wave_normals_texture_2, wave_normals_sampler_2);
        let ocean_normal = normalize(lerp3(ocean_sphere_normal, wave_normal, ocean.wave_strength));

        let optical_depth = 1.0 - exp(-ocean_view_depth * ocean.depth_mul);
        let alpha = 1.0 - exp(-ocean_view_depth * ocean.alpha_mul);
        var ocean_col = lerp3(ocean.color_2.xyz, ocean.color_1.xyz, optical_depth);

        for (var i = 0u; i < view_bindings::lights.n_directional_lights; i++) {
            let directional_light = view_bindings::lights.directional_lights[i];
            let to_light = directional_light.direction_to_light;
            let to_eye = -ray_dir;
            let diffuse = saturate(dot(ocean_sphere_normal, to_light));

            let half_angle = normalize(to_light + to_eye);
            let spec_angle = acos(dot(half_angle, ocean_normal));

            // gaussian distribution
            // let spec_exponent = spec_angle / (1.0 - ocean.smoothness);
            // let specular = exp(-spec_exponent * spec_exponent);

            // beckmann distribution
            let tan_spec_angle = tan(spec_angle);
            let cos_spec_angle = cos(spec_angle);
            let tan2_spec_angle = tan_spec_angle * tan_spec_angle;
            let cos4_spec_angle = cos_spec_angle * cos_spec_angle * cos_spec_angle * cos_spec_angle;
            let roughness = (1.0 - ocean.smoothness) * (1.0 - ocean.smoothness);
            let spec_highlight = exp(-tan2_spec_angle / roughness) / (PI * roughness * cos4_spec_angle);

            // Geometric distribution (describes selfshadowing from microfacets)
            let n_dot_h = dot(half_angle, ocean_normal);
            let v_dot_n = dot(to_eye, ocean_normal);
            let v_dot_h = dot(to_eye, half_angle);
            let l_dot_n = dot(to_light, ocean_normal);

            let view_attenuation = (2.0 * n_dot_h * v_dot_n) / v_dot_h;
            let light_attenuation = (2.0 * n_dot_h * l_dot_n) / v_dot_h;
            let geometric_attenuation = min(1.0, min(view_attenuation, light_attenuation));

            let fresnel = saturate(fresnel(ocean_normal, to_eye));
            let reflected_dir = reflect(to_eye, ocean_normal);
            let reflected_col = dot(reflected_dir, to_light);

            // cook-torrance model
            let specular = (spec_highlight * fresnel * geometric_attenuation) / (4.0 * v_dot_n * l_dot_n);

            ocean_col *= vec3(diffuse);
            ocean_col += specular * directional_light.color.xyz;
        }

        return vec4(ocean_col, alpha);
    }

    return vec4(0.0);
}
@group(0) @binding(0)
var height_map: texture_storage_2d<rgba32float, read_write>;
@group(0) @binding(1)
var<uniform> settings: SettingsUniform;

struct SettingsUniform {
    texture_size: vec2<i32>,
}

var<private> directions: array<vec3<f32>, 6> = array<vec3<f32>, 6>(
    vec3<f32>(0.0, 1.0, 0.0), vec3<f32>(0.0, -1.0, 0.0), 
    vec3<f32>(1.0, 0.0, 0.0), vec3<f32>(-1.0, 0.0, 0.0),
    vec3<f32>(0.0, 0.0, 1.0), vec3<f32>(0.0, 0.0, -1.0),
);


@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) id: vec3<u32>) {
    let coord = vec2(i32(id.x), i32(id.y));
    let uv = vec2<f32>(id.xy) / vec2<f32>(settings.texture_size.xy);
    let layer = id.z;

    let face_local_up = directions[layer];
    let face_axis_a = vec3(face_local_up.y, face_local_up.z, face_local_up.x);
    let face_axis_b = cross(face_local_up, face_axis_a);

    let point_on_cube = face_local_up + (uv.x - 0.5) * 2.0 * face_axis_a + (uv.y - 0.5) * 2.0 * face_axis_b;
    let pos = normalize(point_on_cube);

    var height = sin(pos.x) + sin(pos.y) + sin(pos.z);
    height = height * 0.5 + 0.5;

    storageBarrier();
    textureStore(height_map, coord, vec4(1.0));
}

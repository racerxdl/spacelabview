@group(0)
@binding(0)
var height_map: texture_2d<f32>;

@group(0)
@binding(1)
var texture: texture_storage_2d<rgba8unorm, write>;


@compute
@workgroup_size(8,8)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let width = u32(textureDimensions(height_map).x);
    let height = u32(textureDimensions(height_map).y);

    let numSamples = 4;

    let x0 = i32(global_id.x);
    let y0 = i32(global_id.y);

    let x1 = i32(u32(x0 + 1) % width);
    let y1 = i32(u32(y0 + 1) % height);

    let z0 = textureLoad(height_map, vec2<i32>(x0, y0), 0).r;
    let z1 = textureLoad(height_map, vec2<i32>(x1, y1), 0).r;

    let delta_z = z1 - z0;
    let delta_x = (f32(x1) - f32(x0))/f32(width);
    let delta_y = (f32(y1) - f32(y0))/f32(height);

    let slope = atan(delta_z / sqrt(delta_x * delta_x + delta_y * delta_y)) * 57.2958;
    let a = abs(slope);

    let normalized_a = a / 256.0;

    textureStore(texture, vec2<i32>(x0, y0), vec4<f32>(normalized_a, normalized_a, normalized_a, 1.0));
}
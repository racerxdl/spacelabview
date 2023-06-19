
@group(0) @binding(0) var height_map: texture_2d<f32>;
@group(0) @binding(1) var texture: texture_storage_2d<rgba8unorm, write>;

@compute
@workgroup_size(8,8)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let coord: vec2<i32> = vec2<i32>(global_id.xy);

    // Sampling around the current texel for Sobel filter
    let tl: f32 = abs(textureLoad(height_map, coord + vec2<i32>(-1, -1), 0).r);
    let  l: f32 = abs(textureLoad(height_map, coord + vec2<i32>(-1,  0), 0).r);
    let bl: f32 = abs(textureLoad(height_map, coord + vec2<i32>(-1,  1), 0).r);
    let  t: f32 = abs(textureLoad(height_map, coord + vec2<i32>( 0, -1), 0).r);
    let  b: f32 = abs(textureLoad(height_map, coord + vec2<i32>( 0,  1), 0).r);
    let tr: f32 = abs(textureLoad(height_map, coord + vec2<i32>( 1, -1), 0).r);
    let  r: f32 = abs(textureLoad(height_map, coord + vec2<i32>( 1,  0), 0).r);
    let br: f32 = abs(textureLoad(height_map, coord + vec2<i32>( 1,  1), 0).r);

    // Compute dx using Sobel
    let dX: f32 = tr + 2.0 * r + br - tl - 2.0 * l - bl;

    // Compute dy using Sobel
    let dY: f32 = bl + 2.0 * b + br - tl - 2.0 * t - tr;

    // Build the normalized normal
    let N: vec3<f32> = normalize(vec3<f32>(dX, dY, 1.0));

    // Convert (-1.0 , 1.0) to (0.0 , 1.0), if needed
    let N_mapped: vec4<f32> = vec4<f32>(N * 0.5 + 0.5, 1.0);

    // Write to the output texture
    textureStore(texture, coord, N_mapped);
}





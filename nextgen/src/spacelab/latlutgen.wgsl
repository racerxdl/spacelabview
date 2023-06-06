struct LatLutGenParams {
    face_num: u32,
    width: u32,
    height: u32,
};

@group(0)
@binding(0)
var<uniform> params : LatLutGenParams;

@group(0)
@binding(1)
var texture: texture_storage_2d<rgba8unorm, write>;

fn compute_point(u: f32, v: f32, face: i32) -> vec3<f32> {
    var point: vec3<f32>;

    if face == 0 { // "front"
        point = vec3<f32>(u, v, -1.0);
    } else if face == 1 { // "back"
        point = vec3<f32>(-u, v, 1.0);
    } else if face == 2 { // "down"
        point = vec3<f32>(u, -1.0, v);
    } else if face == 3 { // "up"
        point = vec3<f32>(u, 1.0, -v);
    } else if face == 4 {// "left"
        point = vec3<f32>(-1.0, v, -u);
    } else if face == 5 {// "right"
        point = vec3<f32>(1.0, v, u);
    } else {
        point = vec3<f32>(0.0, 0.0, 0.0);
    }
    return point;
}

const rad2deg: f32 = 57.29577951308232;  // approximately equal to 180/pi

@compute
@workgroup_size(8,8)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let X: f32 = f32(global_id.x);
    let Y: f32 = f32(global_id.y);
    let face_num: i32 = i32(params.face_num);
    let width: f32 = f32(params.width);
    let height: f32 = f32(params.height);

    let u = (X + 0.5) / width * 2.0 - 1.0;
    let v = (Y + 0.5) / height * 2.0 - 1.0;
    let point = compute_point(u, v, face_num);

    let point_on_sphere = normalize(point);
    let latitude = asin(point_on_sphere.y);
    let latitude_degrees = (latitude * rad2deg) / 256.0; // Normalized byte at texture 0..1
    let color = vec4<f32>(latitude_degrees, latitude_degrees, latitude_degrees, 1.0);
    textureStore(texture, vec2<i32>(i32(X), i32(Y)), color);
}
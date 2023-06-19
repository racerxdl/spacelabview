
struct GPUMaterialRule {
    id: u32,
    color: vec4<f32>,
    height: vec2<f32>,
    latitude: vec2<f32>,
    slope: vec2<f32>,
}

struct RuleArray {
    data: array<GPUMaterialRule>,
}

@group(0) @binding(0) var<storage> default_materials: array<GPUMaterialRule>;
@group(0) @binding(1) var<storage> simple_materials: array<GPUMaterialRule>;
@group(0) @binding(2) var<storage> complex_materials: array<GPUMaterialRule>;

@group(0) @binding(3) var material_map: texture_2d<f32>;
@group(0) @binding(4) var height_map: texture_2d<f32>;
@group(0) @binding(5) var latlut: texture_2d<f32>;
@group(0) @binding(6) var normal_map: texture_2d<f32>;
@group(0) @binding(7) var texture: texture_storage_2d<rgba8unorm, write>;

fn material_match(rule: GPUMaterialRule, height: f32, latitude: f32, slope: f32) -> bool {
    if (height < rule.height.x || height > rule.height.y) {
        return false;
    }

    if (latitude < rule.latitude.x || latitude > rule.latitude.y) {
        return false;
    }

    if (slope < rule.slope.x || slope > rule.slope.y) {
        return false;
    }

    return true;
}

fn slope(x: i32, y: i32) -> f32 {
    let normal = textureLoad(normal_map, vec2<i32>(x, y), 0).rgb;
    // Since the normal is normalized, the z component is the cosine of the angle
    let a = acos(normal.z) * 180.0 / 3.1415926535897932384626433832795;
    return a;
}

@compute
@workgroup_size(8,8)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let X: i32 = i32(global_id.x);
    let Y: i32 = i32(global_id.y);
    let id = u32(textureLoad(material_map, vec2<i32>(X, Y), 0).r * 255.0);
    let slope = slope(X,Y); //textureLoad(slope_map, vec2<i32>(X, Y), 0).r * 90.0;
    let lat = textureLoad(latlut, vec2<i32>(X, Y), 0).r * 90.0;
    let height = textureLoad(height_map, vec2<i32>(X, Y), 0).r;

    for (var i = 0u; i < arrayLength(&complex_materials); i = i + 1u) {
        if (complex_materials[i].id == id && material_match(complex_materials[i], height, lat, slope)) {
            var color = complex_materials[i].color;
            textureStore(texture, vec2<i32>(X, Y), color);
            return;
        }
    }

    for (var i = 0u; i < arrayLength(&simple_materials); i = i + 1u) {
        if (simple_materials[i].id == id) {
            var color = simple_materials[i].color;
            textureStore(texture, vec2<i32>(X, Y), color);
            return;
        }
    }
    if (arrayLength(&default_materials) > 0u) {
            var color = default_materials[0].color;
            textureStore(texture, vec2<i32>(X, Y), color);
            return;
    }
    // TODO: Since it can be interpolated, check for closest match material?

    // Not found, shouldnt happen since all materials has default color.
    textureStore(texture, vec2<i32>(X, Y), vec4<f32>(0.0, 0.0, 0.0, 1.0)); // Not found
}
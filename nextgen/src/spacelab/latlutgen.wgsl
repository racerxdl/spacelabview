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
    switch (face) {
        case 0:     { return vec3<f32>(u, v, -1.0);    } // "front"
        case 1:     { return vec3<f32>(-u, v, 1.0);    } // "back"
        case 2:     { return vec3<f32>(u, -1.0, v);    } // "down"
        case 3:     { return vec3<f32>(u, 1.0, -v);    } // "up"
        case 4:     { return vec3<f32>(-1.0, v, -u);   } // "left"
        case 5:     { return vec3<f32>(1.0, v, u);     } // "right"
        default:    { return vec3<f32>(0.0, 0.0, 0.0); } // "none"
    }
}

const rad: f32 = 1.5707963267948966;

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
    let latitude_radian_norm = abs(latitude) / rad;
    let color = vec4<f32>(latitude_radian_norm, latitude_radian_norm, latitude_radian_norm, 1.0);
    textureStore(texture, vec2<i32>(i32(X), i32(Y)), color);
}
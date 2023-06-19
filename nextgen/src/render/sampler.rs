use image::{DynamicImage, Luma, ImageBuffer};


pub fn sample_displacement(img: &DynamicImage, x: f32, y: f32) -> f32 {
    return match img {
        DynamicImage::ImageLuma16(img) => {
            sample_displacement_g16(img, x, y)
        },
        DynamicImage::ImageLuma8(img) => {
            sample_displacement_g8(img, x, y)
        },
        DynamicImage::ImageRgba8(img) => {
            sample_displacement_rgba8(img, x, y)
        },
        DynamicImage::ImageRgb8(img) => {
            sample_displacement_rgb8(img, x, y)
        },
        _ => 0.0
    }
}

fn sample_displacement_rgb8(img: &ImageBuffer<image::Rgb<u8>, Vec<u8>>, x: f32, y: f32) -> f32 {
    if x < 0.0 || x > 1.0 || y < 0.0 || y > 1.0 {
        return 0.0;
    }
    let mut x = (x) * (img.width() as f32);
    let mut y = (y) * (img.height() as f32);

    if x == (img.width()-1) as f32 || y == (img.height()-1) as f32 {
        let px = img.get_pixel(x as u32, y as u32).0;
        return px[0] as f32 / 255.0;
    }

    if x == (img.width()) as f32 || y == (img.height()) as f32 {
        if x == img.width() as f32 {
            x = (img.width() - 1) as f32;
        }
        if y == img.height() as f32 {
            y = (img.height() - 1) as f32;
        }
        let px = img.get_pixel(x as u32, y as u32).0;
        return px[0] as f32 / 255.0;
    }

    // Bilinear interpolation
    let x0 = x.floor() as u32;
    let x1 = x0 + 1;
    let y0 = y.floor() as u32;
    let y1 = y0 + 1;
    let px00 = img.get_pixel(x0, y0).0;
    let px01 = img.get_pixel(x0, y1).0;
    let px10 = img.get_pixel(x1, y0).0;
    let px11 = img.get_pixel(x1, y1).0;
    let x0f = x - x0 as f32;
    let x1f = 1.0 - x0f;
    let y0f = y - y0 as f32;
    let y1f = 1.0 - y0f;
    let px0 = px00[0] as f32 * x1f + px10[0] as f32 * x0f;
    let px1 = px01[0] as f32 * x1f + px11[0] as f32 * x0f;
    let px = px0 * y1f + px1 * y0f;
    return px / 255.0;
}

fn sample_displacement_rgba8(img: &ImageBuffer<image::Rgba<u8>, Vec<u8>>, x: f32, y: f32) -> f32 {
    if x < 0.0 || x > 1.0 || y < 0.0 || y > 1.0 {
        return 0.0;
    }
    let mut x = (x) * (img.width() as f32);
    let mut y = (y) * (img.height() as f32);

    if x == (img.width()-1) as f32 || y == (img.height()-1) as f32 {
        let px = img.get_pixel(x as u32, y as u32).0;
        return px[0] as f32 / 255.0;
    }

    if x == (img.width()) as f32 || y == (img.height()) as f32 {
        if x == img.width() as f32 {
            x = (img.width() - 1) as f32;
        }
        if y == img.height() as f32 {
            y = (img.height() - 1) as f32;
        }
        let px = img.get_pixel(x as u32, y as u32).0;
        return px[0] as f32 / 255.0;
    }

    // Bilinear interpolation
    let x0 = x.floor() as u32;
    let x1 = x0 + 1;
    let y0 = y.floor() as u32;
    let y1 = y0 + 1;
    let px00 = img.get_pixel(x0, y0).0;
    let px01 = img.get_pixel(x0, y1).0;
    let px10 = img.get_pixel(x1, y0).0;
    let px11 = img.get_pixel(x1, y1).0;
    let x0f = x - x0 as f32;
    let x1f = 1.0 - x0f;
    let y0f = y - y0 as f32;
    let y1f = 1.0 - y0f;
    let px0 = px00[0] as f32 * x1f + px10[0] as f32 * x0f;
    let px1 = px01[0] as f32 * x1f + px11[0] as f32 * x0f;
    let px = px0 * y1f + px1 * y0f;
    return px / 255.0;
}

fn sample_displacement_g16(img: &ImageBuffer<Luma<u16>, Vec<u16>>, x: f32, y: f32) -> f32 {
    if x < 0.0 || x > 1.0 || y < 0.0 || y > 1.0 {
        return 0.0;
    }
    let x = (x) * (img.width() as f32 / 2.0);
    let y = (y) * (img.height() as f32 / 2.0);

    if x == (img.width()-1) as f32 || y == (img.height()-1) as f32 {
        let px = img.get_pixel(x as u32, y as u32).0;
        return px[0] as f32 / 65535.0;
    }
    // Bilinear interpolation
    let x0 = x.floor() as u32;
    let x1 = x0 + 1;
    let y0 = y.floor() as u32;
    let y1 = y0 + 1;
    let px00 = img.get_pixel(x0, y0).0;
    let px01 = img.get_pixel(x0, y1).0;
    let px10 = img.get_pixel(x1, y0).0;
    let px11 = img.get_pixel(x1, y1).0;
    let x0f = x - x0 as f32;
    let x1f = 1.0 - x0f;
    let y0f = y - y0 as f32;
    let y1f = 1.0 - y0f;
    let px0 = px00[0] as f32 * x1f + px10[0] as f32 * x0f;
    let px1 = px01[0] as f32 * x1f + px11[0] as f32 * x0f;
    let px = px0 * y1f + px1 * y0f;
    return px / 65535.0;
}


fn sample_displacement_g8(img: &ImageBuffer<Luma<u8>, Vec<u8>>, x: f32, y: f32) -> f32 {
    if x < 0.0 || x > 1.0 || y < 0.0 || y > 1.0 {
        return 0.0;
    }
    let mut x = (x) * (img.width() as f32);
    let mut y = (y) * (img.height() as f32);

    if x == (img.width()-1) as f32 || y == (img.height()-1) as f32 {
        let px = img.get_pixel(x as u32, y as u32).0;
        return px[0] as f32 / 255.0;
    }

    if x == (img.width()) as f32 || y == (img.height()) as f32 {
        if x == img.width() as f32 {
            x = (img.width() - 1) as f32;
        }
        if y == img.height() as f32 {
            y = (img.height() - 1) as f32;
        }
        let px = img.get_pixel(x as u32, y as u32).0;
        return px[0] as f32 / 255.0;
    }

    // Bilinear interpolation
    let x0 = x.floor() as u32;
    let x1 = x0 + 1;
    let y0 = y.floor() as u32;
    let y1 = y0 + 1;
    let px00 = img.get_pixel(x0, y0).0;
    let px01 = img.get_pixel(x0, y1).0;
    let px10 = img.get_pixel(x1, y0).0;
    let px11 = img.get_pixel(x1, y1).0;
    let x0f = x - x0 as f32;
    let x1f = 1.0 - x0f;
    let y0f = y - y0 as f32;
    let y1f = 1.0 - y0f;
    let px0 = px00[0] as f32 * x1f + px10[0] as f32 * x0f;
    let px1 = px01[0] as f32 * x1f + px11[0] as f32 * x0f;
    let px = px0 * y1f + px1 * y0f;
    return px / 255.0;
}
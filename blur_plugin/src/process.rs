//! Методы обработки изображения.

use crate::params::Params;

/// Срез RGBA.
#[repr(C)]
#[derive(Clone, Copy)]
pub(super) struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[inline]
fn idx(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}

/// Обработать задание по размытию изображения.
pub(super) fn blur_executor(rgba_data: &mut [Rgba], width: usize, height: usize, params: &Params) {
    let radius = params.radius;
    let iterations = params.iterations;

    if radius <= 0 || iterations == 0 {
        return;
    }

    let mut src = rgba_data.to_vec();
    let mut dst = src.clone();

    for _ in 0..iterations {
        for y in 0..height {
            for x in 0..width {
                let mut sum_w = 0.0_f32;
                let mut acc_r = 0.0_f32;
                let mut acc_g = 0.0_f32;
                let mut acc_b = 0.0_f32;
                let mut acc_a = 0.0_f32;

                let y0 = (y as isize - radius).max(0) as usize;
                let y1 = (y as isize + radius).min(height as isize - 1) as usize;
                let x0 = (x as isize - radius).max(0) as usize;
                let x1 = (x as isize + radius).min(width as isize - 1) as usize;

                for ny in y0..=y1 {
                    for nx in x0..=x1 {
                        let dx = nx as isize - x as isize;
                        let dy = ny as isize - y as isize;
                        let dist = ((dx * dx + dy * dy) as f32).sqrt();

                        if dist > radius as f32 {
                            continue;
                        }

                        let w = 1.0 / (1.0 + dist);
                        let p = src[idx(nx, ny, width)];

                        acc_r += p.r as f32 * w;
                        acc_g += p.g as f32 * w;
                        acc_b += p.b as f32 * w;
                        acc_a += p.a as f32 * w;
                        sum_w += w;
                    }
                }

                let out = &mut dst[idx(x, y, width)];
                out.r = (acc_r / sum_w).round().clamp(0.0, 255.0) as u8;
                out.g = (acc_g / sum_w).round().clamp(0.0, 255.0) as u8;
                out.b = (acc_b / sum_w).round().clamp(0.0, 255.0) as u8;
                out.a = (acc_a / sum_w).round().clamp(0.0, 255.0) as u8;
            }
        }

        core::mem::swap(&mut src, &mut dst);
    }

    rgba_data.copy_from_slice(&src);
}

use crate::{framebuffer::Framebuffer, types::Pos3};
use glam::{Vec2, Vec3Swizzles};

pub fn fill_triangle(
    framebuffer: &mut Framebuffer<char>,
    p0: Pos3, // Pos3 includes .z
    p1: Pos3,
    p2: Pos3,
    shade: char,
) {
    let area = edge_function(p0.xy(), p1.xy(), p2.xy());

    if area <= 0.0 {
        return;
    }

    let inv_area = 1.0 / area;

    let width = framebuffer.width() as i32;
    let height = framebuffer.height() as i32;

    let min_x = (p0.x.min(p1.x).min(p2.x).floor() as i32).max(0);
    let max_x = (p0.x.max(p1.x).max(p2.x).ceil() as i32).min(width - 1);
    let min_y = (p0.y.min(p1.y).min(p2.y).floor() as i32).max(0);
    let max_y = (p0.y.max(p1.y).max(p2.y).ceil() as i32).min(height - 1);

    let start = Vec2::new(min_x as f32 + 0.5, min_y as f32 + 0.5);

    let mut w0_row = edge_function(p1.xy(), p2.xy(), start);
    let mut w1_row = edge_function(p2.xy(), p0.xy(), start);
    let mut w2_row = edge_function(p0.xy(), p1.xy(), start);

    let w0_dx = p2.y - p1.y;
    let w0_dy = p1.x - p2.x;
    let w1_dx = p0.y - p2.y;
    let w1_dy = p2.x - p0.x;
    let w2_dx = p1.y - p0.y;
    let w2_dy = p0.x - p1.x;

    for y in min_y..=max_y {
        let mut w0 = w0_row;
        let mut w1 = w1_row;
        let mut w2 = w2_row;

        for x in min_x..=max_x {
            if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
                let z = (w0 * p0.z + w1 * p1.z + w2 * p2.z) * inv_area;
                framebuffer.set_pixel_with_depth(&Vec2::new(x as f32, y as f32), shade, z);
            }

            w0 += w0_dx;
            w1 += w1_dx;
            w2 += w2_dx;
        }

        w0_row += w0_dy;
        w1_row += w1_dy;
        w2_row += w2_dy;
    }
}

fn edge_function(a: Vec2, b: Vec2, c: Vec2) -> f32 {
    let ac = c - a;
    let ba = b - a;
    ac.perp_dot(ba)
}

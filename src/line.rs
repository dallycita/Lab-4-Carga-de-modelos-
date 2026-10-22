use raylib::prelude::*;

use crate::framebuffer::Framebuffer;

/// Dibuja una línea entre dos puntos con el algoritmo de Bresenham.
pub fn line(fb: &mut Framebuffer, start: Vector2, end: Vector2) {
    let mut x0 = start.x.round() as i32;
    let mut y0 = start.y.round() as i32;
    let x1 = end.x.round() as i32;
    let y1 = end.y.round() as i32;

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        fb.point(x0, y0);
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

/// Dibuja el contorno de un triángulo usando tres líneas.
pub fn triangle(fb: &mut Framebuffer, a: Vector2, b: Vector2, c: Vector2) {
    line(fb, a, b);
    line(fb, b, c);
    line(fb, c, a);
}

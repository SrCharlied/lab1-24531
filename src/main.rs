mod framebuffer;
mod bmp;
mod line;

use crate::bmp::WriteBmp;
use crate::framebuffer::Framebuffer;
use crate::line::Line;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    framebuffer.set_current_color(0xFF0000);
    framebuffer.line(100, 100, 600, 500);
    framebuffer.line(100, 100, 600, 400);
    framebuffer.line(100, 100, 600, 300);
    framebuffer.line(100, 100, 600, 200);
    framebuffer.point(0,0);

    let _ = framebuffer.render_buffer("output.png");

    println!("Framebuffer rendered to output.png");
}
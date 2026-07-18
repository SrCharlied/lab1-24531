use crate::framebuffer::Framebuffer;

pub trait Scan {
    fn scan(&mut self, vertices: &[(i32, i32)]);
}

impl Scan for Framebuffer {
    fn scan(&mut self, vertices: &[(i32, i32)]) {

        for y in y_min..=y_max {
            let mut cruces: Vec<i32> = Vec::new();
            for i in 0..vertices.len(){
                let a = vertices[i];
                let b = vertices[(i + 1) % vertices.len()];
                if (a.1 <= y && b.1 > y) || (a.1 > y && b.1 <= y) {
                    cruces.push((b.0 - a.0) * (y - a.1) / (b.1 - a.1) + a.0);
                }
            cruces.sort();
            }
            for i in (0..cruces.len()).step_by(2) {
                let x_start = cruces[i];
                let x_end = cruces[i + 1];
                for x in x_start..=x_end {
                    self.point(x as usize, y as usize);
                }
            }
        }
    
    }
}
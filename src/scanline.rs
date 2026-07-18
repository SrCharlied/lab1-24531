use crate::framebuffer::Framebuffer;

pub trait Scan {
    fn scan(&mut self, polygons: &[&[(i32, i32)]]);
}

impl Scan for Framebuffer {
    fn scan(&mut self, polygons: &[&[(i32, i32)]]) {

        let mut y_min = polygons[0][0].1;
        let mut y_max = polygons[0][0].1;

        for poly in polygons {
            for v in *poly {
                if v.1 < y_min {
                    y_min = v.1;
                }
                if v.1 > y_max {
                    y_max = v.1;
                }
            }
        }

        for y in y_min..=y_max {
            let mut cruces: Vec<i32> = Vec::new();
            for poly in polygons {
                for i in 0..poly.len(){
                    let a = poly[i];
                    let b = poly[(i + 1) % poly.len()];
                    if (a.1 <= y && b.1 > y) || (a.1 > y && b.1 <= y) {
                        cruces.push((b.0 - a.0) * (y - a.1) / (b.1 - a.1) + a.0);
                    }
                }
            }
            cruces.sort();
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
use std::collections::HashMap;

#[derive(Clone)]
pub struct Latex2D<T> {
    resolution: f32,
    w: f32,
    h: f32,
    cells: HashMap<(i32, i32), Vec<T>>,
}

impl<T> Latex2D<T>
where T: Clone {
    pub fn new(resolution: f32, w: f32, h: f32) -> Latex2D<T> {
        Latex2D {
            resolution,
            w, h,
            cells: HashMap::new(),
        }
    }

    pub fn add(&mut self, pos: (f32, f32), element: T) {
        let pos = (
            (pos.0 / self.resolution) as i32,
            (pos.1 / self.resolution) as i32,
        );
        if !self.cells.contains_key(&pos) {
            self.cells.insert(pos, vec![element]);
        } else {
            self.cells.get_mut(&pos).unwrap().push(element);
        }
    }

    pub fn get(&self, pos: (f32, f32), radius: f32) -> Vec<T> {
        let w = (self.w / self.resolution) as i32;
        let h = (self.h / self.resolution) as i32;

        let d = pos.0 - radius;
        let s = if d < 0.0 { d - self.resolution as f32 } else { d };
        let mut cell_start_x = (s / self.resolution) as i32;
        let mut cell_end_x = ((pos.0 + radius) / self.resolution) as i32;
        if cell_end_x - cell_start_x > w {
            cell_start_x = 0;
            cell_end_x = w-1;
        }
        let d = pos.1 - radius;
        let s = if d < 0.0 { d - self.resolution as f32 } else { d };
        let mut cell_start_y = (s / self.resolution) as i32;
        let mut cell_end_y = ((pos.1 + radius) / self.resolution) as i32;
        if cell_end_y - cell_start_y > h {
            cell_start_y = 0;
            cell_end_y = h-1;
        }
        // println!("{} {}", cell_start_x, cell_end_x);

        let mut ret = Vec::with_capacity(1000);
        let vec = vec![];

        for x in cell_start_x..cell_end_x+1 {
            for y in cell_start_y..cell_end_y+1 {
                let x = if x < 0 { x + w } else if x >= w { x - w } else { x };
                let y = if y < 0 { y + h } else if y >= h { y - h } else { y };
                ret.extend_from_slice(
                    &self.cells.get(&(x, y)).unwrap_or(&vec)
                )
            }
        }
        ret
    }
}

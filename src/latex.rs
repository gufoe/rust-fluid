// use std::collections::HashMap;
use hashbrown::HashMap;

#[derive(Clone)]
pub struct Latex2D<T> {
    resolution: f32,
    pub w: f32,
    pub h: f32,
    cells: HashMap<(i16, i16), Vec<T>>,
}

fn f32_to_i16(n: f32) -> i16 {
    n.floor() as i16
}

impl<T> Latex2D<T>
where
    T: Clone,
{
    pub fn new(resolution: f32, w: f32, h: f32) -> Latex2D<T> {
        Latex2D {
            resolution,
            w,
            h,
            cells: HashMap::new(),
        }
    }

    pub fn add(&mut self, pos: (f32, f32), element: T) {
        let pos = self.hash(pos);
        if !self.cells.contains_key(&pos) {
            self.cells.insert(pos, vec![element]);
        } else {
            self.cells.get_mut(&pos).unwrap().push(element);
        }
    }

    fn hash_f32(&self, p: f32) -> i16 {
        f32_to_i16(p / self.resolution)
    }
    fn hash(&self, p: (f32, f32)) -> (i16, i16) {
        let mut p = (self.hash_f32(p.0), self.hash_f32(p.1));
        let w = self.hash_f32(self.w);
        let h = self.hash_f32(self.h);
        while p.0 < 0 {
            p.0 += w;
        }
        while p.1 < 0 {
            p.1 += h;
        }
        while p.0 >= w {
            p.0 -= w;
        }
        while p.1 >= h {
            p.1 -= h;
        }
        p
    }

    pub fn get(&self, pos: (f32, f32), radius: f32) -> Vec<&T> {
        return flatten(self.cells.values().collect());
        // return
        let mut ret = vec![];
        let start = self.hash((pos.0 - radius, pos.1 - radius));
        let end = self.hash((pos.0 + radius, pos.1 + radius));
        let mut y = start.1;

        // println!("start {:?} {}: {:?} {:?}", pos, radius, start, end);
        // println!(
        //     "start {} {}x{}",
        //     self.resolution,
        //     f32_to_i16(self.w / self.resolution),
        //     f32_to_i16(self.h / self.resolution)
        // );
        loop {
            let mut x = start.0;
            loop {
                // println!("test {} {}", x, y);

                if let Some(agents) = self.cells.get(&(x, y)) {
                    ret.push(agents);
                }

                if x == end.0 {
                    break;
                } else {
                    // println!("incr");
                    x += 1;
                    x %= self.hash_f32(self.w);
                }
            }

            if y == end.1 {
                break;
            } else {
                y += 1;
                y %= self.hash_f32(self.h);
            }
        }


        return flatten(ret);

        // // let mut tim = utils::Timer::new("LATEX");
        // let w = (self.w / self.resolution) as i16;
        // let h = (self.h / self.resolution) as i16;

        // let d = pos.0 - radius;
        // let sx = if d < 0.0 { d + self.w as f32 } else { d };
        // let mut cell_start_x = (sx / self.resolution) as i16;
        // let mut cell_end_x = ((pos.0 + radius) / self.resolution) as i16;
        // if cell_end_x - cell_start_x > w {
        //     cell_start_x = 0;
        //     cell_end_x = w - 1;
        // }
        // let d = pos.1 - radius;
        // let sy = if d < 0.0 { d + self.h as f32 } else { d };
        // let mut cell_start_y = (sy / self.resolution) as i16;
        // let mut cell_end_y = ((pos.1 + radius) / self.resolution) as i16;
        // if cell_end_y - cell_start_y > h {
        //     cell_start_y = 0;
        //     cell_end_y = h - 1;
        // }
        // // println!("{} {}", cell_start_x, cell_end_x);
        // println!("start {:?} {}", pos, radius);
        // println!("{} {}", sx, sy);

        // let mut ret = Vec::with_capacity(50);

        // // tim.tick("latex start");
        // for x in cell_start_x..cell_end_x + 1 {
        //     for y in cell_start_y..cell_end_y + 1 {
        //         let x = if x < 0 {
        //             x + w
        //         } else if x >= w {
        //             x - w
        //         } else {
        //             x
        //         };
        //         let y = if y < 0 {
        //             y + h
        //         } else if y >= h {
        //             y - h
        //         } else {
        //             y
        //         };
        //         match self.cells.get(&(x, y)) {
        //             None => {}
        //             Some(v) => ret.push(v),
        //         }
        //     }
        // }
        // // tim.tick("latex end");
        // // tim.show();
        // flatten(ret)
    }
}

fn flatten<T>(nested: Vec<&Vec<T>>) -> Vec<&T> {
    nested.into_iter().flatten().collect()
}

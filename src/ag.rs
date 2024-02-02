use std::cmp::Ordering;

use nannou::prelude::hsla;
use nannou::Draw;

use crate::latex::HasPosition;
use crate::utils;
use crate::vec;

const MAX_DIST: f32 = 120.0;
const WANTED_DP_LEN: f32 = 4.0;
const MAXACC: f32 = 0.005;
const DRAG: f32 = 0.02;
const AG_SIZE: f32 = 1.5;
const KEEP: usize = 3;
const DRAW_RANGE: bool = false;
const DRAW_WANTED_DP_LEN: bool = false;
// const RULES: [[f32; 3]; 3]  = [
//     [1.0, -0.5, 0.0],
//     [0.0, 1.0, -0.5],
//     [-0.5, 0.0, 1.0],
// ];
// const RULES: [[f32; 1]; 1] = [[1.0]];
const RULES: [[f32; 2]; 2] = [[1.0, -0.1], [1.0, 0.1]];
// const RULES: [[f32; 2]; 2] = [[1.0, 1.0], [-1.0, 1.0]];
// const RULES: [[f32; 1]; 1] = [[1.0]];
// let rules = [
//     [1.0,1.1,-0.2,-0.2],
//     [-0.2,1.0,1.1,-0.2],
//     [-0.2,-0.2,1.0,1.1],
//     [1.1,-0.2,-0.2,1.0],
// ];
// const RULES: [[f32; 8]; 8] = [
//     [2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.1],
//     [-0.1, 2.0, 1.2, 0.2, 0.1, 0.0, -0.05, -0.05],
//     [-0.05, -0.1, 2.0, 1.2, 0.2, 0.1, 0.0, -0.05],
//     [-0.05, -0.05, -0.1, 2.0, 1.2, 0.2, 0.1, 0.0],
//     [0.0, -0.05, -0.05, -0.1, 2.0, 1.2, 0.2, 0.1],
//     [0.1, 0.0, -0.05, -0.05, -0.1, 2.0, 1.2, 0.2],
//     [0.2, 0.1, 0.0, -0.05, -0.05, -0.1, 2.0, 1.2],
//     [1.2, 0.2, 0.1, 0.0, -0.05, -0.05, -0.1, 2.0],
// ];
// const RULES: [[f32; 8]; 8] = [
//     [0.9, 0.9, 0.0, 0.0, -0.1, -0.2, -0.2, 0.0],
//     [-0.1, 0.9, 0.9, 0.0, 0.0, 0.0, 0.0, 0.0],
//     [-0.2, -0.1, 0.9, 0.9, -0.2, 0.0, 0.0,-0.2],
//     [-0.2, -0.2, -0.1, 0.9, 1.2, 0.0, -0.2, 0.0],
//     [-0.2, -0.2, -0.2, -0.3, 2.4, 0.0, 0.0, -0.2],
//     [0.0, -0.2, -0.2, -0.1, 0.9, 1.2, 0.0, 0.0],
//     [2.4, 0.0, -0.2, -0.2, -0.2, -0.3, 2.4, 0.0],
//     [-1., -1., -1., -1., -1., -1., -1., -2.],
// ];
// const RULES: [[f32; 4]; 4] = [
//     [1.0, 0.0, 0.0, -1.0],
//     [0.0, 1.0, 0.5, 0.0],
//     [0.0, 0.0, 1.0, 0.5],
//     [0.5, 0.0, 0.0, 1.0],
// ];
// const RULES: [[f32; 4]; 4] = [
//     [2.0, 0.5, -1.0, 0.0],
//     [-1.0, 2.0, 0.5, -1.0],
//     [0.5, -1.0, 2.0, 0.5],
//     [0.0, 0.5, -1.0, 2.0],
// ];

// const rules = [
//   [1.0, 0.0, random::<f32>()*2.0-1.0, 0.0, 0.0, 0.0, ],
//   [0.0, 1.0, 0.0, 0.0, 0.0, 1.0, ],
//   [random::<f32>()*2.0-1.0, 0.0, 1.0, 0.0, random::<f32>()*2.0-1.0, 0.0, ],
//   [0.0, random::<f32>()*2.0-1.0, 0.0, 1.0, 0.0, 0.0, ],
//   [0.0, 0.0, 0.0, 0.0, 1.0, 0.5, ],
//   [-1.0, -1.0, -1.0, -1.0, -1.0, 0.2 ],
// ];

#[derive(Clone)]
pub struct Update<'a> {
    pub w: f32,
    pub h: f32,
    pub gravity_f: f32,
    pub agents: &'a crate::latex::Latex2D<Agent>,
    pub gravity: Vec<vec::Vec>,
}

#[derive(Clone, Copy)]
pub struct Agent {
    pub id: usize,
    pub pos: vec::Vec,
    pub vel: vec::Vec,
    pub radius: f32,
    pub view_range: f32,
    pub pos_w: f32,
    pub vel_w: f32,
    pub drag: f32,
    pub max_acc: f32,
    pub weirdness: f32,
    pub s_in_range: usize,
    pub s_vel: f32,
    pub color: f32,
}

impl Agent {
    pub fn new(id: usize, pos: vec::Vec, vel: vec::Vec) -> Agent {
        let a = Agent {
            id,
            pos,
            vel,
            s_vel: 0.0,
            s_in_range: 0,
            color:  //1.0,
                // 0.1,0.1,0.1,
                utils::rand_int(RULES.len() as u32) as f32 * (360 as f32/RULES.len() as f32),
                // utils::rand_float(0.0, 1.0),
                // utils::rand_float(0.0, 1.0),
            // ],

            // view_range: utils::rand_float(5.0, 200.0),
            // pos_w: utils::rand_float(-1.0, 0.0),
            // vel_w: utils::rand_float(0.0, 1.0),
            // drag: utils::rand_float(0.0001, 0.01),
            // max_acc: utils::rand_float(0.01, 0.4),
            // weirdness: utils::rand_float(0.1, 15.5),
            radius: 2.0,
            view_range: MAX_DIST,
            pos_w: -0.1,
            vel_w: 1.0,
            drag: DRAG,
            max_acc: MAXACC,
            weirdness: 1.0,
        };
        // utils::norm(&mut a.color);
        a
    }

    pub fn new_update(&mut self, update: &Update) {
        let color_friendlyness = |b: f32| -> f32 {
            let a = self.color;
            let ai = (a / (360 / RULES.len()) as f32).floor() as usize;
            let bi = (b / (360 / RULES.len()) as f32).floor() as usize;
            // println!("xx {}={} +  {}={} = {}", a, ai, b, bi, RULES[ai][bi]);
            return RULES[ai][bi];
        };

        let fix_pos = |mut p: vec::Vec| -> vec::Vec {
            let a = self.pos;
            if (p.x - a.x).abs() > (p.x - a.x + update.w).abs() {
                p.x += update.w;
            } else if (p.x - a.x).abs() > (p.x - a.x - update.w).abs() {
                p.x -= update.w;
            }
            if (p.y - a.y).abs() > (p.y - a.y + update.h).abs() {
                p.y += update.h;
            } else if (p.y - a.y).abs() > (p.y - a.y - update.h).abs() {
                p.x -= update.h;
            }
            return p;
        };
        // Items within latex distance
        let neighbours = update
            .agents
            .get_safe((self.pos.x, self.pos.y), self.view_range);
        if self.id == 2 {
            println!(
                "count: {}  res = {:.02}x{:.02} {:.02}x{:.02} ",
                neighbours.len(),
                update.agents.vsize().0,
                update.agents.vsize().1,
                update.agents.w,
                update.agents.h,
            );
        }

        let mut neighbours: Vec<(f32, &Agent)> = neighbours
            .into_iter()
            .map(|n| (fix_pos(n.pos).squared_dist(&self.pos), n))
            .collect();

        let mut top_keep: Vec<(f32, &Agent)> = vec![];
        while !neighbours.is_empty() {
            let n = neighbours.pop();

            if let Some(n) = n {
                if n.1.id == self.id {
                    continue;
                }
                // for (_dist, n) in neighbours {
                let f = color_friendlyness(n.1.color);
                // println!("xx fr {} {:.2}", f, distance / MAX_DIST);

                if f == 0.0 {
                    continue;
                }
                // println!("xx {:?}", top_keep.iter().map(|x| x.0).collect::<Vec<_>>());
                let mut better = if top_keep.is_empty() { Some(0) } else { None };
                for i in 0..top_keep.len() {
                    if top_keep[i].0 > n.0 {
                        better = Some(i);
                        break;
                    }
                }
                if let Some(better) = better {
                    // println!("xx insert {} = {}", n.0, better);
                    if KEEP == 0 {
                        top_keep.push(n);
                    } else if better <= KEEP.max(1) - 1 {
                        top_keep.insert(better, n);
                        if top_keep.len() > KEEP {
                            top_keep.pop();
                        }
                    }
                }
            }
        }

        // println!(
        //     "xx  final {:?}",
        //     top_keep.iter().map(|x| x.0).collect::<Vec<_>>()
        // );

        let mut wanted_dp_sum = vec::Vec::new();
        // let draw_vec = None;
        let mut adds = 0;
        for (dist, n) in top_keep {
            let distance = dist.sqrt();
            if distance > MAX_DIST {
                continue;
            }
            // for (_dist, n) in neighbours {
            let f = color_friendlyness(n.color);
            // println!("xx fr {} {:.2}", f, distance / MAX_DIST);

            if f == 0.0 {
                continue;
            }

            let n_pos = fix_pos(n.pos);

            let mut dp = n_pos.clone();
            dp.sub(&self.pos);

            // This is the position difference normalized
            let mut dir = dp.clone();
            dir.norm(1.0);

            if f > 0.0 {
                let delta = distance - WANTED_DP_LEN;
                wanted_dp_sum.add(
                    dir.clone()
                        .mul(0.7 * delta * f * (1.0 - distance / MAX_DIST).powi(2)),
                );
            } else {
                // Repulsive force
                wanted_dp_sum.add(dir.clone().mul(
                    100.0 * f / (0.1 + (distance / MAX_DIST * 10.0).exp())
                        * (1.0 - distance / MAX_DIST).powi(2),
                ));
            }

            let mut dv = n.vel.clone();
            dv.sub(&self.vel);
            dv.mul(f * 0.1);
            // wanted_dp_sum.add(&dv);

            adds += 1;
        }
        if wanted_dp_sum.mag() > 0.0 {
            // wanted_dp_sum.mul(self.max_acc / adds as f32);
            wanted_dp_sum.mul(self.max_acc);
            // wanted_dp_sum.limit(self.max_acc);
            self.vel.add(&wanted_dp_sum);
        }
    }

    pub fn new_update2(&mut self, update: &Update) {
        let by_dist = update.agents.get(self.pos.as_tuple(), 1000.0);
        let mut by_dist = by_dist
            .iter()
            .filter_map(|agent| {
                if agent.id == self.id {
                    None
                } else {
                    Some((self.pos.dist_mod(&agent.pos, update.w, update.h), agent))
                }
            })
            .collect::<Vec<_>>();
        by_dist.sort_by(|(a, _), (b, _)| {
            if a > b {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
        if by_dist.is_empty() {
            return;
        }
        // Skip self
        let (dist, closest) = by_dist[0];
        let mut dir = closest.pos.clone();

        // run away: the closest we are, the more we run
        // dir.sub(&self.pos).norm(10.0 / (dist + 1.0));
        // self.vel.sub(&dir);

        // come togheter: the closer, the less we run
        dir.sub(&self.pos).limit(dist / 1000.0);
        self.vel.add(&dir);
    }

    pub fn update(&mut self, update: &Update) {
        self.new_update(update);

        // Apply velocity
        self.pos.add(&self.vel);

        // Apply friction
        self.vel.mul(1.0 - self.drag);

        // Apply gravity
        for mut g in update.gravity.iter().cloned() {
            // let mut g = vec::Vec::new_from(update.w, update.h);
            g.sub(&self.pos);
            let mag = g.mag();
            g.mul(update.gravity_f.powf(1.4) * 0.2 / (100.0 + mag * mag));
            self.vel.add(&g);
        }

        // Correct the player position
        while self.pos.x > update.w {
            self.pos.x -= update.w
        }
        while self.pos.x < 0.0 {
            self.pos.x += update.w
        }
        while self.pos.y > update.h {
            self.pos.y -= update.h
        }
        while self.pos.y < 0.0 {
            self.pos.y += update.h
        }

        self.s_vel = self.vel.mag();
        // self.s_in_range = in_range_incl.len();
        // tim.tick("finish");
        // tim.show();
    }

    pub fn draw(&self, draw: &Draw, max_vel: f32, _max_range: f32) {
        let mut g = self.s_vel as f32 / max_vel * 1.5;
        let mut q = 1.0f32; // self.s_in_range as f32 / max_range;

        g = ((g - 0.0) * 1.0).max(0.0);
        q = ((q - 0.0) * 1.0).max(0.0);
        // let g = g;
        // let q = 0.0;

        // let col = graphics::Color::new(
        //     q,
        //     g,
        //     q*g, (q*g).max(0.1));
        let col = hsla(self.color / 360.0, 0.8, 0.4, (q + g) / 2.0);
        // let col = graphics::Color::new(1.0,1.0,1.0, (q*g).max(0.4));
        draw.rect()
            .color(col)
            .x_y(self.pos.x, self.pos.y)
            .w_h(AG_SIZE, AG_SIZE);
        let col = hsla(self.color / 360.0, 0.8, 0.4, 0.01);
        // draw.ellipse()
        //     .color(col)
        //     .x_y(self.pos.x, self.pos.y)
        //     .w_h(self.view_range, self.view_range);
        if DRAW_RANGE {
            draw.ellipse()
                .color(col)
                .x_y(self.pos.x, self.pos.y)
                .w_h(MAX_DIST * 2.0, MAX_DIST * 2.0);
        }
        if DRAW_WANTED_DP_LEN {
            draw.ellipse()
                .color(col)
                .x_y(self.pos.x, self.pos.y)
                .w_h(WANTED_DP_LEN * 2.0, WANTED_DP_LEN * 2.0);
        }

        //     2.8,
        //     1.0,
        //     // graphics::Color::new(q/2.0+0.1, g, q*g, (g*q).max(0.1)),
        //     col, // graphics::Color::new(g, 1.0 - g * 0.9, q, 0.5+g*0.5),
        // );
        // _mb_bg.circle(
        //     graphics::DrawMode::fill(),
        //     ggez::nalgebra::Point2::new(self.pos.x, self.pos.y),
        //     self.view_range/2.0,
        //     0.5,
        //     col,
        //     // graphics::Color::new(g, 1.0 - g * 0.9, q, 0.5+g*0.5),
        // );
    }
}

impl HasPosition for Agent {
    fn get_latex_pos(&self) -> &vec::Vec {
        &self.pos
    }
}

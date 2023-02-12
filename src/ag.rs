use nannou::prelude::hsla;
use nannou::prelude::rgba;
use nannou::wgpu::Color;
use nannou::Draw;

use crate::utils;
use crate::vec;

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
        let mut a = Agent {
            id,
            pos,
            vel,
            s_vel: 0.0,
            s_in_range: 0,
            color: 
                // 0.1,0.1,0.1,
                utils::rand_float(0.0, 360.0),
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
            view_range: 17.0,
            pos_w: -0.1,
            vel_w: 1.0,
            drag: 0.5,
            max_acc: 1.0,
            weirdness: 1.0,
        };
        // utils::norm(&mut a.color);
        a
    }

    pub fn new_update(&mut self, update: &Update) {
        let rules = [
            [1.0, 0.5, 0.0, -1.1],
            [-1.1, 1.0, 0.5, 0.0],
            [0.0, -1.1, 1.0, 0.5],
            [0.5, 0.0, -1.1, 1.0],
        ];

        let color_friendlyness = |b: f32| -> f32 {
            let a = self.color;
            let a = (a / (360 / rules.len()) as f32).floor() as usize;
            let b = (b / (360 / rules.len()) as f32).floor() as usize;
            return rules[a][b];
        };


        // Items within latex distance
        let neighbours = update.agents.get((self.pos.x, self.pos.y), self.view_range);

        let mut wanted_dp_sum = vec::Vec::new();
        // let draw_vec = None;
        for n in neighbours {
            if n.id == self.id {
                continue;
            }
            let mut f = color_friendlyness( n.color);
            if f == 0.0 {
                continue;
            }
            let max_dist = 3000.0;
            let wanted_dp_len = 10.0;

            let mut dp = n.pos.clone();
            dp.sub( &self.pos);

            if dp.mag() > max_dist {
                continue;
            }
            f *= 1.0 - dp.mag() / max_dist;
            if f == 0.0 {
                continue;
            }

            let mut dir = dp.clone();
            dir.div(dp.mag());

            let delta = wanted_dp_len - dp.mag();
            wanted_dp_sum.add(dir.clone().mul( -delta * f));

        }
        if wanted_dp_sum.mag() > 0.0 {
            wanted_dp_sum.limit(self.max_acc);
            self.vel.add( &wanted_dp_sum);
        }
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
        let col = hsla(self.color, 60.0, 50.0, (q + g) / 2.0);
        // let col = graphics::Color::new(1.0,1.0,1.0, (q*g).max(0.4));
        draw.ellipse()
            .color(col)
            .x_y(self.pos.x, self.pos.y)
            .w_h(10.0, 10.0)
            .radius(2.8);

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

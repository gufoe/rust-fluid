use crate::vec;
use crate::utils;

#[derive(Clone)]
pub struct Update <'a> {
    pub w: f32,
    pub h: f32,
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
    pub color: [f32; 3],
}

impl Agent {
    pub fn new(id: usize, pos: vec::Vec, vel: vec::Vec) -> Agent {
        let mut a = Agent {
            id,
            pos,
            vel,
            s_vel: 0.0,
            s_in_range: 0,
            radius: 5.0,
            color: [
                // 0.1,0.1,0.1,
                utils::rand_float(0.0, 1.0),
                utils::rand_float(0.0, 1.0),
                utils::rand_float(0.0, 1.0),
            ],

            // view_range: utils::rand_float(5.0, 200.0),
            // pos_w: utils::rand_float(-1.0, 0.0),
            // vel_w: utils::rand_float(0.0, 1.0),
            // drag: utils::rand_float(0.0001, 0.01),
            // max_acc: utils::rand_float(0.01, 0.4),
            // weirdness: utils::rand_float(0.1, 15.5),

            view_range: 17.0,
            pos_w: -0.1,
            vel_w: 1.0,
            drag: 0.0,
            max_acc: 5.0,
            weirdness: 1.0,
        };
        utils::norm(&mut a.color);
        a
    }

    fn red (&self) -> f32 {
        // self.color[0]
        self.color[0] / (self.color[1] + self.color[2]).max(0.2)
    }
    fn green (&self) -> f32 {
        // self.color[1]
        self.color[1] / (self.color[0] + self.color[2]).max(0.2)
    }
    fn blue (&self) -> f32 {
        // self.color[2]
        self.color[2] / (self.color[0] + self.color[1]).max(0.2)
    }
    fn white (&self) -> f32 {
        (self.color[0] + self.color[1] + self.color[2]) / 3.0
    }

    pub fn update(&mut self, update: &Update) {
        let mut in_range_incl = update.agents.get((self.pos.x, self.pos.y), self.view_range);
        in_range_incl.retain(|x| {
            let d = self.pos.dist_mod(&x.pos, update.w, update.h);
            d < self.view_range
        });
        // assert!(self.view_range > 0.0);
        // assert!(!in_range.is_empty());
        let mut in_range = in_range_incl.clone();
        in_range.retain(|x| {
            self.id != x.id
        });



        if !in_range.is_empty() {
            let mut entour = [0.0,0.0,0.0];
            let mut dom: Option<(f32, &Agent)> = None;
            in_range_incl.iter().for_each(|x| {
                entour[0]+= x.color[0] as f32;
                entour[1]+= x.color[1] as f32;
                entour[2]+= x.color[2] as f32;
                let d = self.vel.dist(&x.vel);
                // let mut d = self.pos.dist_mod(&x.pos, update.w, update.h);
                if dom.is_none() || dom.unwrap().0 < d {
                    dom = Some((d, x));
                }
            });
            utils::norm(&mut entour);
            // utils::vavg(&mut self.color, &entour, 0.5);

            if entour[0] < 0.9 {
                utils::eavg(&mut self.color[0], entour[1], 0.01);
                utils::eavg(&mut self.color[2], entour[2], 0.01);
            }
            if entour[1] < 0.9 {
                utils::eavg(&mut self.color[1], entour[2], 0.01);
                utils::eavg(&mut self.color[0], entour[0], 0.01);
            }
            if entour[2] < 0.9 {
                utils::eavg(&mut self.color[2], entour[0], 0.01);
                utils::eavg(&mut self.color[1], entour[1], 0.01);
            }


            // match dom {
            //     None => {},
            //     Some((_, dom)) => {
            //     }
            // }

            utils::norm(&mut self.color);
            // self.drag = 0.02+(self.white()*0.8).powi(2)*0.01;
            // // self.pos_w = -1.25 +(1.0- self.color[0]*1.0) + self.color[1] * 0.1 + self.color[2] * 0.1;
            {
                self.pos_w = 0.2 + self.color[1] * 4.0;
                self.vel_w = self.color[0] * 1.0;

                self.pos_w = (self.pos_w / 100.0).max(0.01);
                self.vel_w = (self.vel_w / 100.0).max(0.01);
                self.view_range = (10.0 + 20.0 * self.color[2]) * 0.2;
                self.pos_w*= -1.0;
                self.drag = self.color[0] * 0.01 + self.color[1] * 0.001;
            }
            // self.pos_w = -self.color[0].min(1.0).max(0.2)*20.0;
            // // self.vel_w = (1.0-self.color[1]*2.0).min(1.0).max(0.2)*10.0;
            // self.weirdness = 2.0;
            // self.drag = (1.0-self.color[2])*0.1;

            // self.pos_w = -30.0;
            // self.vel_w = 1.0;

            let mut avg_vel = vec::Vec::new();
            in_range.iter().for_each(|x| {
                let mut d = self.pos.dist_mod(&x.pos, update.w, update.h);
                d/= self.view_range;
                // d+= 1.0;
                // d*= 1.0;
                avg_vel.sub(&self.vel.clone().sub(&x.vel).mul(1.0-d));
                // avg_vel.sub(self.vel.clone().sub(&x.vel).mul(1.0/(d.powi(2))));
            });
            avg_vel.div(in_range.len() as f32);

            let mut avg_pos = vec::Vec::new();
            in_range.iter().for_each(|x| {
                let mut diff = self.pos.clone();
                diff.sub(&self.pos.rel(&x.pos, update.w, update.h));
                diff.div(diff.mag().max(1.0).powi(2));
                avg_pos.sub(&diff);
            });
            avg_pos.div(in_range.len() as f32);

            avg_vel.norm(1.0);
            avg_pos.norm(1.0);
            let diff = avg_vel.mul(self.vel_w)
            .add(avg_pos.mul(self.pos_w))
            .div(self.pos_w.abs() + self.vel_w.abs())
            .mul(self.weirdness);

            // let mut diff = avg_pos;
            // diff.limit(self.max_acc);
            // diff.norm(0.5);
            diff.mul(0.1);
            self.vel.add(&diff);
        }

        // self.vel.limit(10.0);
        self.pos.add(&self.vel);
        self.vel.mul(1.0-self.drag);


        for mut g in update.gravity.iter().cloned() {
            // let mut g = vec::Vec::new_from(update.w, update.h);
            g.sub(&self.pos);
            let mag = g.mag();
            g.mul(1.0/(100.0 + mag*mag));
            self.vel.add(&g);
        }
        // if self.pos.y > update.h {
        //     self.vel.y*= -0.8;
        //     self.pos.y = update.h - (update.h - self.pos.y).abs();
        // }

        while self.pos.x > update.w { self.pos.x-= update.w }
        while self.pos.x < 0.0 { self.pos.x+= update.w }
        while self.pos.y > update.h { self.pos.y-= update.h }
        while self.pos.y < 0.0 { self.pos.y+= update.h }

        self.s_vel = self.vel.mag();
        self.s_in_range = in_range.len();
    }

    pub fn draw(&self, _ctx: &mut ggez::Context,
                mb: &mut ggez::graphics::MeshBuilder,
                _mb_bg: &mut ggez::graphics::MeshBuilder,
                max_vel: f32,
                max_range: f32) {
        use ggez::graphics;


        let mut g = self.s_vel as f32 / max_vel * 1.5;
        let mut q = 1.0f32;// self.s_in_range as f32 / max_range;

        g = ((g-0.0)*1.0).max(0.0);
        q = ((q-0.0)*1.0).max(0.0);
        // let g = g;
        // let q = 0.0;

        // let col = graphics::Color::new(
        //     q,
        //     g,
        //     q*g, (q*g).max(0.1));
        let col = graphics::Color::new(
            self.color[0],
            self.color[1],
            self.color[2],
            (q + g) / 2.0);
        // let col = graphics::Color::new(1.0,1.0,1.0, (q*g).max(0.4));
        mb.circle(
            graphics::DrawMode::fill(),
            ggez::nalgebra::Point2::new(self.pos.x, self.pos.y),
            4.5,
            2.0,
            // graphics::Color::new(q/2.0+0.1, g, q*g, (g*q).max(0.1)),
            col
            // graphics::Color::new(g, 1.0 - g * 0.9, q, 0.5+g*0.5),
        );
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

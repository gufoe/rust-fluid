use rayon::prelude::*;
use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics;
use latex::Latex2D;
use ggez::input::keyboard::KeyMods;
use ggez::input::keyboard::KeyCode;
mod vec;
mod ag;
mod utils;
mod latex;

const AGENT_NUM: usize = 20000;
const ST_LEN: usize = 40;
const BRUSH_SIZE: f32 = 50.0;

fn main() {
    rayon::ThreadPoolBuilder::new().num_threads(12).build_global().expect("no thread pool");

    // Make a Context and an EventLoop.
    let (mut ctx, mut event_loop) =
       ContextBuilder::new("game_name", "author_name")
       .window_setup(ggez::conf::WindowSetup {
            title: "An easy, good game".to_owned(),
            samples: ggez::conf::NumSamples::Zero,
            vsync: true,
            icon: "".to_owned(),
            srgb: true,
        })
       .window_mode(ggez::conf::WindowMode {
            width: 1600.0,
            height: 800.0,
            maximized: false,
            fullscreen_type: ggez::conf::FullscreenType::Windowed,
            borderless: false,
            min_width: 0.0,
            max_width: 0.0,
            min_height: 0.0,
            max_height: 0.0,
            resizable: false,
        })
           .build()
           .expect("cannot make window");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object
    // so it can load resources like images during setup.
    let mut my_game = MyGame::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

struct MyGame {
    // Your state here...
    agents: Vec<ag::Agent>,
    frames: u32,
    latex: Latex2D<ag::Agent>,
    // pool: scoped_threadpool::Pool,
    btn_left: bool,
    btn_right: bool,
    btn_middle: bool,
    latex_div: f32,
    avg_stats_vel: Vec<f32>,
    avg_stats_range: Vec<f32>,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> MyGame {
        use rand::Rng;

            // graphics::set_fullscreen(ctx, ggez::conf::FullscreenType::True).unwrap();

        // Load/create resources here: images, fonts, sounds, etc.
        let mut agents = vec![];
        let (w, h) = graphics::drawable_size(ctx);
        while agents.len() < AGENT_NUM {
            agents.push(ag::Agent::new(
                agents.len(),
                vec::Vec {
                    x: rand::thread_rng().gen_range(0.0, w),
                    y: rand::thread_rng().gen_range(0.0, h),
                },
                vec::Vec {
                    x: 0.0,//rand::thread_rng().gen_range(-1.0, 1.0),
                    y: 0.0,//rand::thread_rng().gen_range(-1.0, 1.0),
                },
            ))
        }
        let mut game = MyGame {
            frames: 0,
            agents,
            latex: Latex2D::new(0.0, 0.0, 0.0),
            btn_left: false,
            btn_right: false,
            btn_middle: false,
            latex_div: 4.0,
            avg_stats_vel: vec![],
            avg_stats_range: vec![],
            // pool: scoped_threadpool::Pool::new(8),
        };

        game.adjust_latex_div(ctx);

        game
    }

    pub fn adjust_latex_div(&mut self, ctx: &mut Context) {
        let mut min: Option<(f64, f32)> = None;
        let ag = self.agents.clone();
        for ld in 10..70 {
            self.agents = ag.clone();
            self.latex_div = ld as f32;
            let t_start = utils::now();
            for _ in 0..2 {
                self.update(ctx).expect("no update");
                self.frames-= 1;
            }
            let t_diff = utils::now() - t_start;
            println!("ld {}: {:.4}", ld, t_diff);
            if !min.is_none() && min.unwrap().0 < t_diff {
                break
            }
            if min.is_none() || min.unwrap().0 > t_diff {
                min = Some((t_diff, ld as f32));
            }
        }
        println!("best latex div: {:?}", min.unwrap());
        self.agents = ag.clone();
        self.latex_div = min.unwrap().1 + 6.0;
    }

    pub fn update_latex(&mut self, w: f32, h: f32) {
        let mut latex = Latex2D::new(w / 2.0 / self.latex_div, w, h);
        let _t0 = utils::now();
        self.agents.iter().for_each(|x| latex.add((x.pos.x, x.pos.y), x.clone()));
        // println!("latex:   {:.3}", utils::now() - _t0);
        self.latex = latex
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let (w, h) = graphics::drawable_size(ctx);
        self.update_latex(w, h);

        let update = ag::Update {
            w,
            h,
            agents: &self.latex,
        };

        let _t0 = utils::now();
        self.agents.par_iter_mut().for_each(|x| x.update(&update));
        self.update_latex(w, h);
        // println!("update:  {:.3}", utils::now() - _t0);
        // self.agents.remove(0);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let (w, h) = graphics::drawable_size(ctx);
        // if self.frames == 1 {
        // }
        // graphics::clear(ctx, graphics::BLACK);
        // graphics::clear(ctx, graphics::Color::new(0.0, 0.0, 0.0, 0.01));

        // Draw bbackground
        let mut mb_bg = &mut graphics::MeshBuilder::new();
        mb_bg.rectangle(graphics::DrawMode::fill(), graphics::Rect::new(0.0, 0.0, w, h),
                graphics::Color::new(0.0, 0.0, 0.0, 0.62));

        // Get stats
        let max_speed: f32 = self.agents.par_iter()
            .fold(|| 0.0, |v: f32, x| v.max(x.s_vel))
            .reduce(|| 0.0, |v: f32, x| v.max(x));
        let mut col: [f32; 3] = self.agents.par_iter()
            .fold(|| [0.0, 0.0, 0.0], |v, x| utils::sum(&v, &x.color))
            .reduce(|| [0.0, 0.0, 0.0], |v, x| utils::sum(&v, &x));
        utils::softmax_fast(&mut col);
        let mut stats_mesh = ggez::graphics::MeshBuilder::new();
        let mut tot = 0.0;
        let width = 1000.0;
        let height = 20.0;
        for i in 0..3 {
            stats_mesh.rectangle(
                graphics::DrawMode::fill(),
                ggez::graphics::Rect::new(10.0 + tot * width, 10.0,
                    col[i] * width, height),
                    graphics::Color::new(
                        if i == 0 { 1.0 } else { 0.0 },
                        if i == 1 { 1.0 } else { 0.0 },
                        if i == 2 { 1.0 } else { 0.0 },
                        1.0
                    )
            );
            tot+= col[i];
        }

        self.avg_stats_vel.push(max_speed);
        if self.avg_stats_vel.len() > ST_LEN { self.avg_stats_vel.remove(0); }
        let max_speed = utils::avg(&self.avg_stats_vel);

        let max_range: f32 = self.agents.par_iter()
            .fold(|| 0.0, |v: f32, x| v.max(x.s_in_range as f32))
            .reduce(|| 0.0, |v: f32, x| v.max(x));
        self.avg_stats_range.push(max_range as f32);
        if self.avg_stats_range.len() > ST_LEN { self.avg_stats_range.remove(0); }
        let max_range = utils::avg(&self.avg_stats_range);

        // Draw agents
        let _t0 = utils::now();
        let mut mb = &mut graphics::MeshBuilder::new();
        self.agents.iter().for_each(|x| x.draw(ctx, &mut mb, &mut mb_bg, max_speed, max_range));

        // Draw background and foreground
        let mb_bg = mb_bg.build(ctx).unwrap();
        graphics::draw(ctx, &mb_bg, graphics::DrawParam::new()).unwrap();
        let mb = mb.build(ctx).unwrap();
        graphics::draw(ctx, &mb, graphics::DrawParam::new()).unwrap();
        let stats_mesh = stats_mesh.build(ctx).unwrap();
        graphics::draw(ctx, &stats_mesh, graphics::DrawParam::new()).unwrap();
        // println!("prebuild:   {:.3}", utils::now() - _t0);

        // println!("draw:       {:.3}", utils::now() - _t0);
        // println!("build:      {:.3}", utils::now() - _t0);

        // let _t0 = utils::now();
        graphics::present(ctx)?;
        // println!("present:    {:.3}", utils::now() - _t0);

        self.frames += 1;
        if (self.frames % 30) == 0 {
            println!("FPS: {:.2}", ggez::timer::fps(ctx));
        }

        ggez::timer::yield_now();
        Ok(())
    }
    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32
    ) {
        let p = vec::Vec::new_from(x, y);
        let d = vec::Vec::new_from(dx, dy);
        let radius = BRUSH_SIZE;
        if self.btn_left {
            let agents: Vec<usize> = self.latex.get((x, y), radius).iter().map(|x| x.id).collect();
            // let mut i: Vec<usize> = Vec::new();
            // let mut ids = std::collections::HashSet::new();
            agents.iter().for_each(|id| {
                // ids.insert(id);
                let x = self.agents.get_mut(*id).expect("element in latex too much");
                let dist = x.pos.dist_mod(&p, self.latex.w, self.latex.h);
                if dist > radius { return; }
                let mut d = d.clone();
                // d.mul(2.0);
                d.mul(( 1.0 - dist / radius) * 0.1);
                x.vel.add(&d);
                // x.pos.add(&d);
            });
            // self.agents.retain(|a| !ids.contains(&a.id));
        }
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: ggez::input::mouse::MouseButton,
        _x: f32,
        _y: f32
    ) {
        use ggez::input::mouse::MouseButton as mb;
        match _button {
            mb::Left => self.btn_left = true,
            mb::Right => self.btn_right = true,
            mb::Middle => {
                self.btn_middle = true;
                self.adjust_latex_div(_ctx);
            },
            mb::Other(_) => {},
        }
    }
    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: ggez::input::mouse::MouseButton,
        _x: f32,
        _y: f32
    ) {
        use ggez::input::mouse::MouseButton as mb;
        match _button {
            mb::Left => self.btn_left = false,
            mb::Right => self.btn_right = false,
            mb::Middle => self.btn_middle = false,
            mb::Other(_) => {},
        }
    }
    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32) {
        self.agents.par_iter_mut().for_each(|x| {
            x.pos_w+= _y;
        });
        println!("pos_w set to {}", self.agents[0].pos_w);
    }


    fn key_down_event(&mut self, _ctx: &mut Context, key: KeyCode, mods: KeyMods, _: bool) {
        let rounds = if mods.contains(KeyMods::SHIFT | KeyMods::CTRL) {
            1000
        } else {
            100
        };
        match key {
            // Quit if Shift+Ctrl+Q is pressed.
            KeyCode::Escape => {
                event::quit(_ctx);
            }
            KeyCode::R => {
                println!("making one aggressive");
                for _ in 0..rounds {
                    let s = self.agents.len();
                    self.agents.get_mut(utils::rand_usize(s)).unwrap().color = [1.0, 0.0, 0.0];
                }

            }
            KeyCode::B => {
                println!("making one aggressive");
                for _ in 0..rounds {
                    let s = self.agents.len();
                    self.agents.get_mut(utils::rand_usize(s)).unwrap().color = [0.0, 0.0, 1.0];
                }

            }
            KeyCode::G => {
                println!("making one green");
                for _ in 0..rounds {
                    let s = self.agents.len();
                    self.agents.get_mut(utils::rand_usize(s)).unwrap().color = [0.0, 1.0, 0.0];
                }

            }
            _ => (),
        }
    }
}


#[test]
fn test_vec () {
    let w = 10.0;
    let h = 10.0;
    let tol = 0.001;

    let v1 = vec::Vec { x: w-1.0, y: 0.0 };
    let v2 = vec::Vec { x: w-2.0, y: 0.0 };
    let rel = v1.rel(&v2, w, h);
    println!("{:?}", rel);
    assert!((rel.dist(&v1) - 1.0).abs() < tol);

    let v1 = vec::Vec { x: w-1.0, y: 0.0 };
    let v2 = vec::Vec { x: 1.0, y: 0.0 };
    let rel = v1.rel(&v2, w, h);
    println!("{:?}", rel);
    assert!((rel.dist(&v1) - 2.0).abs() < tol);

    let v1 = vec::Vec { x: 1.0, y: 0.0 };
    let v2 = vec::Vec { x: w-1.0, y: 0.0 };
    let rel = v1.rel(&v2, w, h);
    println!("{:?}", rel);
    assert!((rel.dist(&v1) - 2.0).abs() < tol);

    let v1 = vec::Vec { x: 1.0, y: 0.0 };
    let v2 = vec::Vec { x: 2.0, y: 0.0 };
    let rel = v1.rel(&v2, w, h);
    println!("{:?}", rel);
    assert!((rel.dist(&v1) - 1.0).abs() < tol);
}

#[test]
fn test_latex () {
    let res = 10.0;
    let w = 100.0;
    let h = 100.0;
    let mut latex = latex::Latex2D::new(res, w, h);
    latex.add((99.0, 0.0), 1);
    assert!(latex.get((0.0, 0.0), 2.0) == vec![1]);
    assert!(latex.get((0.0, 0.0), 10.0) == vec![1]);
    assert!(latex.get((0.0, 0.0), 20.0) == vec![1]);
    assert!(latex.get((0.0, 0.0), 1000.0) == vec![1]);

    let mut latex = latex::Latex2D::new(res, w, h);
    latex.add((99.0, 99.0), 1);
    latex.add((51.0, 0.0), 2);
    latex.add((50.0, 0.0), 3);
    assert!(latex.get((0.0, 0.0), 2.0) == vec![1]);
    assert!(latex.get((50.0, 0.0), 10.0) == vec![2, 3]);
}

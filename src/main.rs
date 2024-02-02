use latex::Latex2D;
use nannou::{prelude::*, state::Mouse};
use rayon::prelude::*;

mod ag;
mod latex;
mod utils;
mod vec;

const AGENT_NUM: usize = 6000;
const ST_LEN: usize = 40;
#[allow(dead_code)]
const BRUSH_SIZE: f32 = 100.0;
#[allow(unused)]
macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);
fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

fn model(app: &App) -> MyGame {
    MyGame::new(app)
}
fn update(app: &App, game: &mut MyGame, _update: Update) {
    if game.agents.is_empty() {
        game.init_agents(app)
    }
    game.mouse_events(app);
    game.update_agents(app, true);
}
fn view(app: &App, game: &MyGame, frame: Frame) {
    game.view(app, frame);
}

#[derive(Default)]
struct GemeOptions {}

struct MyGame {
    // Your state here...
    agents: Vec<ag::Agent>,
    frames: u32,
    frames_start: f64,
    latex: Latex2D<ag::Agent>,
    // pool: scoped_threadpool::Pool,
    #[allow(dead_code)]
    key_mod: String,
    // #[allow(dead_code)]
    // btn_left: bool,
    // #[allow(dead_code)]
    // btn_right: bool,
    // #[allow(dead_code)]
    // btn_middle: bool,
    prev_mouse: Option<Mouse>,
    gravity_mod: usize,
    gravity_f: f32,
    latex_div: f32,
    avg_stats_vel: Vec<f32>,
    avg_stats_range: Vec<f32>,
    fast: usize,
}

impl MyGame {
    pub fn new(_app: &App) -> MyGame {
        // Load/create resources here: images, fonts, sounds, etc.
        let game = MyGame {
            frames: 0,
            frames_start: utils::now(),
            agents: vec![],
            latex: Latex2D::new(1.0, 1.0, 0.0, 0.0),
            gravity_mod: 0,
            gravity_f: 1.0,
            latex_div: 4.0,
            avg_stats_vel: vec![],
            avg_stats_range: vec![],
            prev_mouse: None,
            fast: 0,
            key_mod: "F".to_string(), // TODO: KeyCode::F,
        };

        game
    }

    pub fn init_agents(&mut self, app: &App) {
        let (w, h) = app.window_rect().w_h();
        let mut agents = vec![];
        while agents.len() < AGENT_NUM {
            agents.push(ag::Agent::new(
                agents.len(),
                vec::Vec {
                    x: random_f32() * w,
                    y: random_f32() * h,
                },
                vec::Vec {
                    x: 0.0, //rand::thread_rng().gen_range(-1.0, 1.0),
                    y: 0.0, //rand::thread_rng().gen_range(-1.0, 1.0),
                },
            ))
        }
        self.agents = agents;
        self.adjust_latex_div(app);
    }

    pub fn restart_fps(&mut self) {
        self.frames = 0;
        self.frames_start = utils::now();
    }

    pub fn get_fps(&self) -> f32 {
        self.frames as f32 / (utils::now() - self.frames_start) as f32
    }

    pub fn adjust_latex_div(&mut self, app: &App) {
        let mut min: Option<(f64, f32)> = None;
        let ag = self.agents.clone();
        for _ in 0..10 {
            self.agents = ag.clone();
            self.latex_div = 10.0 + (random::<usize>() % 100) as f32;
            // Boot up
            self.update_agents(app, true);

            // Measure
            let t_start = utils::now();
            for _ in 0..4 {
                self.update_agents(app, true);
            }
            let t_diff = utils::now() - t_start;

            // Compare
            println!("adj_latex: ld {}: {:.4}", self.latex_div, t_diff);
            // if !min.is_none() && min.unwrap().0 < t_diff {
            //     break;
            // }
            if min.is_none() || min.unwrap().0 > t_diff {
                min = Some((t_diff, self.latex_div as f32));
            }
        }
        println!("adj_latex: best latex div: {:?}", min.unwrap());
        self.agents = ag;
        self.latex_div = min.unwrap().1;
        self.restart_fps();
    }

    pub fn update_latex(&mut self, w: f32, h: f32) {
        let mut latex = Latex2D::new(w / self.latex_div, h / self.latex_div, w, h);
        let _t0 = utils::now();
        self.agents
            .iter()
            .for_each(|x| latex.add((x.pos.x, x.pos.y), x.clone()));
        // println!("latex:   {:.3}", utils::now() - _t0);
        self.latex = latex
    }

    pub fn update_stats(&mut self) {
        // Get stats
        let max_speed: f32 = self
            .agents
            .par_iter()
            .fold(|| 0.0, |v: f32, x| v.max(x.s_vel))
            .reduce(|| 0.0, |v: f32, x| v.max(x));
        self.avg_stats_vel.push(max_speed);
        if self.avg_stats_vel.len() > ST_LEN {
            self.avg_stats_vel.remove(0);
        }
        let max_range: f32 = self
            .agents
            .par_iter()
            .fold(|| 0.0, |v: f32, x| v.max(x.s_in_range as f32))
            .reduce(|| 0.0, |v: f32, x| v.max(x));
        self.avg_stats_range.push(max_range as f32);
        if self.avg_stats_range.len() > ST_LEN {
            self.avg_stats_range.remove(0);
        }
    }
    pub fn update_agents(&mut self, app: &App, parallel: bool) {
        let (w, h) = app.window_rect().w_h();
        let pos = Vec2::new(app.mouse.x, app.mouse.y);
        app.mouse.buttons.left();

        for _ in 0..(self.fast * 2).max(1) {
            let mut tim = utils::Timer::new("UPDATE");
            self.update_latex(w, h);
            tim.tick("latex updated");

            let _dx = (self.frames as f32 * 0.03).cos() * 0.3;
            let _dy = (self.frames as f32 * 0.03).sin() * 0.3;

            let update = ag::Update {
                w,
                h,
                agents: &self.latex,
                gravity_f: self.gravity_f,
                gravity: match self.gravity_mod {
                    1 => vec![vec::Vec::new_from(w * 0.5, h * 0.5)],
                    2 => vec![
                        vec::Vec::new_from(w * 0.5, h * 0.5),
                        vec::Vec::new_from(w * (0.5 + _dx), h * (0.5 + _dy)),
                    ],
                    3 => vec![
                        vec::Vec::new_from(w * 0.5, h * 0.5),
                        vec::Vec::new_from(pos.x, pos.y),
                    ],
                    4 => vec![vec::Vec::new_from(pos.x, pos.y)],
                    _ => vec![],
                },
            };
            tim.tick("update ready");

            if parallel {
                self.agents.par_iter_mut().for_each(|x| x.update(&update));
            } else {
                self.agents.iter_mut().for_each(|x| x.update(&update));
            }
            self.frames += 1;

            tim.tick("agents updated");

            self.update_stats();

            tim.tick("stats updated");
            tim.show();
        }
        // println!("update:  {:.3}", utils::now() - _t0);
        // game.agents.remove(0);
    }
    fn view(&self, app: &App, frame: Frame) {
        let mut tim = utils::Timer::new("DRAW");

        let (w, h) = frame.rect().w_h();
        let draw = app.draw().translate(vec3(-w / 2.0, -h / 2.0, 0.0));

        // if self.frames == 1 {
        // }
        // graphics::clear(ctx, graphics::BLACK);
        // graphics::clear(ctx, graphics::Color::new(0.0, 0.0, 0.0, 0.01));

        // Draw bbackground
        // let mut mb_bg = &mut graphics::MeshBuilder::new();
        draw.rect()
            .color(rgba(0.0, 0.0, 0.0, 1.0))
            .w_h(w * 2.0, h * 2.0);
        //     graphics::DrawMode::fill(),
        //     graphics::Rect::new(0.0, 0.0, w, h),
        //     graphics::Color::new(0.0, 0.0, 0.0, 0.97),
        // );

        // let mut col: [f32; 3] = self
        //     .agents
        //     .par_iter()
        //     .fold(|| [0.0, 0.0, 0.0], |v, x| utils::sum(&v, &x.color))
        //     .reduce(|| [0.0, 0.0, 0.0], |v, x| utils::sum(&v, &x));
        // utils::softmax_fast(&mut col);

        tim.tick("done stats done");

        // let mut stats_mesh = ggez::graphics::MeshBuilder::new();
        // let mut tot = 0.0;
        // let width = 1000.0;
        // let height = 20.0;
        // for i in 0..3 {
        //     // stats_mesh.rectangle(
        //     //     graphics::DrawMode::fill(),
        //     //     ggez::graphics::Rect::new(10.0 + tot * width, 10.0, col[i] * width, height),
        //     //     graphics::Color::new(),
        //     // );

        //     draw.rect()
        //         .color(rgba(
        //             if i == 0 { 1.0 } else { 0.0 },
        //             if i == 1 { 1.0 } else { 0.0 },
        //             if i == 2 { 1.0 } else { 0.0 },
        //             1.0,
        //         ))
        //         .x_y(10.0 + tot * width, 10.0)
        //         .w_h(col[i] * width, height);
        //     tot += col[i];
        // }

        tim.tick("draw stats done");
        let max_speed = utils::avg(&self.avg_stats_vel);

        let max_range = utils::avg(&self.avg_stats_range);

        tim.tick("drew stats");

        // Draw agents
        let _t0 = utils::now();
        // let mut mb = &mut graphics::MeshBuilder::new();
        self.agents
            .iter()
            .for_each(|x| x.draw(&draw, max_speed, max_range));
        tim.tick("drew agents");

        // Draw background and foreground
        // let mb_bg = mb_bg.build().unwrap();
        // graphics::draw(ctx, &mb_bg, graphics::DrawParam::new()).unwrap();
        // let mb = mb.build().unwrap();
        // graphics::draw(ctx, &mb, graphics::DrawParam::new()).unwrap();
        // let stats_mesh = stats_mesh.build().unwrap();
        // graphics::draw(ctx, &stats_mesh, graphics::DrawParam::new()).unwrap();
        // println!("prebuild:   {:.3}", utils::now() - _t0);

        // println!("draw:       {:.3}", utils::now() - _t0);
        // println!("build:      {:.3}", utils::now() - _t0);

        // let _t0 = utils::now();
        tim.tick("drawed bg and fg");

        // graphics::present(ctx)?;
        tim.tick("presented");
        // println!("present:    {:.3}", utils::now() - _t0);

        // print!("{}[2J", 27 as char);
        println!(
            "FPS:  DRAW = {:.2}  UPDATE = {:.2}",
            0.0, // ggez::timer::fps(ctx),
            self.get_fps()
        );
        tim.show();
        if utils::now() > self.frames_start + 1.0 {
            // self.restart_fps();
        }

        draw.to_frame(app, &frame).unwrap();
    }

    fn mouse_events(&mut self, app: &App) {
        if self.prev_mouse.is_none() {
            self.prev_mouse = Some(app.mouse.clone());
            return;
        }
        let prev = self.prev_mouse.clone().unwrap();
        self.prev_mouse = Some(app.mouse.clone());

        if prev.buttons.middle().is_down() && app.mouse.buttons.middle().is_up() {
            self.adjust_latex_div(app);
        }

        let p = vec::Vec::new_from(
            app.mouse.x + app.window_rect().w() / 2.0,
            app.mouse.y + app.window_rect().h() / 2.0,
        );

        let d = vec::Vec::new_from(app.mouse.x - prev.x, app.mouse.y - prev.y);

        let radius = BRUSH_SIZE;
        if app.mouse.buttons.left().is_down() {
            let agents: Vec<usize> = self
                .latex
                .get(
                    (
                        p.x, // / 2.,
                        p.y, // / 2.,
                    ),
                    radius,
                )
                .iter()
                .map(|x| x.id)
                .collect();
            // let mut i: Vec<usize> = Vec::new();
            // let mut ids = std::collections::HashSet::new();
            agents.iter().for_each(|id| {
                // ids.insert(id);
                let x = self.agents.get_mut(*id).expect("element in latex too much");
                let dist = x.pos.dist_mod(&p, self.latex.w, self.latex.h);
                if dist > radius {
                    return;
                }
                let mut d = d.clone();
                // d.mul(2.0);
                d.mul((1.0 - dist / radius) * 0.3);
                x.vel.add(&d);
                // x.pos.add(&d);
            });
            // self.agents.retain(|a| !ids.contains(&a.id));
        }
    }
}
// impl EventHandler for MyGame {
//     fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, dx: f32, dy: f32) {
//         let p = vec::Vec::new_from(x, y);
//         let d = vec::Vec::new_from(dx, dy);
//         let radius = BRUSH_SIZE;
//         if self.btn_left {
//             let agents: Vec<usize> = self
//                 .latex
//                 .get((x, y), radius)
//                 .iter()
//                 .map(|x| x.id)
//                 .collect();
//             // let mut i: Vec<usize> = Vec::new();
//             // let mut ids = std::collections::HashSet::new();
//             agents.iter().for_each(|id| {
//                 // ids.insert(id);
//                 let x = self.agents.get_mut(*id).expect("element in latex too much");
//                 let dist = x.pos.dist_mod(&p, self.latex.w, self.latex.h);
//                 if dist > radius {
//                     return;
//                 }
//                 let mut d = d.clone();
//                 // d.mul(2.0);
//                 d.mul((1.0 - dist / radius) * 0.1);
//                 x.vel.add(&d);
//                 // x.pos.add(&d);
//             });
//             // self.agents.retain(|a| !ids.contains(&a.id));
//         }
//     }

//     fn mouse_button_down_event(
//         &mut self,
//         _ctx: &mut Context,
//         _button: ggez::input::mouse::MouseButton,
//         _x: f32,
//         _y: f32,
//     ) {
//         use ggez::input::mouse::MouseButton as mb;
//         match _button {
//             mb::Left => self.btn_left = true,
//             mb::Right => self.btn_right = true,
//             mb::Middle => {
//                 self.btn_middle = true;
//                 self.adjust_latex_div(_ctx);
//             }
//             mb::Other(_) => {}
//         }
//     }
//     fn mouse_button_up_event(
//         &mut self,
//         _ctx: &mut Context,
//         _button: ggez::input::mouse::MouseButton,
//         _x: f32,
//         _y: f32,
//     ) {
//         use ggez::input::mouse::MouseButton as mb;
//         match _button {
//             mb::Left => self.btn_left = false,
//             mb::Right => self.btn_right = false,
//             mb::Middle => self.btn_middle = false,
//             mb::Other(_) => {}
//         }
//     }
//     fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32) {
//         self.agents.par_iter_mut().for_each(|x| {
//             x.pos_w += _y;
//         });
//         println!("pos_w set to {}", self.agents[0].pos_w);
//     }

//     fn key_down_event(&mut self, _ctx: &mut Context, key: KeyCode, mods: KeyMods, _: bool) {
//         let rounds = if mods.contains(KeyMods::SHIFT | KeyMods::CTRL) {
//             1000
//         } else {
//             100
//         };
//         match key {
//             KeyCode::F
//             | KeyCode::A
//             | KeyCode::R
//             | KeyCode::B
//             | KeyCode::G
//             | KeyCode::Z
//             | KeyCode::X
//             | KeyCode::Escape => {
//                 self.key_mod = key;
//             }
//             _ => {}
//         }

//         let input_num = map! {
//             KeyCode::Key1 => 1,
//             KeyCode::Key2 => 2,
//             KeyCode::Key3 => 3,
//             KeyCode::Key4 => 4,
//             KeyCode::Key5 => 5,
//             KeyCode::Key6 => 6,
//             KeyCode::Key7 => 7,
//             KeyCode::Key8 => 8,
//             KeyCode::Key9 => 9,
//             KeyCode::Key0 => 0
//         };
//         let input_num = input_num.get(&key);

//         println!("keymod: {:?}, input_num: {:?}", self.key_mod, input_num);

//         match self.key_mod {
//             // Quit if Shift+Ctrl+Q is pressed.
//             KeyCode::Escape => {
//                 event::quit(_ctx);
//             }
//             KeyCode::A => {
//                 println!("making one aggressive");
//                 for _ in 0..rounds {
//                     let s = self.agents.len();
//                     self.agents.get_mut(utils::rand_usize(s)).unwrap().color = [1.0, 0.0, 0.0];
//                 }
//             }
//             KeyCode::R => {
//                 println!("making one aggressive");
//                 for _ in 0..rounds {
//                     let s = self.agents.len();
//                     self.agents.get_mut(utils::rand_usize(s)).unwrap().color = [1.0, 0.0, 0.0];
//                 }
//             }
//             KeyCode::B => {
//                 println!("making one aggressive");
//                 for _ in 0..rounds {
//                     let s = self.agents.len();
//                     self.agents.get_mut(utils::rand_usize(s)).unwrap().color = [0.0, 0.0, 1.0];
//                 }
//             }
//             KeyCode::G => {
//                 println!("making one green");
//                 for _ in 0..rounds {
//                     let s = self.agents.len();
//                     self.agents.get_mut(utils::rand_usize(s)).unwrap().color = [0.0, 1.0, 0.0];
//                 }
//             }
//             KeyCode::F => {
//                 println!("making fast");
//                 match input_num {
//                     Some(f) => {
//                         self.fast = *f;
//                     }
//                     _ => {}
//                 }
//             }
//             KeyCode::Z => {
//                 println!("gravity change");
//                 match input_num {
//                     Some(f) => {
//                         self.gravity_mod = *f;
//                     }
//                     _ => {}
//                 }
//             }
//             KeyCode::X => {
//                 println!("gravity force");
//                 match input_num {
//                     Some(f) => {
//                         self.gravity_f = *f as f32;
//                     }
//                     _ => {}
//                 }
//             }
//             _ => (),
//         }
//     }
// }

#[test]
fn test_vec() {
    let w = 10.0;
    let h = 10.0;
    let tol = 0.001;

    let v1 = vec::Vec { x: w - 1.0, y: 0.0 };
    let v2 = vec::Vec { x: w - 2.0, y: 0.0 };
    let rel = v1.rel(&v2, w, h);
    println!("{:?}", rel);
    assert!((rel.dist(&v1) - 1.0).abs() < tol);

    let v1 = vec::Vec { x: w - 1.0, y: 0.0 };
    let v2 = vec::Vec { x: 1.0, y: 0.0 };
    let rel = v1.rel(&v2, w, h);
    println!("{:?}", rel);
    assert!((rel.dist(&v1) - 2.0).abs() < tol);

    let v1 = vec::Vec { x: 1.0, y: 0.0 };
    let v2 = vec::Vec { x: w - 1.0, y: 0.0 };
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
fn test_latex() {
    let res = 10.0;
    let w = 100.0;
    let h = 100.0;
    let mut latex = latex::Latex2D::new(res, w, h);
    latex.add((0.0, 0.0), 1);
    assert_eq!(latex.get((0.0, 0.0), 2.0), vec![&1]);
    assert_eq!(latex.get((0.0, 0.0), 10.0), vec![&1]);
    assert_eq!(latex.get((0.0, 0.0), 20.0), vec![&1]);
    assert_eq!(latex.get((0.0, 0.0), 1000.0), vec![&1]);
    assert_eq!(latex.get((w, h), 2.0), vec![&1]);
    assert_eq!(latex.get((w, h), 10.0), vec![&1]);
    assert_eq!(latex.get((w, h), 20.0), vec![&1]);
    assert_eq!(latex.get((w, h), 1000.0), vec![&1]);
    assert_eq!(latex.get((w, h - 3.0), 2.0), vec![] as Vec<&i32>);
    assert_eq!(latex.get((w, h - 3.0), 3.0), vec![&1] as Vec<&i32>);
    assert_eq!(latex.get((0.0, 9.0), 2.0), vec![&1] as Vec<&i32>);

    assert_eq!(latex.get((0.0, -9.0), 2.0), vec![] as Vec<&i32>);
    assert_eq!(latex.get((0.0, -9.0), 10.0), vec![&1] as Vec<&i32>);

    assert_eq!(latex.get((w, h + 3.0), 3.0), vec![&1] as Vec<&i32>);
    assert_eq!(latex.get((w, h), 10.0), vec![&1]);
    assert_eq!(latex.get((w, h), 20.0), vec![&1]);
    assert_eq!(latex.get((w, h), 1000.0), vec![&1]);
}

#[test]
fn test_latex2() {
    let res = 10.0;
    let w = 100.0;
    let h = 100.0;
    let mut latex = latex::Latex2D::new(res, w, h);
    latex.add((99.0, 99.0), 1);
    latex.add((51.0, 0.0), 2);
    latex.add((50.0, 0.0), 3);
    assert_eq!(latex.get((0.0, 0.0), 2.0), vec![&1]);
    assert_eq!(latex.get((50.0, 0.0), 10.0), vec![&2, &3]);
}
#[test]
fn test_latex3() {
    let res = 10.0;
    let w = 100.0;
    let h = 100.0;
    let mut latex = latex::Latex2D::new(res, w, h);
    latex.add((99.0, 99.0), 1);
    latex.add((51.0, 0.0), 2);
    latex.add((50.0, 0.0), 3);
    assert_eq!(latex.get((0.0, 0.0), 2.0), vec![&1]);
    assert_eq!(latex.get((50.0, 0.0), 10.0), vec![&2, &3]);
}

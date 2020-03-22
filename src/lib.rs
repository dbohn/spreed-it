mod utils;
mod human;
mod zone;

use wasm_bindgen::prelude::*;

use web_sys::{CanvasRenderingContext2d};

use human::{Human, Vector, Health};
use zone::{Zone};

use std::f64;

extern crate js_sys;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct Universe {
    width: f64,
    height: f64,
    humans: Vec<Human>,
    ticks: u128,
    quarantine : Zone,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: f64, height: f64, humans: u32) -> Universe {

        let quarantine = Zone {
            x1: 100.0,
            x2: 102.0,
        };

        let mut humans : Vec<Human> = Vec::with_capacity(humans as usize);

        while humans.len() < humans.capacity() {
            let mut human = Human {
                    pos: Vector {
                        x: 15.0 + js_sys::Math::random() * (width - 30.0),
                        y: 15.0 + js_sys::Math::random() * (height - 30.0),
                    },
                    velocity: Vector::normalize(
                        js_sys::Math::random() * 2.0 - 1.0,
                        js_sys::Math::random() * 2.0 - 1.0,
                    ),
                    health: if humans.len() == 0 { Health::Infected } else { Health::Susceptible },
                    thickness: 10.0
                };

            let mut collision_counter = humans.len();
            while collision_counter != 0 {
                collision_counter = humans.len();
                for i in 0..humans.len() {
                    if human.collide(&humans[i]) {
                        human.pos.x =  15.0 + js_sys::Math::random() * (width - 30.0);
                        human.pos.y = 15.0 + js_sys::Math::random() * (height - 30.0);
                    } else {
                        collision_counter -= 1;
                    }
                }
            }
            humans.push(human);
        }
/*
        let humans = (0..1).map(|_i| {
            console_log!("X: {}, Velocity: {}", width * (_i as f64), if _i == 0 { 1.0 } else { -1.0 });
            Human {
                pos: Vector {
                    x: 10.0,
                    y: 10.0,
                },
                velocity: Vector::normalize(
                    1.0,
                    1.0,
                ),
                health: Health::Susceptible,
                thickness: 10.0
            }
        })
        .collect();
*/
        Universe {
            width,
            height,
            humans,
            quarantine,
            ticks: 0
        }
    }

    pub fn width(&self) -> f64 {
        self.width
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn render(&self, ctx: &CanvasRenderingContext2d) {
        ctx.clear_rect(0.0, 0.0, self.width.into(), self.height.into());

        let mut q = self.quarantine.clone();

        ctx.begin_path();
        ctx.rect(q.x1, 0.0, q.x2-q.x1, self.height);
        ctx.stroke();


        for human in self.humans.iter() {
            ctx.begin_path();
            ctx.set_fill_style(&
                match human.health {
                    Health::Susceptible => JsValue::from_str("#00ff00"),
                    Health::Infected => JsValue::from_str("#ff0000"),
                    Health::Removed => JsValue::from_str("#0000ff")
                });
            ctx
                .arc(human.pos.x, human.pos.y, human.thickness, 0.0, std::f64::consts::PI * 2.0)
                .unwrap();
            ctx.fill();
        }
    }

    pub fn tick(&mut self) {
        let mut humans = self.humans.clone();

        for i in 0..self.humans.len() {
            humans[i].pos = humans[i].pos + humans[i].velocity;
            for j in (i+1)..humans.len() {
                if humans[i].collide(&humans[j]) {
                    let mut human_i = humans[i].clone();
                    let mut human_j = humans[j].clone();
                    human_i.bounce(&mut human_j);

                    human_i.infect(&mut human_j);

                    humans[i] = human_i;
                    humans[j] = human_j;
                }
            }

            if self.quarantine.inside(&humans[i]) {
                humans[i].bounce_edge(0.0, self.quarantine.x1, 0.0, self.height);
            } else {
                humans[i].bounce_edge(self.quarantine.x2, self.width, 0.0, self.height);
            }
        }

        self.humans = humans;
        self.ticks += 1;
    }
}

mod utils;
mod human;

use wasm_bindgen::prelude::*;

use web_sys::{CanvasRenderingContext2d};

use human::{Human, Vector, Health};

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
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: f64, height: f64, humans: u32) -> Universe {
        let humans = (0..humans).map(|_i| {
            Human::new(
                Vector {
                    x: 15.0 + utils::rand() * (width - 30.0),
                    y: 15.0 + utils::rand() * (height - 30.0),
                },
                Vector::normalize(
                    utils::rand() * 2.0 - 1.0,
                    utils::rand() * 2.0 - 1.0,
                ),
                if _i == 0 { Health::Infected } else { Health::Susceptible },
                10.0
            )
        })
        .collect();

        /*let humans = (0..2).map(|_i| {
            console_log!("X: {}, Velocity: {}", width * (_i as f64), if _i == 0 { 1.0 } else { -1.0 });
            Human {
                pos: Vector {
                    x: 15.0 + (width - 30.0) * (_i as f64),
                    y: height / 2.0,
                },
                velocity: Vector::normalize(
                    if _i == 0 { 1.0 } else { -1.0 },
                    0.0,
                ),
                health: Health::Susceptible,
                thickness: 10.0
            }
        })
        .collect();*/

        Universe {
            width,
            height,
            humans,
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

                    human_i.infect(&mut human_j, self.ticks);

                    humans[i] = human_i;
                    humans[j] = human_j;
                }
            }

            humans[i].bounce_edge(self.width, self.height);
            humans[i].recover_or_die(self.ticks);
        }

        self.humans = humans;
        self.ticks += 1;
    }
}
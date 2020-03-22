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
pub struct AgeGroup {
    /// How many humans should be in this age group?
    size: usize,
    /// How fast should the humans in this group move?
    activity: f64,
    /// How large is the probability to infect in this age group?
    vulnerability: f64,
    /// How large is the probability, that people of this group will die?
    letality: f64,
}

impl AgeGroup {
    pub fn spawn(&self, pos: Vector, health: Health) -> Human {
        Human::new(
            pos,
            Vector::normalize(
                utils::rand() * 2.0 - 1.0,
                utils::rand() * 2.0 - 1.0,
            ).scale(self.activity),
            health,
            7.0,
            self.vulnerability,
            self.letality,
        )
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: f64, height: f64, humans: u32) -> Universe {
        let age_group = AgeGroup {
            size: humans as usize,
            activity: 1.0,
            vulnerability: 1.0,
            letality: 0.08,
        };

        let mut universe = Universe {
            width,
            height,
            humans: Vec::new(),
            ticks: 0
        };

        universe.spawn_age_group(&age_group);

        universe
    }

    pub fn spawn_age_group(&mut self, age_group: &AgeGroup) {
        let mut humans : Vec<Human> = Vec::with_capacity(age_group.size);

        while humans.len() < humans.capacity() {
            let mut human = age_group.spawn(
                Vector {
                    x: 15.0 + utils::rand() * (self.width - 30.0),
                    y: 15.0 + utils::rand() * (self.height - 30.0),
                },
                if humans.len() == 0 { Health::Infected } else { Health::Susceptible }
            );

            // Prevent overlapping in initial configuration
            let mut collision_counter = humans.len();
            while collision_counter != 0 {
                collision_counter = humans.len();
                for i in 0..humans.len() {
                    if human.collide(&humans[i]) {
                        human.pos.x = 15.0 + utils::rand() * (self.width - 30.0);
                        human.pos.y = 15.0 + utils::rand() * (self.height - 30.0);
                    } else {
                        collision_counter -= 1;
                    }
                }
            }
            humans.push(human);
        }

        self.humans.extend(humans.iter().cloned());
    }

    pub fn width(&self) -> f64 {
        self.width
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn susceptible(&self) -> usize {
        self.humans.iter().filter(|h| h.health == Health::Susceptible).count()
    }

    pub fn infected(&self) -> usize {
        self.humans.iter().filter(|h| h.health == Health::Infected).count()
    }

    pub fn removed(&self) -> usize {
        self.humans.iter().filter(|h| h.health == Health::Removed).count()
    }

    pub fn died(&self) -> usize {
        self.humans.iter().filter(|h| h.health == Health::Died).count()
    }

    pub fn render(&self, ctx: &CanvasRenderingContext2d) {
        ctx.clear_rect(0.0, 0.0, self.width.into(), self.height.into());

        for human in self.humans.iter() {
            ctx.begin_path();
            ctx.set_fill_style(&
                match human.health {
                    Health::Susceptible => JsValue::from_str("#68d391"),
                    Health::Infected => JsValue::from_str("#e53e3e"),
                    Health::Removed => JsValue::from_str("#63b3ed"),
                    Health::Died => JsValue::from_str("rgba(0,0,0,.1)"),
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

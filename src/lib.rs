mod utils;

use wasm_bindgen::prelude::*;

use web_sys::{CanvasRenderingContext2d};

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

// Next let's define a macro that's like `println!`, only it works for
// `console.log`. Note that `println!` doesn't actually work on the wasm target
// because the standard library currently just eats all output. To get
// `println!`-like behavior in your app you'll likely want a macro like this.

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
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Health {
    Susceptible = 0,
    Infected = 1,
    Removed = 2,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Human {
    pos: Vector,
    velocity: Vector,
    health: Health,
    thickness: f64,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector {
    x: f64,
    y: f64,
}

impl std::ops::Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Vector {
    fn normalize(x: f64, y: f64) -> Vector {
        let length = (x*x + y*y).sqrt();

        Vector {
            x: x / length,
            y: y / length
        }
    }

    fn dot(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y
    }

    fn scale(&self, scale: f64) -> Vector {
        Vector {
            x: self.x * scale,
            y: self.y * scale,
        }
    }
}

impl Human {
    fn collide(&self, other: &Human) -> bool {
        (self.pos.x - other.pos.x).powi(2) + (self.pos.y - other.pos.y).powi(2) <= (self.thickness + other.thickness).powi(2)
    }

    fn bounce(&mut self, other: &mut Human) {
        let tangent_vector = Vector::normalize(self.pos.y - other.pos.y, -(self.pos.x - other.pos.x));
        //let tangent_vector = Vector::normalize(-(self.pos.y - other.pos.y), self.pos.x - other.pos.x);

        let relative_velocity = self.velocity - other.velocity;

        let length = relative_velocity.dot(&tangent_vector);

        let velocity_component_on_tangent = tangent_vector.scale(length);

        let velocity_component_perpendicular_to_tangent = relative_velocity - velocity_component_on_tangent;

        self.velocity = self.velocity - velocity_component_perpendicular_to_tangent;
        other.velocity = other.velocity + velocity_component_perpendicular_to_tangent;
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: f64, height: f64) -> Universe {
        let humans = (0..10).map(|_i| {
            Human {
                pos: Vector {
                    x: js_sys::Math::random() * (width - 30.0),
                    y: js_sys::Math::random() * (height - 30.0),
                },
                velocity: Vector::normalize(
                    js_sys::Math::random() * 2.0 - 1.0,
                    js_sys::Math::random() * 2.0 - 1.0,
                ),
                health: Health::Susceptible,
                thickness: 10.0
            }
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
            humans
        }
    }

    pub fn width(&self) -> f64 {
        self.width
    }

    pub fn height(&self) -> f64 {
        self.height
    }

    /*pub fn render(&self) -> String {
        self.to_string()
    }*/

    pub fn render(&self, ctx: &CanvasRenderingContext2d) {

        ctx.clear_rect(0.0, 0.0, self.width.into(), self.height.into());

        for human in self.humans.iter() {
            ctx.begin_path();
            ctx
                .arc(human.pos.x, human.pos.y, human.thickness, 0.0, std::f64::consts::PI * 2.0)
                .unwrap();
            ctx.stroke();
        }
    }

    pub fn tick(&mut self) {
        let mut humans = self.humans.clone();

        for (i, _) in self.humans.iter().enumerate() {
            humans[i].pos = humans[i].pos + humans[i].velocity;
            for j in (i+1)..humans.len() {
                if humans[i].collide(&humans[j]) {
                    let mut human_i = humans[i].clone();
                    let mut human_j = humans[j].clone();
                    human_i.bounce(&mut human_j);
                    humans[i] = human_i;
                    humans[j] = human_j;
                    //log("Collides!");
                }
            }

            if humans[i].pos.x - humans[i].thickness <= 0.0 || humans[i].pos.x + humans[i].thickness >= self.width {
                humans[i].velocity.x *= -1.0;
            }

            if humans[i].pos.y - humans[i].thickness <= 0.0 || humans[i].pos.y + humans[i].thickness >= self.height {
                humans[i].velocity.y *= -1.0;
            }
        }

        self.humans = humans;
    }
}

/*impl std::fmt::Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}*/
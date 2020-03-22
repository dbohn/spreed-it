use wasm_bindgen::prelude::*;

use crate::human::{Human, Vector};

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Zone {
    pub x1: f64,
    pub x2: f64,
}

impl Zone {
    pub fn inside(& self, human: &Human) -> bool {
        human.pos.x - human.thickness < self.x1
    }
}

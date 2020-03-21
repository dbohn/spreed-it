use wasm_bindgen::prelude::*;

use crate::human::{Human, Vector};

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Zone {
    pub x1: f64,
    pub x2: f64,
/*
    pub top_left_corner: Vector,
    pub width: f64,
    pub height: f64,
*/
}

impl Zone {
    pub fn inside(& self, human: &Human) -> bool {
        human.pos.x - human.thickness < self.x1
    /*
        human.pos.x - human.thickness >= self.top_left_corner.x &&
        human.pos.x + human.thickness <= self.top_left_corner.x + self.width &&
        human.pos.y - human.thickness >= self.top_left_corner.y &&
        human.pos.y + human.thickness <= self.top_left_corner.y + self.height
    */
    }
}

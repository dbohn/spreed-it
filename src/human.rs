use crate::utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Health {
    Susceptible = 0,
    Infected = 1,
    Removed = 2,
}

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct Human {
    pub pos: Vector,
    pub velocity: Vector,
    pub health: Health,
    pub thickness: f64,
    infected_at: u128,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
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
    pub fn normalize(x: f64, y: f64) -> Vector {
        let length = (x*x + y*y).sqrt();

        Vector {
            x: x / length,
            y: y / length,
        }
    }

    pub fn dot(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn scale(&self, scale: f64) -> Vector {
        Vector {
            x: self.x * scale,
            y: self.y * scale,
        }
    }
}

impl Human {

    pub fn new(pos: Vector, velocity: Vector, health: Health, thickness: f64) -> Human {
        Human {
            pos,
            velocity,
            health,
            thickness,
            infected_at: 0,
        }
    }

    pub fn collide(&self, other: &Human) -> bool {
        (self.pos.x - other.pos.x).powi(2) + (self.pos.y - other.pos.y).powi(2) <= (self.thickness + other.thickness).powi(2)
    }

    pub fn bounce(&mut self, other: &mut Human) {
        let tangent_vector = Vector::normalize(self.pos.y - other.pos.y, -(self.pos.x - other.pos.x));

        let relative_velocity = self.velocity - other.velocity;

        let length = relative_velocity.dot(&tangent_vector);

        let velocity_component_on_tangent = tangent_vector.scale(length);

        let velocity_component_perpendicular_to_tangent = relative_velocity - velocity_component_on_tangent;

        self.velocity = self.velocity - velocity_component_perpendicular_to_tangent;
        other.velocity = other.velocity + velocity_component_perpendicular_to_tangent;
    }

    pub fn infect(&mut self, other: &mut Human, now: u128) {

        if self.health == Health::Removed {
            return
        }

        if self.health == Health::Infected {
            other.health = Health::Infected;
            other.infected_at = now;
        }

        if other.health == Health::Infected {
            self.health = Health::Infected;
            self.infected_at = now;
        }
    }

    pub fn bounce_edge(&mut self, width: f64, height: f64) {
        if self.pos.x - self.thickness <= 0.0 || self.pos.x + self.thickness >= width {
            self.velocity.x *= -1.0;
        }

        if self.pos.y - self.thickness <= 0.0 || self.pos.y + self.thickness >= height {
            self.velocity.y *= -1.0;
        }
    }

    pub fn recover_or_die(&mut self, now: u128) {
        // We have a maximum infect length of 14 days
        // during this time, we have an increasing probability of recover
        if !self.is_infected() {
            return;
        }

        let seconds = (now as f64) / 60.0;

        let coefficient = seconds * 0.5 - 12.0;

        if utils::rand() < coefficient.tanh() * 0.5 + 0.5 {
            self.health = Health::Removed;
        }
    }

    pub fn is_infected(&self) -> bool {
        self.health == Health::Infected
    }
}
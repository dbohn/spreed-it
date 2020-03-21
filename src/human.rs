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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Human {
    pub pos: Vector,
    pub velocity: Vector,
    pub health: Health,
    pub thickness: f64,
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
            y: y / length
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

    pub fn infect(&mut self, other: &mut Human) {
        if self.health == Health::Infected {
            other.health = Health::Infected;
        }

        if other.health == Health::Infected {
            self.health = Health::Infected;
        }
    }

    pub fn bounce_edge(&mut self, lower_x: f64, upper_x: f64, lower_y: f64, upper_y: f64) {

        /* push human back into the allowed area if necessary */
        if self.pos.x - self.thickness < lower_x {
            self.pos.x = lower_x + self.thickness;
        }

        if self.pos.x + self.thickness > upper_x {
            self.pos.x = upper_x - self.thickness;
        }

        if self.pos.y - self.thickness < lower_y {
            self.pos.y = lower_y + self.thickness;
        }

        if self.pos.y + self.thickness > upper_y {
            self.pos.y = upper_y - self.thickness;
        }

        if self.pos.x - self.thickness <= lower_x || self.pos.x + self.thickness >= upper_x {
            self.velocity.x *= -1.0;
        }

        if self.pos.y - self.thickness <= lower_y || self.pos.y + self.thickness >= upper_y {
            self.velocity.y *= -1.0;
        }
    }
}

mod temperature_conversions;
use crate::temperature::temperature_conversions::{*};

pub enum Unit {
    FAHRENEHIT,
    CELSIUS,
    KELVIN,
}

#[derive(Default, Debug)]
pub struct Temperature {
    pub fahrenheit: f32,
    pub celsius: f32,
    pub kelvin: f32,
}

impl Temperature {
    pub fn new() -> Self {
        Self {
            fahrenheit: Default::default(),
            celsius: Default::default(),
            kelvin: Default::default(),
        }
    }

    pub fn from_fahrenheit(mut self, temp: f32) -> Self {
        self.fahrenheit = temp;
        self.celsius = f_to_c(temp);
        self.kelvin = f_to_k(temp);
        self
    }

    pub fn from_celsius(mut self, temp: f32) -> Self {
        self.celsius = temp;
        self.fahrenheit = c_to_f(temp);
        self.kelvin = c_to_k(temp);
        self
    }

    pub fn from_kelvin(mut self, temp: f32) -> Self {
        self.kelvin = temp;
        self.fahrenheit = k_to_f(temp);
        self.celsius = k_to_c(temp);
        self
    }
}

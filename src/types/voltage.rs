use core::{marker::PhantomData, ops::{Deref, Mul}};

use crate::types::sensor_type::SensorType;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Voltage<T: SensorType> {
    voltage: f64,
    sensor_type: PhantomData<T>,
}

impl<T: SensorType> Voltage<T> {
    pub fn new(voltage: f64) -> Self {
        Self {
            voltage,
            sensor_type: PhantomData,
        }
    }

    pub fn voltage(&self) -> f64 {
        self.voltage
    }
}

impl<T: SensorType> Deref for Voltage<T> {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.voltage
    }
}

impl<T: SensorType> Mul<f64> for &Voltage<T> {
    type Output = Voltage<T>;

    fn mul(self, rhs: f64) -> Self::Output {
        Voltage::new(self.voltage() * rhs)
    }
}

impl<T: SensorType> Mul<f64> for Voltage<T> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        &self * rhs
    }
}

impl<T: SensorType> Mul<&Voltage<T>> for f64 {
    type Output = Voltage<T>;

    fn mul(self, rhs: &Voltage<T>) -> Self::Output {
        rhs * self
    }
}

impl<T: SensorType> Mul<Voltage<T>> for f64 {
    type Output = Voltage<T>;

    fn mul(self, rhs: Voltage<T>) -> Self::Output {
        rhs * self
    }
}

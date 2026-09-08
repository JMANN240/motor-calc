use core::{f64::consts::TAU, ops::Mul};

use crate::types::{sensor_type::SensorType, voltage::Voltage};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SensorParameters<T: SensorType> {
    voltage_scale: f64,
    voltage_offset: Voltage<T>,
}

impl<T: SensorType> SensorParameters<T> {
    pub fn new(voltage_scale: f64, voltage_offset: Voltage<T>) -> Self {
        Self {
            voltage_scale,
            voltage_offset,
        }
    }

    pub fn voltage_scale(&self) -> f64 {
        self.voltage_scale
    }

    pub fn voltage_offset(&self) -> &Voltage<T> {
        &self.voltage_offset
    }
}

impl<T: SensorType> Default for SensorParameters<T> {
    fn default() -> Self {
        Self::new(48.0 / TAU, Voltage::new(2.5))
    }
}

impl<T: SensorType> Mul<f64> for &SensorParameters<T> {
    type Output = SensorParameters<T>;

    fn mul(self, rhs: f64) -> Self::Output {
        SensorParameters::new(self.voltage_scale() * rhs, self.voltage_offset() * rhs)
    }
}

impl<T: SensorType> Mul<f64> for SensorParameters<T> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        &self * rhs
    }
}

impl<T: SensorType> Mul<&SensorParameters<T>> for f64 {
    type Output = SensorParameters<T>;

    fn mul(self, rhs: &SensorParameters<T>) -> Self::Output {
        rhs * self
    }
}

impl<T: SensorType> Mul<SensorParameters<T>> for f64 {
    type Output = SensorParameters<T>;

    fn mul(self, rhs: SensorParameters<T>) -> Self::Output {
        rhs * self
    }
}

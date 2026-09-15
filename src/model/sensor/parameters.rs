use core::{
    f64::consts::TAU,
    ops::{Add, Div, Mul, Neg, Sub},
};

use motor_calc_core::parameters::Parameters;

use crate::{
    model::Adjustable,
    types::{
        sensor_type::{Alpha, Beta, SensorType},
        voltage::Voltage,
    },
};

#[repr(C)]
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

impl Adjustable for SensorParameters<Alpha> {
    fn adjusted(&self, gradient: &Parameters) -> Self {
        Self::new(
            self.voltage_scale() + gradient.alpha_voltage_scale(),
            self.voltage_offset() + Voltage::from_volts_f64(gradient.alpha_voltage_offset()),
        )
    }
}

impl Adjustable for SensorParameters<Beta> {
    fn adjusted(&self, gradient: &Parameters) -> Self {
        Self::new(
            self.voltage_scale() + gradient.beta_voltage_scale(),
            self.voltage_offset() + Voltage::from_volts_f64(gradient.beta_voltage_offset()),
        )
    }
}

impl<T: SensorType> Default for SensorParameters<T> {
    fn default() -> Self {
        Self::new(48.0 / TAU, Voltage::from_volts_f64(2.5))
    }
}

impl<T: SensorType> Add for SensorParameters<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.voltage_scale() + rhs.voltage_scale(),
            self.voltage_offset() + rhs.voltage_offset(),
        )
    }
}

impl<T: SensorType> Sub for SensorParameters<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.voltage_scale() - rhs.voltage_scale(),
            self.voltage_offset() - rhs.voltage_offset(),
        )
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

impl<T: SensorType> Div<f64> for &SensorParameters<T> {
    type Output = SensorParameters<T>;

    fn div(self, rhs: f64) -> Self::Output {
        SensorParameters::new(self.voltage_scale() / rhs, self.voltage_offset() / rhs)
    }
}

impl<T: SensorType> Div<f64> for SensorParameters<T> {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        &self / rhs
    }
}

impl<T: SensorType> Neg for SensorParameters<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.voltage_scale(), -self.voltage_offset())
    }
}

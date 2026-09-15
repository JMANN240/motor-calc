use core::ops::{Add, Div, Mul, Neg, Sub};

use motor_calc_core::parameters::Parameters;
use num_traits::{Float, FloatConst};

use crate::{
    model::Adjustable,
    types::{
        sensor_type::{Alpha, Beta, SensorType},
        voltage::Voltage,
    },
};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SensorParameters<F: Float, T: SensorType> {
    voltage_scale: F,
    voltage_offset: Voltage<F, T>,
}

impl<F: Float, T: SensorType> SensorParameters<F, T> {
    pub fn new(voltage_scale: F, voltage_offset: Voltage<F, T>) -> Self {
        Self {
            voltage_scale,
            voltage_offset,
        }
    }

    pub fn voltage_scale(self) -> F {
        self.voltage_scale
    }

    pub fn voltage_offset(self) -> Voltage<F, T> {
        self.voltage_offset
    }
}

impl<F: Float> Adjustable<F> for SensorParameters<F, Alpha> {
    fn adjusted(&self, gradient: Parameters<F>) -> Self {
        Self::new(
            self.voltage_scale() + gradient.alpha_voltage_scale(),
            self.voltage_offset() + Voltage::from_volts_f(gradient.alpha_voltage_offset()),
        )
    }
}

impl<F: Float> Adjustable<F> for SensorParameters<F, Beta> {
    fn adjusted(&self, gradient: Parameters<F>) -> Self {
        Self::new(
            self.voltage_scale() + gradient.beta_voltage_scale(),
            self.voltage_offset() + Voltage::from_volts_f(gradient.beta_voltage_offset()),
        )
    }
}

impl<F: Float + FloatConst, T: SensorType> Default for SensorParameters<F, T> {
    fn default() -> Self {
        Self::new(
            F::from(48).expect("48 can always be represented with a float") / F::TAU(),
            Voltage::from_volts_f(
                F::from(2.5).expect("2.5 can always be represented with a float"),
            ),
        )
    }
}

impl<F: Float, T: SensorType> Add for SensorParameters<F, T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.voltage_scale() + rhs.voltage_scale(),
            self.voltage_offset() + rhs.voltage_offset(),
        )
    }
}

impl<F: Float, T: SensorType> Sub for SensorParameters<F, T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.voltage_scale() - rhs.voltage_scale(),
            self.voltage_offset() - rhs.voltage_offset(),
        )
    }
}

impl<F: Float, T: SensorType> Mul<F> for SensorParameters<F, T> {
    type Output = Self;

    fn mul(self, rhs: F) -> Self::Output {
        Self::new(
            self.voltage_scale() * rhs,
            self.voltage_offset() * rhs,
        )
    }
}

impl<F: Float, T: SensorType> Div<F> for SensorParameters<F, T> {
    type Output = Self;

    fn div(self, rhs: F) -> Self::Output {
        Self::new(
            self.voltage_scale() / rhs,
            self.voltage_offset() / rhs,
        )
    }
}

impl<F: Float, T: SensorType> Neg for SensorParameters<F, T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.voltage_scale(), -self.voltage_offset())
    }
}

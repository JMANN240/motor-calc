use core::ops::{Add, Div, Mul, Neg, Sub};

use motor_calc_core::parameters::Parameters;

use crate::model::Adjustable;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MotorParameters {
    distance_ratio: f64,
}

impl MotorParameters {
    pub fn new(distance_ratio: f64) -> Self {
        Self { distance_ratio }
    }

    pub fn distance_ratio(&self) -> f64 {
        self.distance_ratio
    }
}

impl Adjustable for MotorParameters {
    fn adjusted(&self, gradient: &Parameters) -> Self {
        Self::new(self.distance_ratio() + gradient.distance_ratio())
    }
}

impl Add for MotorParameters {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.distance_ratio() + rhs.distance_ratio())
    }
}

impl Sub for MotorParameters {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.distance_ratio() - rhs.distance_ratio())
    }
}

impl Mul<f64> for &MotorParameters {
    type Output = MotorParameters;

    fn mul(self, rhs: f64) -> Self::Output {
        MotorParameters::new(self.distance_ratio() * rhs)
    }
}

impl Mul<f64> for MotorParameters {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        &self * rhs
    }
}

impl Mul<&MotorParameters> for f64 {
    type Output = MotorParameters;

    fn mul(self, rhs: &MotorParameters) -> Self::Output {
        rhs * self
    }
}

impl Mul<MotorParameters> for f64 {
    type Output = MotorParameters;

    fn mul(self, rhs: MotorParameters) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for &MotorParameters {
    type Output = MotorParameters;

    fn div(self, rhs: f64) -> Self::Output {
        MotorParameters::new(self.distance_ratio() / rhs)
    }
}

impl Div<f64> for MotorParameters {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        &self / rhs
    }
}

impl Neg for MotorParameters {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.distance_ratio())
    }
}

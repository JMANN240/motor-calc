use core::ops::{Add, Div, Mul, Neg, Sub};

use motor_calc_core::parameters::Parameters;
use num_traits::Float;

use crate::model::Adjustable;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MotorParameters<F: Float> {
    distance_ratio: F,
}

impl<F: Float> MotorParameters<F> {
    pub fn new(distance_ratio: F) -> Self {
        Self { distance_ratio }
    }

    pub fn distance_ratio(self) -> F {
        self.distance_ratio
    }
}

impl<F: Float> Adjustable<F> for MotorParameters<F> {
    fn adjusted(&self, gradient: Parameters<F>) -> Self {
        Self::new(self.distance_ratio() + gradient.distance_ratio())
    }
}

impl<F: Float> Default for MotorParameters<F> {
    fn default() -> Self {
        Self::new(F::from(6.75).expect("6.75 can always be represented with a float"))
    }
}

impl<F: Float> Add for MotorParameters<F> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.distance_ratio() + rhs.distance_ratio())
    }
}

impl<F: Float> Sub for MotorParameters<F> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.distance_ratio() - rhs.distance_ratio())
    }
}

impl<F: Float> Mul<F> for MotorParameters<F> {
    type Output = Self;

    fn mul(self, rhs: F) -> Self::Output {
        Self::new(self.distance_ratio() * rhs)
    }
}

impl<F: Float> Div<F> for MotorParameters<F> {
    type Output = Self;

    fn div(self, rhs: F) -> Self::Output {
        Self::new(self.distance_ratio() / rhs)
    }
}

impl<F: Float> Neg for MotorParameters<F> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.distance_ratio())
    }
}

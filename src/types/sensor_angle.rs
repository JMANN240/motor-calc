use core::{
    marker::PhantomData,
    ops::{Add, Deref, Div, Mul, Neg, Sub},
};

use num_traits::Float;

use crate::types::sensor_type::SensorType;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SensorAngle<F: Float, T: SensorType> {
    radians: F,
    sensor_type: PhantomData<T>,
}

impl<F: Float, T: SensorType> SensorAngle<F, T> {
    pub fn from_radians_f(radians: F) -> Self {
        Self {
            radians,
            sensor_type: PhantomData,
        }
    }

    pub fn from_milliradians_u32(milliradians: u32) -> Option<Self> {
        F::from(milliradians).map(|milliradians_f| {
            Self::from_radians_f(
                milliradians_f
                    / F::from(1000).expect("1000 can always be represented with a float"),
            )
        })
    }

    pub fn radians(self) -> F {
        self.radians
    }
}

impl<F: Float, T: SensorType> Deref for SensorAngle<F, T> {
    type Target = F;

    fn deref(&self) -> &Self::Target {
        &self.radians
    }
}

impl<F: Float, T: SensorType> Add for SensorAngle<F, T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_radians_f(self.radians() + rhs.radians())
    }
}

impl<F: Float, T: SensorType> Sub for SensorAngle<F, T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_radians_f(self.radians() - rhs.radians())
    }
}

impl<F: Float, T: SensorType> Mul<F> for SensorAngle<F, T> {
    type Output = Self;

    fn mul(self, rhs: F) -> Self::Output {
        Self::from_radians_f(self.radians() * rhs)
    }
}

impl<F: Float, T: SensorType> Div<F> for SensorAngle<F, T> {
    type Output = Self;

    fn div(self, rhs: F) -> Self::Output {
        Self::from_radians_f(self.radians() / rhs)
    }
}

impl<F: Float, T: SensorType> Neg for SensorAngle<F, T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::from_radians_f(-self.radians())
    }
}

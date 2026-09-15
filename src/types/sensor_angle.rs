use core::{
    marker::PhantomData,
    ops::{Add, Deref, Div, Mul, Neg, Sub},
};

use crate::types::sensor_type::SensorType;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SensorAngle<T: SensorType> {
    radians: f64,
    sensor_type: PhantomData<T>,
}

impl<T: SensorType> SensorAngle<T> {
    pub fn from_radians_f64(radians: f64) -> Self {
        Self {
            radians,
            sensor_type: PhantomData,
        }
    }

    pub fn from_milliradians_u32(milliradians: u32) -> Self {
        Self::from_radians_f64(milliradians as f64 / 1000.0)
    }

    pub fn radians(&self) -> f64 {
        self.radians
    }
}

impl<T: SensorType> Deref for SensorAngle<T> {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.radians
    }
}

impl<T: SensorType> Add for SensorAngle<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_radians_f64(self.radians() + rhs.radians())
    }
}

impl<T: SensorType> Sub for SensorAngle<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_radians_f64(self.radians() - rhs.radians())
    }
}

impl<T: SensorType> Mul<f64> for &SensorAngle<T> {
    type Output = SensorAngle<T>;

    fn mul(self, rhs: f64) -> Self::Output {
        SensorAngle::from_radians_f64(self.radians() * rhs)
    }
}

impl<T: SensorType> Mul<f64> for SensorAngle<T> {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        &self * rhs
    }
}

impl<T: SensorType> Mul<&SensorAngle<T>> for f64 {
    type Output = SensorAngle<T>;

    fn mul(self, rhs: &SensorAngle<T>) -> Self::Output {
        rhs * self
    }
}

impl<T: SensorType> Mul<SensorAngle<T>> for f64 {
    type Output = SensorAngle<T>;

    fn mul(self, rhs: SensorAngle<T>) -> Self::Output {
        rhs * self
    }
}

impl<T: SensorType> Div<f64> for &SensorAngle<T> {
    type Output = SensorAngle<T>;

    fn div(self, rhs: f64) -> Self::Output {
        SensorAngle::from_radians_f64(self.radians() / rhs)
    }
}

impl<T: SensorType> Div<f64> for SensorAngle<T> {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        &self / rhs
    }
}

impl<T: SensorType> Neg for &SensorAngle<T> {
    type Output = SensorAngle<T>;

    fn neg(self) -> Self::Output {
        SensorAngle::from_radians_f64(-self.radians())
    }
}

impl<T: SensorType> Neg for SensorAngle<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        -&self
    }
}

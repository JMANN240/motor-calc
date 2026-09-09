use core::{
    marker::PhantomData, ops::{Add, Deref, Div, Mul, Neg, Sub},
};

use crate::types::sensor_type::SensorType;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SensorAngle<T: SensorType> {
    angle: f64,
    sensor_type: PhantomData<T>,
}

impl<T: SensorType> SensorAngle<T> {
    pub fn new(angle: f64) -> Self {
        Self {
            angle,
            sensor_type: PhantomData,
        }
    }

    pub fn angle(&self) -> f64 {
        self.angle
    }
}

impl<T: SensorType> Deref for SensorAngle<T> {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.angle
    }
}

impl<T: SensorType> Add for SensorAngle<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.angle() + rhs.angle())
    }
}

impl<T: SensorType> Sub for SensorAngle<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.angle() - rhs.angle())
    }
}

impl<T: SensorType> Mul<f64> for &SensorAngle<T> {
    type Output = SensorAngle<T>;

    fn mul(self, rhs: f64) -> Self::Output {
        SensorAngle::new(self.angle() * rhs)
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
        SensorAngle::new(self.angle() / rhs)
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
        SensorAngle::new(-self.angle())
    }
}

impl<T: SensorType> Neg for SensorAngle<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        -&self
    }
}

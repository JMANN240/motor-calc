use core::{marker::PhantomData, ops::{Add, Deref, Div, Mul, Neg, Sub}};

use crate::types::sensor_type::SensorType;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Voltage<T: SensorType> {
    voltage: f64,
    sensor_type: PhantomData<T>,
}

impl<T: SensorType> Voltage<T> {
    pub fn from_volts_f64(voltage: f64) -> Self {
        Self {
            voltage,
            sensor_type: PhantomData,
        }
    }

    pub fn from_millivolts_u32(millivolts: u32) -> Self {
        Self::from_volts_f64(millivolts as f64 / 1000.0)
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

impl<T: SensorType> Add for Voltage<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl<T: SensorType> Add<&Self> for Voltage<T> {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        &self + rhs
    }
}

impl<T: SensorType> Add<Voltage<T>> for &Voltage<T> {
    type Output = Voltage<T>;

    fn add(self, rhs: Voltage<T>) -> Self::Output {
        self + &rhs
    }
}

impl<T: SensorType> Add for &Voltage<T> {
    type Output = Voltage<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Voltage::from_volts_f64(self.voltage() + rhs.voltage())
    }
}

impl<T: SensorType> Sub for Voltage<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}

impl<T: SensorType> Sub<&Self> for Voltage<T> {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        &self - rhs
    }
}

impl<T: SensorType> Sub<Voltage<T>> for &Voltage<T> {
    type Output = Voltage<T>;

    fn sub(self, rhs: Voltage<T>) -> Self::Output {
        self - &rhs
    }
}

impl<T: SensorType> Sub for &Voltage<T> {
    type Output = Voltage<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Voltage::from_volts_f64(self.voltage() - rhs.voltage())
    }
}

impl<T: SensorType> Mul<f64> for &Voltage<T> {
    type Output = Voltage<T>;

    fn mul(self, rhs: f64) -> Self::Output {
        Voltage::from_volts_f64(self.voltage() * rhs)
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

impl<T: SensorType> Div<f64> for &Voltage<T> {
    type Output = Voltage<T>;

    fn div(self, rhs: f64) -> Self::Output {
        Voltage::from_volts_f64(self.voltage() / rhs)
    }
}

impl<T: SensorType> Div<f64> for Voltage<T> {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        &self / rhs
    }
}

impl<T: SensorType> Neg for &Voltage<T> {
    type Output = Voltage<T>;

    fn neg(self) -> Self::Output {
        Voltage::from_volts_f64(-self.voltage())
    }
}

impl<T: SensorType> Neg for Voltage<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        -&self
    }
}

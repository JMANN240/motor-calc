use core::ops::{Add, Div, Mul, Neg, Sub};

use crate::types::shaft_angle::ShaftAngle;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MountedSensorParameters {
    shaft_angle_offset: ShaftAngle,
}

impl MountedSensorParameters {
    pub fn new(shaft_angle_offset: ShaftAngle) -> Self {
        Self { shaft_angle_offset }
    }

    pub fn shaft_angle_offset(&self) -> ShaftAngle {
        self.shaft_angle_offset
    }
}

impl Add for MountedSensorParameters {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.shaft_angle_offset() + rhs.shaft_angle_offset())
    }
}

impl Sub for &MountedSensorParameters {
    type Output = MountedSensorParameters;

    fn sub(self, rhs: Self) -> Self::Output {
        MountedSensorParameters::new(self.shaft_angle_offset() - rhs.shaft_angle_offset())
    }
}

impl Sub for MountedSensorParameters {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}

impl Sub<&Self> for MountedSensorParameters {
    type Output = Self;

    fn sub(self, rhs: &Self) -> Self::Output {
        &self - rhs
    }
}

impl Sub<MountedSensorParameters> for &MountedSensorParameters {
    type Output = MountedSensorParameters;

    fn sub(self, rhs: MountedSensorParameters) -> Self::Output {
        self - &rhs
    }
}

impl Mul<f64> for &MountedSensorParameters {
    type Output = MountedSensorParameters;

    fn mul(self, rhs: f64) -> Self::Output {
        MountedSensorParameters::new(self.shaft_angle_offset() * rhs)
    }
}

impl Mul<f64> for MountedSensorParameters {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        &self * rhs
    }
}

impl Mul<&MountedSensorParameters> for f64 {
    type Output = MountedSensorParameters;

    fn mul(self, rhs: &MountedSensorParameters) -> Self::Output {
        rhs * self
    }
}

impl Mul<MountedSensorParameters> for f64 {
    type Output = MountedSensorParameters;

    fn mul(self, rhs: MountedSensorParameters) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for &MountedSensorParameters {
    type Output = MountedSensorParameters;

    fn div(self, rhs: f64) -> Self::Output {
        MountedSensorParameters::new(self.shaft_angle_offset() / rhs)
    }
}

impl Div<f64> for MountedSensorParameters {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        &self / rhs
    }
}

impl Neg for MountedSensorParameters {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.shaft_angle_offset())
    }
}

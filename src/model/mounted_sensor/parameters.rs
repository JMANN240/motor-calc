use core::ops::Mul;

use crate::types::shaft_angle::ShaftAngle;

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

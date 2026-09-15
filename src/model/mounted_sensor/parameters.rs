use core::ops::{Add, Div, Mul, Neg, Sub};

use num_traits::Float;

use crate::types::shaft_angle::ShaftAngle;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MountedSensorParameters<F: Float> {
    shaft_angle_offset: ShaftAngle<F>,
}

impl<F: Float> MountedSensorParameters<F> {
    pub fn new(shaft_angle_offset: ShaftAngle<F>) -> Self {
        Self { shaft_angle_offset }
    }

    pub fn shaft_angle_offset(self) -> ShaftAngle<F> {
        self.shaft_angle_offset
    }
}

impl<F: Float> Add for MountedSensorParameters<F> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.shaft_angle_offset() + rhs.shaft_angle_offset())
    }
}

impl<F: Float> Sub for MountedSensorParameters<F> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.shaft_angle_offset() - rhs.shaft_angle_offset())
    }
}

impl<F: Float> Mul<F> for MountedSensorParameters<F> {
    type Output = Self;

    fn mul(self, rhs: F) -> Self::Output {
        Self::new(self.shaft_angle_offset() * rhs)
    }
}

impl<F: Float> Div<F> for MountedSensorParameters<F> {
    type Output = Self;

    fn div(self, rhs: F) -> Self::Output {
        Self::new(self.shaft_angle_offset() / rhs)
    }
}

impl<F: Float> Neg for MountedSensorParameters<F> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.shaft_angle_offset())
    }
}

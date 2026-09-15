use core::{f64::consts::TAU, ops::Mul};

use motor_calc_core::parameters::Parameters;

use crate::{
    model::{Adjustable, mounted_sensor::parameters::MountedSensorParameters},
    types::shaft_angle::ShaftAngle,
};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AssemblyParameters {
    mounted_alpha_sensor_parameters: MountedSensorParameters,
    mounted_beta_sensor_parameters: MountedSensorParameters,
}

impl AssemblyParameters {
    pub fn new(
        mounted_alpha_sensor_parameters: MountedSensorParameters,
        mounted_beta_sensor_parameters: MountedSensorParameters,
    ) -> Self {
        Self {
            mounted_alpha_sensor_parameters,
            mounted_beta_sensor_parameters,
        }
    }

    pub fn mounted_alpha_sensor_parameters(&self) -> MountedSensorParameters {
        self.mounted_alpha_sensor_parameters
    }

    pub fn mounted_beta_sensor_parameters(&self) -> MountedSensorParameters {
        self.mounted_beta_sensor_parameters
    }
}

impl Adjustable for AssemblyParameters {
    fn adjusted(&self, gradient: &Parameters) -> Self {
        Self::new(
            self.mounted_alpha_sensor_parameters(),
            MountedSensorParameters::new(
                self.mounted_beta_sensor_parameters().shaft_angle_offset()
                    + ShaftAngle::from_radians_f64(gradient.beta_shaft_angle_offset()),
            ),
        )
    }
}

impl Default for AssemblyParameters {
    fn default() -> Self {
        Self::new(
            MountedSensorParameters::new(ShaftAngle::from_radians_f64(0.0)),
            MountedSensorParameters::new(ShaftAngle::from_radians_f64(2.0 * TAU / 7.0)),
        )
    }
}

impl Mul<f64> for &AssemblyParameters {
    type Output = AssemblyParameters;

    fn mul(self, rhs: f64) -> Self::Output {
        AssemblyParameters::new(
            self.mounted_alpha_sensor_parameters() * rhs,
            self.mounted_beta_sensor_parameters() * rhs,
        )
    }
}

impl Mul<f64> for AssemblyParameters {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        &self * rhs
    }
}

impl Mul<&AssemblyParameters> for f64 {
    type Output = AssemblyParameters;

    fn mul(self, rhs: &AssemblyParameters) -> Self::Output {
        rhs * self
    }
}

impl Mul<AssemblyParameters> for f64 {
    type Output = AssemblyParameters;

    fn mul(self, rhs: AssemblyParameters) -> Self::Output {
        rhs * self
    }
}

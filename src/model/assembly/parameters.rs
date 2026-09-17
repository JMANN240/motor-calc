use core::ops::Mul;

use motor_calc_core::parameters::Parameters;
use num_traits::{Float, FloatConst};

use crate::{
    model::{Adjustable, mounted_sensor::parameters::MountedSensorParameters},
    types::shaft_angle::ShaftAngle,
};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AssemblyParameters<F: Float> {
    mounted_alpha_sensor_parameters: MountedSensorParameters<F>,
    mounted_beta_sensor_parameters: MountedSensorParameters<F>,
}

impl<F: Float> AssemblyParameters<F> {
    pub fn new(
        mounted_alpha_sensor_parameters: MountedSensorParameters<F>,
        mounted_beta_sensor_parameters: MountedSensorParameters<F>,
    ) -> Self {
        Self {
            mounted_alpha_sensor_parameters,
            mounted_beta_sensor_parameters,
        }
    }

    pub fn mounted_alpha_sensor_parameters(self) -> MountedSensorParameters<F> {
        self.mounted_alpha_sensor_parameters
    }

    pub fn mounted_alpha_sensor_parameters_mut(&mut self) -> &mut MountedSensorParameters<F> {
        &mut self.mounted_alpha_sensor_parameters
    }

    pub fn mounted_beta_sensor_parameters(self) -> MountedSensorParameters<F> {
        self.mounted_beta_sensor_parameters
    }

    pub fn mounted_beta_sensor_parameters_mut(&mut self) -> &mut MountedSensorParameters<F> {
        &mut self.mounted_beta_sensor_parameters
    }
}

impl<F: Float> Adjustable<F> for AssemblyParameters<F> {
    fn adjusted(&self, gradient: Parameters<F>) -> Self {
        Self::new(
            self.mounted_alpha_sensor_parameters(),
            MountedSensorParameters::new(
                self.mounted_beta_sensor_parameters().shaft_angle_offset()
                    + ShaftAngle::from_radians_f(gradient.beta_shaft_angle_offset()),
            ),
        )
    }
}

impl<F: Float + FloatConst> Default for AssemblyParameters<F> {
    fn default() -> Self {
        Self::new(
            MountedSensorParameters::new(ShaftAngle::from_radians_f(F::zero())),
            MountedSensorParameters::new(ShaftAngle::from_radians_f(
                F::from(2).expect("2 can always be represented with a float") * F::TAU()
                    / F::from(7).expect("7 can always be represented with a float"),
            )),
        )
    }
}

impl<F: Float> Mul<F> for AssemblyParameters<F> {
    type Output = Self;

    fn mul(self, rhs: F) -> Self::Output {
        AssemblyParameters::new(
            self.mounted_alpha_sensor_parameters() * rhs,
            self.mounted_beta_sensor_parameters() * rhs,
        )
    }
}

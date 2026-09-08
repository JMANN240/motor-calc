use core::f64::consts::TAU;

use crate::{
    model::mounted_sensor::parameters::MountedSensorParameters, types::shaft_angle::ShaftAngle,
};

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

impl Default for AssemblyParameters {
    fn default() -> Self {
        Self::new(
            MountedSensorParameters::new(ShaftAngle::new(0.0)),
            MountedSensorParameters::new(ShaftAngle::new(2.0 * TAU / 7.0)),
        )
    }
}

use core::f64::consts::TAU;

use crate::types::shaft_angle::ShaftAngle;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AssemblyParameters {
    alpha_sensor_shaft_angle_offset: ShaftAngle,
    beta_sensor_shaft_angle_offset: ShaftAngle,
}

impl AssemblyParameters {
    pub fn new(
        alpha_sensor_shaft_angle_offset: ShaftAngle,
        beta_sensor_shaft_angle_offset: ShaftAngle,
    ) -> Self {
        Self {
            alpha_sensor_shaft_angle_offset,
            beta_sensor_shaft_angle_offset,
        }
    }

    pub fn alpha_sensor_shaft_angle_offset(&self) -> ShaftAngle {
        self.alpha_sensor_shaft_angle_offset
    }

    pub fn beta_sensor_shaft_angle_offset(&self) -> ShaftAngle {
        self.beta_sensor_shaft_angle_offset
    }
}

impl Default for AssemblyParameters {
    fn default() -> Self {
        Self::new(ShaftAngle::new(0.0), ShaftAngle::new(2.0 * TAU / 7.0))
    }
}

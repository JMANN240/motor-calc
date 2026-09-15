use motor_calc_core::parameters::Parameters;

use crate::{
    model::mounted_sensor::MountedSensorModel,
    types::{
        sensor_type::{Beta, SensorType},
        shaft_angle::ShaftAngle,
    },
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GradientMountedSensorModel<'g, 'm, 's, T: SensorType> {
    mounted_sensor: &'g MountedSensorModel<'m, 's, T>,
}

impl<'g, 'm, 's, T: SensorType> GradientMountedSensorModel<'g, 'm, 's, T> {
    pub fn new(mounted_sensor: &'g MountedSensorModel<'m, 's, T>) -> Self {
        Self { mounted_sensor }
    }

    pub fn mounted_sensor(&self) -> &MountedSensorModel<'m, 's, T> {
        self.mounted_sensor
    }
}

impl<'g, 'm, 's> GradientMountedSensorModel<'g, 'm, 's, Beta> {
    pub fn grad_shaft_angle_offset(&self) -> Parameters {
        Parameters::just_beta_shaft_angle_offset()
    }

    pub fn grad_sin_estimated_shaft_angle_offset(&self) -> Parameters {
        motor_calc_core::gradient::grad_sin_beta_shaft_angle_offset(
            self.mounted_sensor()
                .parameters()
                .shaft_angle_offset()
                .radians(),
            self.grad_shaft_angle_offset(),
        )
    }

    pub fn grad_cos_estimated_shaft_angle_offset(&self) -> Parameters {
        motor_calc_core::gradient::grad_cos_beta_shaft_angle_offset(
            self.mounted_sensor()
                .parameters()
                .shaft_angle_offset()
                .radians(),
            self.grad_shaft_angle_offset(),
        )
    }

    pub fn grad_cos_estimated_relative_shaft_angle(
        &self,
        estimated_shaft_angle: ShaftAngle,
        grad_estimated_shaft_angle: Parameters,
    ) -> Parameters {
        motor_calc_core::gradient::grad_cos_estimated_relative_shaft_angle(
            self.mounted_sensor()
                .relative_shaft_angle(estimated_shaft_angle)
                .radians(),
            self.grad_estimated_relative_shaft_angle(grad_estimated_shaft_angle),
        )
    }

    pub fn grad_estimated_relative_shaft_angle(
        &self,
        grad_estimated_shaft_angle: Parameters,
    ) -> Parameters {
        motor_calc_core::gradient::grad_estimated_relative_shaft_angle(
            grad_estimated_shaft_angle,
            self.grad_shaft_angle_offset(),
        )
    }

    pub fn grad_sin_estimated_relative_shaft_angle(
        &self,
        estimated_shaft_angle: ShaftAngle,
        grad_estimated_shaft_angle: Parameters,
    ) -> Parameters {
        motor_calc_core::gradient::grad_sin_estimated_relative_shaft_angle(
            self.mounted_sensor()
                .relative_shaft_angle(estimated_shaft_angle)
                .radians(),
            self.grad_estimated_relative_shaft_angle(grad_estimated_shaft_angle),
        )
    }
}

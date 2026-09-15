use motor_calc_core::parameters::Parameters;

use crate::{
    model::{
        Adjustable,
        motor::MotorModel,
        mounted_sensor::{
            forward::ForwardMountedSensorModel, gradient::GradientMountedSensorModel,
            inverse::InverseMountedSensorModel, parameters::MountedSensorParameters,
        },
        sensor::SensorModel,
    },
    types::{
        sensor_type::{Beta, SensorType},
        shaft_angle::ShaftAngle,
    },
};

pub mod forward;
pub mod gradient;
pub mod inverse;
pub mod parameters;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MountedSensorModel<'m, 's, T: SensorType> {
    motor: &'m MotorModel,
    sensor: &'s SensorModel<T>,
    parameters: MountedSensorParameters,
}

impl<'m, 's, T: SensorType> MountedSensorModel<'m, 's, T> {
    pub fn new(
        motor: &'m MotorModel,
        sensor: &'s SensorModel<T>,
        parameters: MountedSensorParameters,
    ) -> Self {
        Self {
            motor,
            sensor,
            parameters,
        }
    }

    pub fn motor(&self) -> &'m MotorModel {
        self.motor
    }

    pub fn sensor(&self) -> &'s SensorModel<T> {
        self.sensor
    }

    pub fn parameters(&self) -> &MountedSensorParameters {
        &self.parameters
    }

    pub fn forward(&self) -> ForwardMountedSensorModel<'_, '_, '_, T> {
        ForwardMountedSensorModel::new(self)
    }

    pub fn inverse(&self) -> InverseMountedSensorModel<'_, '_, '_, T> {
        InverseMountedSensorModel::new(self)
    }

    pub fn gradient(&self) -> GradientMountedSensorModel<'_, '_, '_, T> {
        GradientMountedSensorModel::new(self)
    }

    pub fn relative_shaft_angle(&self, shaft_angle: ShaftAngle) -> ShaftAngle {
        shaft_angle - self.parameters().shaft_angle_offset()
    }
}

impl<'m, 's> Adjustable for MountedSensorModel<'m, 's, Beta> {
    fn adjusted(&self, gradient: &Parameters) -> Self {
        Self::new(
            self.motor(),
            self.sensor(),
            MountedSensorParameters::new(
                self.parameters().shaft_angle_offset()
                    + ShaftAngle::from_radians_f64(gradient.beta_shaft_angle_offset()),
            ),
        )
    }
}

use crate::{
    model::{
        motor::MotorModel,
        mounted_sensor::{
            forward::ForwardMountedSensorModel, inverse::InverseMountedSensorModel,
            parameters::MountedSensorParameters,
        },
        sensor::SensorModel,
    },
    types::sensor_type::SensorType,
};

pub mod forward;
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

    pub fn motor(&self) -> &MotorModel {
        self.motor
    }

    pub fn sensor(&self) -> &SensorModel<T> {
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
}

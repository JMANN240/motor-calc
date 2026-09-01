use crate::{
    model::{
        motor::MotorModel,
        mounted_sensor::{forward::ForwardMountedSensorModel, inverse::InverseMountedSensorModel},
        sensor::SensorModel,
    },
    types::{sensor_type::SensorType, shaft_angle::ShaftAngle},
};

pub mod forward;
pub mod inverse;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MountedSensorModel<'m, 's, T: SensorType> {
    motor: &'m MotorModel,
    sensor: &'s SensorModel<T>,
    shaft_angle_offset: ShaftAngle,
}

impl<'m, 's, T: SensorType> MountedSensorModel<'m, 's, T> {
    pub fn new(
        motor: &'m MotorModel,
        sensor: &'s SensorModel<T>,
        shaft_angle_offset: ShaftAngle,
    ) -> Self {
        Self {
            motor,
            sensor,
            shaft_angle_offset,
        }
    }

    pub fn motor(&self) -> &MotorModel {
        self.motor
    }

    pub fn sensor(&self) -> &SensorModel<T> {
        self.sensor
    }

    pub fn shaft_angle_offset(&self) -> ShaftAngle {
        self.shaft_angle_offset
    }

    pub fn forward(&self) -> ForwardMountedSensorModel<'_, '_, '_, T> {
        ForwardMountedSensorModel::new(self)
    }

    pub fn inverse(&self) -> InverseMountedSensorModel<'_, '_, '_, T> {
        InverseMountedSensorModel::new(self)
    }
}

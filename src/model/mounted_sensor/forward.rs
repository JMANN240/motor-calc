use libm::{atan, cos, sin};

use crate::{
    model::mounted_sensor::MountedSensorModel,
    types::{sensor_angle::SensorAngle, sensor_type::SensorType, shaft_angle::ShaftAngle},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ForwardMountedSensorModel<'f, 'm, 's, T: SensorType> {
    mounted_sensor: &'f MountedSensorModel<'m, 's, T>,
}

impl<'f, 'm, 's, T: SensorType> ForwardMountedSensorModel<'f, 'm, 's, T> {
    pub fn new(mounted_sensor: &'f MountedSensorModel<'m, 's, T>) -> Self {
        Self { mounted_sensor }
    }

    pub fn mounted_sensor(&self) -> &MountedSensorModel<'m, 's, T> {
        self.mounted_sensor
    }

    pub fn sensor_angle(&self, displacement: f64, shaft_angle: ShaftAngle) -> SensorAngle<T> {
        let mounted_sensor = self.mounted_sensor();

        let motor = mounted_sensor.motor();

        SensorAngle::new(atan(
            -displacement * sin(shaft_angle.angle() - mounted_sensor.parameters().shaft_angle_offset().angle())
                / (motor.parameters().distance_ratio()
                    - displacement
                        * cos(shaft_angle.angle() - mounted_sensor.parameters().shaft_angle_offset().angle())),
        ))
    }
}

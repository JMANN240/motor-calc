use num_traits::Float;

use crate::{
    model::mounted_sensor::MountedSensorModel,
    types::{sensor_angle::SensorAngle, sensor_type::SensorType, shaft_angle::ShaftAngle},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ForwardMountedSensorModel<'f, 'm, 's, F: Float, T: SensorType> {
    mounted_sensor: &'f MountedSensorModel<'m, 's, F, T>,
}

impl<'f, 'm, 's, F: Float, T: SensorType> ForwardMountedSensorModel<'f, 'm, 's, F, T> {
    pub fn new(mounted_sensor: &'f MountedSensorModel<'m, 's, F, T>) -> Self {
        Self { mounted_sensor }
    }

    pub fn mounted_sensor(self) -> &'f MountedSensorModel<'m, 's, F, T> {
        self.mounted_sensor
    }

    pub fn sensor_angle(self, displacement: F, shaft_angle: ShaftAngle<F>) -> SensorAngle<F, T> {
        SensorAngle::from_radians_f(motor_calc_core::forward::sensor_angle(
            displacement,
            self.relative_shaft_angle(shaft_angle).radians(),
            self.mounted_sensor().motor().parameters().distance_ratio(),
        ))
    }

    pub fn relative_shaft_angle(self, shaft_angle: ShaftAngle<F>) -> ShaftAngle<F> {
        ShaftAngle::from_radians_f(motor_calc_core::forward::relative_shaft_angle(
            shaft_angle.radians(),
            self.mounted_sensor()
                .parameters()
                .shaft_angle_offset()
                .radians(),
        ))
    }
}

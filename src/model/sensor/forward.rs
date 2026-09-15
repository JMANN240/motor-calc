use num_traits::Float;

use crate::{
    model::sensor::SensorModel,
    types::{sensor_angle::SensorAngle, sensor_type::SensorType, voltage::Voltage},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ForwardSensorModel<'f, F: Float, T: SensorType> {
    sensor: &'f SensorModel<F, T>,
}

impl<'f, F: Float, T: SensorType> ForwardSensorModel<'f, F, T> {
    pub fn new(sensor: &'f SensorModel<F, T>) -> Self {
        Self { sensor }
    }

    pub fn sensor(self) -> &'f SensorModel<F, T> {
        self.sensor
    }

    pub fn voltage(self, sensor_angle: SensorAngle<F, T>) -> Voltage<F, T> {
        let sensor_parameters = self.sensor().parameters();

        Voltage::<F, T>::from_volts_f(motor_calc_core::forward::voltage(
            sensor_parameters.voltage_scale(),
            sensor_angle.radians(),
            sensor_parameters.voltage_offset().volts(),
        ))
    }
}

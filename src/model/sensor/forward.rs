use crate::{
    model::sensor::SensorModel,
    types::{sensor_angle::SensorAngle, sensor_type::SensorType, voltage::Voltage},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ForwardSensorModel<'f, T: SensorType> {
    sensor: &'f SensorModel<T>,
}

impl<'f, T: SensorType> ForwardSensorModel<'f, T> {
    pub fn new(sensor: &'f SensorModel<T>) -> Self {
        Self { sensor }
    }

    pub fn sensor(&self) -> &SensorModel<T> {
        self.sensor
    }

    pub fn voltage(&self, sensor_angle: &SensorAngle<T>) -> Voltage<T> {
        let sensor_parameters = self.sensor().parameters();

        Voltage::<T>::from_volts_f64(motor_calc_core::forward::voltage(
            sensor_parameters.voltage_scale(),
            sensor_angle.radians(),
            sensor_parameters.voltage_offset().volts(),
        ))
    }
}

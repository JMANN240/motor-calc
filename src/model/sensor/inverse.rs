use num_traits::Float;

use crate::{
    model::sensor::SensorModel,
    types::{sensor_angle::SensorAngle, sensor_type::SensorType, voltage::Voltage},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InverseSensorModel<'i, F: Float, T: SensorType> {
    sensor: &'i SensorModel<F, T>,
}

impl<'i, F: Float, T: SensorType> InverseSensorModel<'i, F, T> {
    pub fn new(sensor: &'i SensorModel<F, T>) -> Self {
        Self { sensor }
    }

    pub fn sensor(self) -> &'i SensorModel<F, T> {
        self.sensor
    }

    pub fn estimated_sensor_angle(self, voltage: Voltage<F, T>) -> SensorAngle<F, T> {
        SensorAngle::from_radians_f(motor_calc_core::inverse::estimated_sensor_angle(
            self.estimated_offset_voltage(voltage).volts(),
            self.sensor().parameters().voltage_scale(),
        ))
    }

    pub fn estimated_offset_voltage(self, voltage: Voltage<F, T>) -> Voltage<F, T> {
        Voltage::<F, T>::from_volts_f(motor_calc_core::inverse::estimated_offset_voltage(
            voltage.volts(),
            self.sensor().parameters().voltage_offset().volts(),
        ))
    }
}

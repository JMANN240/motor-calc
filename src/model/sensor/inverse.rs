use crate::{
    model::sensor::SensorModel,
    types::{sensor_angle::SensorAngle, sensor_type::SensorType, voltage::Voltage},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InverseSensorModel<'i, T: SensorType> {
    sensor: &'i SensorModel<T>,
}

impl<'i, T: SensorType> InverseSensorModel<'i, T> {
    pub fn new(sensor: &'i SensorModel<T>) -> Self {
        Self { sensor }
    }

    pub fn sensor(&self) -> &SensorModel<T> {
        self.sensor
    }

    pub fn estimated_sensor_angle(&self, voltage: &Voltage<T>) -> SensorAngle<T> {
        SensorAngle::new(motor_calc_core::inverse::estimated_sensor_angle(
            self.estimated_offset_voltage(voltage).voltage(),
            self.sensor().parameters().voltage_scale(),
        ))
    }

    pub fn estimated_offset_voltage(&self, voltage: &Voltage<T>) -> Voltage<T> {
        Voltage::<T>::from_volts_f64(motor_calc_core::inverse::estimated_offset_voltage(
            voltage.voltage(),
            self.sensor().parameters().voltage_offset().voltage(),
        ))
    }
}

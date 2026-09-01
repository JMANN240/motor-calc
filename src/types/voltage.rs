use core::marker::PhantomData;

use crate::types::sensor_type::SensorType;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Voltage<T: SensorType> {
    voltage: f64,
    sensor_type: PhantomData<T>,
}

impl<T: SensorType> Voltage<T> {
    pub fn new(voltage: f64) -> Self {
        Self {
            voltage,
            sensor_type: PhantomData,
        }
    }

    pub fn voltage(&self) -> f64 {
        self.voltage
    }
}

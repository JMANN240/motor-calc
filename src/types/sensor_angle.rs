use core::marker::PhantomData;

use crate::types::sensor_type::SensorType;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SensorAngle<T: SensorType> {
    angle: f64,
    sensor_type: PhantomData<T>,
}

impl<T: SensorType> SensorAngle<T> {
    pub fn new(angle: f64) -> Self {
        Self {
            angle,
            sensor_type: PhantomData,
        }
    }

    pub fn angle(&self) -> f64 {
        self.angle
    }
}

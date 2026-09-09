use crate::{
    model::sensor::{
        forward::ForwardSensorModel, gradient::GradientSensorModel, inverse::InverseSensorModel, parameters::SensorParameters,
    }, types::sensor_type::SensorType,
};

pub mod forward;
pub mod gradient;
pub mod inverse;
pub mod parameters;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct SensorModel<T: SensorType> {
    parameters: SensorParameters<T>,
}

impl<T: SensorType> SensorModel<T> {
    pub fn new(parameters: SensorParameters<T>) -> Self {
        Self { parameters }
    }

    pub fn parameters(&self) -> &SensorParameters<T> {
        &self.parameters
    }

    pub fn forward(&self) -> ForwardSensorModel<'_, T> {
        ForwardSensorModel::new(self)
    }

    pub fn inverse(&self) -> InverseSensorModel<'_, T> {
        InverseSensorModel::new(self)
    }

    pub fn gradient(&self) -> GradientSensorModel<'_, T> {
        GradientSensorModel::new(self)
    }
}

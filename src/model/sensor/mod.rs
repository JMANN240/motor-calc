use motor_calc_core::parameters::Parameters;
use num_traits::{Float, FloatConst};

use crate::{
    model::{
        Adjustable,
        sensor::{
            forward::ForwardSensorModel, gradient::GradientSensorModel,
            inverse::InverseSensorModel, parameters::SensorParameters,
        },
    },
    types::sensor_type::SensorType,
};

pub mod forward;
pub mod gradient;
pub mod inverse;
pub mod parameters;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SensorModel<F: Float, T: SensorType> {
    parameters: SensorParameters<F, T>,
}

impl<F: Float, T: SensorType> SensorModel<F, T> {
    pub fn new(parameters: SensorParameters<F, T>) -> Self {
        Self { parameters }
    }

    pub fn parameters(self) -> SensorParameters<F, T> {
        self.parameters
    }

    pub fn forward(&self) -> ForwardSensorModel<'_, F, T> {
        ForwardSensorModel::new(self)
    }

    pub fn inverse(&self) -> InverseSensorModel<'_, F, T> {
        InverseSensorModel::new(self)
    }

    pub fn gradient(&self) -> GradientSensorModel<'_, F, T> {
        GradientSensorModel::new(self)
    }
}

impl<F: Float, T: SensorType> Adjustable<F> for SensorModel<F, T>
where
    SensorParameters<F, T>: Adjustable<F>,
{
    fn adjusted(&self, gradient: Parameters<F>) -> Self {
        Self::new(self.parameters().adjusted(gradient))
    }
}

impl<F: Float + FloatConst, T: SensorType> Default for SensorModel<F, T> {
    fn default() -> Self {
        Self::new(SensorParameters::default())
    }
}

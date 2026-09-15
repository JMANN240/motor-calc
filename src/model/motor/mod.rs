use motor_calc_core::parameters::Parameters;
use num_traits::Float;

use crate::model::{
    Adjustable,
    motor::{gradient::GradientMotorModel, parameters::MotorParameters},
};

pub mod gradient;
pub mod parameters;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MotorModel<F: Float> {
    parameters: MotorParameters<F>,
}

impl<F: Float> MotorModel<F> {
    pub fn new(parameters: MotorParameters<F>) -> Self {
        Self { parameters }
    }

    pub fn parameters(self) -> MotorParameters<F> {
        self.parameters
    }

    pub fn gradient(&self) -> GradientMotorModel<'_, F> {
        GradientMotorModel::new(self)
    }
}

impl<F: Float> Adjustable<F> for MotorModel<F> {
    fn adjusted(&self, gradient: Parameters<F>) -> Self {
        Self::new(self.parameters().adjusted(gradient))
    }
}

impl<F: Float> Default for MotorModel<F> {
    fn default() -> Self {
        Self::new(MotorParameters::default())
    }
}

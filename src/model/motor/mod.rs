use crate::model::motor::{gradient::GradientMotorModel, parameters::MotorParameters};

pub mod gradient;
pub mod parameters;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MotorModel {
    parameters: MotorParameters,
}

impl MotorModel {
    pub fn new(parameters: MotorParameters) -> Self {
        Self { parameters }
    }

    pub fn parameters(&self) -> MotorParameters {
        self.parameters
    }

    pub fn gradient(&self) -> GradientMotorModel<'_> {
        GradientMotorModel::new(self)
    }
}

impl Default for MotorModel {
    fn default() -> Self {
        Self::new(MotorParameters::new(6.75))
    }
}

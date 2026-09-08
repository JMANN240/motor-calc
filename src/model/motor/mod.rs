use crate::model::motor::parameters::MotorParameters;

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
}

impl Default for MotorModel {
    fn default() -> Self {
        Self::new(MotorParameters::new(6.75))
    }
}

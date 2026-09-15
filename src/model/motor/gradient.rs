use motor_calc_core::parameters::Parameters;

use crate::model::motor::MotorModel;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GradientMotorModel<'g> {
    motor: &'g MotorModel,
}

impl<'g> GradientMotorModel<'g> {
    pub fn new(motor: &'g MotorModel) -> Self {
        Self { motor }
    }

    pub fn motor(&self) -> &MotorModel {
        self.motor
    }

    pub fn grad_distance_ratio(&self) -> Parameters {
        Parameters::just_distance_ratio()
    }
}

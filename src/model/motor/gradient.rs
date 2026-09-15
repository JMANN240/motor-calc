use motor_calc_core::parameters::Parameters;
use num_traits::Float;

use crate::model::motor::MotorModel;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GradientMotorModel<'g, F: Float> {
    motor: &'g MotorModel<F>,
}

impl<'g, F: Float> GradientMotorModel<'g, F> {
    pub fn new(motor: &'g MotorModel<F>) -> Self {
        Self { motor }
    }

    pub fn motor(self) -> &'g MotorModel<F> {
        self.motor
    }

    pub fn grad_distance_ratio(self) -> Parameters<F> {
        Parameters::just_distance_ratio()
    }
}

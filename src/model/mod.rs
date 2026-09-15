use motor_calc_core::parameters::Parameters;

pub mod assembly;
pub mod motor;
pub mod mounted_sensor;
pub mod sensor;

pub trait Adjustable: Sized {
    fn adjusted(&self, gradient: &Parameters) -> Self;

    fn adjust(&mut self, gradient: &Parameters) {
        *self = self.adjusted(gradient);
    }
}

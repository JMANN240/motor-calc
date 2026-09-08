use core::ops::Mul;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MotorParameters {
    distance_ratio: f64,
}

impl MotorParameters {
    pub fn new(distance_ratio: f64) -> Self {
        Self { distance_ratio }
    }

    pub fn distance_ratio(&self) -> f64 {
        self.distance_ratio
    }
}

impl Mul<f64> for &MotorParameters {
    type Output = MotorParameters;

    fn mul(self, rhs: f64) -> Self::Output {
        MotorParameters::new(self.distance_ratio() * rhs)
    }
}

impl Mul<f64> for MotorParameters {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        &self * rhs
    }
}

impl Mul<&MotorParameters> for f64 {
    type Output = MotorParameters;

    fn mul(self, rhs: &MotorParameters) -> Self::Output {
        rhs * self
    }
}

impl Mul<MotorParameters> for f64 {
    type Output = MotorParameters;

    fn mul(self, rhs: MotorParameters) -> Self::Output {
        rhs * self
    }
}

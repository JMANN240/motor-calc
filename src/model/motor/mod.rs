#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MotorModel {
    distance_ratio: f64,
}

impl MotorModel {
    pub fn new(distance_ratio: f64) -> Self {
        Self { distance_ratio }
    }

    pub fn distance_ratio(&self) -> f64 {
        self.distance_ratio
    }
}

impl Default for MotorModel {
    fn default() -> Self {
        Self::new(6.75)
    }
}

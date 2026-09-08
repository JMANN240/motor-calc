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
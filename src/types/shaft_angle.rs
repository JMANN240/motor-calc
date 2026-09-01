#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShaftAngle {
    angle: f64,
}

impl ShaftAngle {
    pub fn new(angle: f64) -> Self {
        Self { angle }
    }

    pub fn angle(&self) -> f64 {
        self.angle
    }
}

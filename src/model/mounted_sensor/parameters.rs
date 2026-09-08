use crate::types::shaft_angle::ShaftAngle;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MountedSensorParameters {
    shaft_angle_offset: ShaftAngle,
}

impl MountedSensorParameters {
    pub fn new(shaft_angle_offset: ShaftAngle) -> Self {
        Self { shaft_angle_offset }
    }

    pub fn shaft_angle_offset(&self) -> ShaftAngle {
        self.shaft_angle_offset
    }
}

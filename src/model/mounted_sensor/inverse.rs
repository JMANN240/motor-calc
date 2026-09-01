use libm::{cos, sin, tan};

use crate::{
    model::mounted_sensor::MountedSensorModel,
    types::{sensor_angle::SensorAngle, sensor_type::SensorType, shaft_angle::ShaftAngle},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InverseMountedSensorModel<'i, 'm, 's, T: SensorType> {
    mounted_sensor: &'i MountedSensorModel<'m, 's, T>,
}

impl<'i, 'm, 's, T: SensorType> InverseMountedSensorModel<'i, 'm, 's, T> {
    pub fn new(mounted_sensor: &'i MountedSensorModel<'m, 's, T>) -> Self {
        Self { mounted_sensor }
    }

    pub fn mounted_sensor(&self) -> &MountedSensorModel<'m, 's, T> {
        self.mounted_sensor
    }

    pub fn estimated_displacement(
        &self,
        estimated_sensor_angle: &SensorAngle<T>,
        estimated_shaft_angle: ShaftAngle,
    ) -> Option<f64> {
        let estimated_displacement = self.estimated_displacement_numerator(estimated_sensor_angle)
            / self
                .estimated_displacement_denominator(estimated_sensor_angle, estimated_shaft_angle);

        estimated_displacement
            .is_finite()
            .then_some(estimated_displacement)
    }

    pub fn estimated_displacement_numerator(&self, estimated_sensor_angle: &SensorAngle<T>) -> f64 {
        let mounted_sensor = self.mounted_sensor();

        let motor = mounted_sensor.motor();

        let tan_estimated_sensor_angle = tan(estimated_sensor_angle.angle());
        let distance_ratio = motor.distance_ratio();

        tan_estimated_sensor_angle * distance_ratio
    }

    /// Sometimes the denominator is zero. This is bad, because dividing by zero is bad.
    /// Sadly, this is unavoidable, because there are some scenarios where a single sensor
    /// just can't estimate the displacement given its angle and the angle of the shaft.
    ///
    /// More precisely, when the sine of relative shaft angle
    /// (defined as the difference between shaft angle and the sensor's shaft angle offset)
    /// is equal to zero. The only time this really happens is when the shaft is either facing directly towards or away from the sensor.
    pub fn estimated_displacement_denominator(
        &self,
        estimated_sensor_angle: &SensorAngle<T>,
        estimated_shaft_angle: ShaftAngle,
    ) -> f64 {
        let relative_estimated_shaft_angle =
            estimated_shaft_angle.angle() - self.mounted_sensor().shaft_angle_offset().angle();

        let tan_estimated_sensor_angle = tan(estimated_sensor_angle.angle());
        let sin_relative_estimated_shaft_angle = sin(relative_estimated_shaft_angle);
        let cos_relative_estimated_shaft_angle = cos(relative_estimated_shaft_angle);

        tan_estimated_sensor_angle * cos_relative_estimated_shaft_angle
            - sin_relative_estimated_shaft_angle
    }
}

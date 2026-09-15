use crate::{
    model::mounted_sensor::MountedSensorModel,
    types::{
        sensor_angle::SensorAngle,
        sensor_type::{Alpha, Beta, SensorType},
        shaft_angle::ShaftAngle,
    },
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

    pub fn estimated_relative_shaft_angle(&self, estimated_shaft_angle: ShaftAngle) -> ShaftAngle {
        ShaftAngle::from_radians_f64(motor_calc_core::forward::relative_shaft_angle(
            estimated_shaft_angle.radians(),
            self.mounted_sensor()
                .parameters()
                .shaft_angle_offset()
                .radians(),
        ))
    }
}

impl<'i, 'm, 's> InverseMountedSensorModel<'i, 'm, 's, Alpha> {
    pub fn estimated_displacement(
        &self,
        estimated_alpha_sensor_angle: &SensorAngle<Alpha>,
        estimated_shaft_angle: ShaftAngle,
    ) -> Option<f64> {
        let estimated_displacement = motor_calc_core::inverse::estimated_alpha_displacement(
            self.estimated_displacement_numerator(estimated_alpha_sensor_angle),
            self.estimated_displacement_denominator(
                estimated_alpha_sensor_angle,
                estimated_shaft_angle,
            ),
        );

        estimated_displacement
            .is_finite()
            .then_some(estimated_displacement)
    }

    pub fn estimated_displacement_numerator(
        &self,
        estimated_alpha_sensor_angle: &SensorAngle<Alpha>,
    ) -> f64 {
        motor_calc_core::inverse::estimated_alpha_displacement_numerator(
            estimated_alpha_sensor_angle.radians(),
            self.mounted_sensor().motor().parameters().distance_ratio(),
        )
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
        estimated_alpha_sensor_angle: &SensorAngle<Alpha>,
        estimated_shaft_angle: ShaftAngle,
    ) -> f64 {
        motor_calc_core::inverse::estimated_alpha_displacement_denominator(
            estimated_alpha_sensor_angle.radians(),
            estimated_shaft_angle.radians(),
        )
    }
}

impl<'i, 'm, 's> InverseMountedSensorModel<'i, 'm, 's, Beta> {
    pub fn estimated_displacement(
        &self,
        estimated_beta_sensor_angle: &SensorAngle<Beta>,
        estimated_shaft_angle: ShaftAngle,
    ) -> Option<f64> {
        let estimated_displacement = motor_calc_core::inverse::estimated_beta_displacement(
            self.estimated_displacement_numerator(estimated_beta_sensor_angle),
            self.estimated_displacement_denominator(
                estimated_beta_sensor_angle,
                estimated_shaft_angle,
            ),
        );

        estimated_displacement
            .is_finite()
            .then_some(estimated_displacement)
    }

    pub fn estimated_displacement_numerator(
        &self,
        estimated_beta_sensor_angle: &SensorAngle<Beta>,
    ) -> f64 {
        motor_calc_core::inverse::estimated_beta_displacement_numerator(
            estimated_beta_sensor_angle.radians(),
            self.mounted_sensor().motor().parameters().distance_ratio(),
        )
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
        estimated_beta_sensor_angle: &SensorAngle<Beta>,
        estimated_shaft_angle: ShaftAngle,
    ) -> f64 {
        motor_calc_core::inverse::estimated_beta_displacement_denominator(
            estimated_beta_sensor_angle.radians(),
            self.estimated_relative_shaft_angle(estimated_shaft_angle)
                .radians(),
        )
    }
}

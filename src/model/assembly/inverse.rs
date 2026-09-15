use libm::atan2;

use crate::{
    model::assembly::AssemblyModel,
    types::{
        sensor_angle::SensorAngle,
        sensor_type::{Alpha, Beta},
        shaft_angle::ShaftAngle,
    },
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InverseAssemblyModel<'i> {
    assembly: &'i AssemblyModel,
}

impl<'i> InverseAssemblyModel<'i> {
    pub fn new(assembly: &'i AssemblyModel) -> Self {
        Self { assembly }
    }

    pub fn assembly(&self) -> &AssemblyModel {
        self.assembly
    }

    pub fn squared_error_estimated_displacement(&self, error_estimated_displacement: f64) -> f64 {
        error_estimated_displacement * error_estimated_displacement
    }

    pub fn error_estimated_displacement(
        &self,
        estimated_displacement: f64,
        displacement: f64,
    ) -> f64 {
        estimated_displacement - displacement
    }

    pub fn estimated_displacement(
        &self,
        estimated_alpha_sensor_angle: &SensorAngle<Alpha>,
        estimated_beta_sensor_angle: &SensorAngle<Beta>,
    ) -> f64 {
        let estimated_shaft_angle =
            self.estimated_shaft_angle(estimated_alpha_sensor_angle, estimated_beta_sensor_angle);

        let assembly = self.assembly();

        let maybe_alpha_estimated_displacement = assembly
            .mounted_alpha_sensor()
            .inverse()
            .estimated_displacement(estimated_alpha_sensor_angle, estimated_shaft_angle);

        let maybe_beta_estimated_displacement = assembly
            .mounted_beta_sensor()
            .inverse()
            .estimated_displacement(estimated_beta_sensor_angle, estimated_shaft_angle);

        match (
            maybe_alpha_estimated_displacement,
            maybe_beta_estimated_displacement,
        ) {
            (Some(alpha_estimated_displacement), Some(beta_estimated_displacement)) => {
                (alpha_estimated_displacement + beta_estimated_displacement) / 2.0
            }
            (Some(alpha_estimated_displacement), None) => alpha_estimated_displacement,
            (None, Some(beta_estimated_displacement)) => beta_estimated_displacement,
            (None, None) => unreachable!(
                "unless both sensors are occupying the exact same space, one of their estimated displacements will always be Some"
            ),
        }
    }

    pub fn estimated_shaft_angle(
        &self,
        estimated_alpha_sensor_angle: &SensorAngle<Alpha>,
        estimated_beta_sensor_angle: &SensorAngle<Beta>,
    ) -> ShaftAngle {
        ShaftAngle::from_radians_f64(atan2(
            self.estimated_shaft_angle_numerator(
                estimated_alpha_sensor_angle,
                estimated_beta_sensor_angle,
            ),
            self.estimated_shaft_angle_denominator(
                estimated_alpha_sensor_angle,
                estimated_beta_sensor_angle,
            ),
        ))
    }

    pub fn estimated_shaft_angle_inner(
        &self,
        estimated_alpha_sensor_angle: &SensorAngle<Alpha>,
        estimated_beta_sensor_angle: &SensorAngle<Beta>,
    ) -> f64 {
        self.estimated_shaft_angle_numerator(
            estimated_alpha_sensor_angle,
            estimated_beta_sensor_angle,
        ) / self.estimated_shaft_angle_denominator(
            estimated_alpha_sensor_angle,
            estimated_beta_sensor_angle,
        )
    }

    pub fn estimated_shaft_angle_numerator(
        &self,
        estimated_alpha_sensor_angle: &SensorAngle<Alpha>,
        estimated_beta_sensor_angle: &SensorAngle<Beta>,
    ) -> f64 {
        motor_calc_core::inverse::estimated_shaft_angle_numerator(
            estimated_alpha_sensor_angle.radians(),
            estimated_beta_sensor_angle.radians(),
            self.assembly()
                .parameters()
                .mounted_beta_sensor_parameters()
                .shaft_angle_offset()
                .radians(),
        )
    }

    pub fn estimated_shaft_angle_denominator(
        &self,
        estimated_alpha_sensor_angle: &SensorAngle<Alpha>,
        estimated_beta_sensor_angle: &SensorAngle<Beta>,
    ) -> f64 {
        motor_calc_core::inverse::estimated_shaft_angle_denominator(
            estimated_alpha_sensor_angle.radians(),
            estimated_beta_sensor_angle.radians(),
            self.assembly()
                .parameters()
                .mounted_beta_sensor_parameters()
                .shaft_angle_offset()
                .radians(),
        )
    }
}

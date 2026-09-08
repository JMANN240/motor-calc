use libm::{atan2, cos, sin, tan};

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

    pub fn estimated_displacement(
        &self,
        estimated_alpha_sensor_angle: SensorAngle<Alpha>,
        estimated_beta_sensor_angle: SensorAngle<Beta>,
    ) -> f64 {
        let estimated_shaft_angle =
            self.estimated_shaft_angle(estimated_alpha_sensor_angle, estimated_beta_sensor_angle);

        let assembly = self.assembly();

        let maybe_alpha_estimated_displacement = assembly
            .mounted_alpha_sensor()
            .inverse()
            .estimated_displacement(&estimated_alpha_sensor_angle, estimated_shaft_angle);

        let maybe_beta_estimated_displacement = assembly
            .mounted_beta_sensor()
            .inverse()
            .estimated_displacement(&estimated_beta_sensor_angle, estimated_shaft_angle);

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
        estimated_alpha_sensor_angle: SensorAngle<Alpha>,
        estimated_beta_sensor_angle: SensorAngle<Beta>,
    ) -> ShaftAngle {
        ShaftAngle::new(atan2(
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

    pub fn estimated_shaft_angle_numerator(
        &self,
        estimated_alpha_sensor_angle: SensorAngle<Alpha>,
        estimated_beta_sensor_angle: SensorAngle<Beta>,
    ) -> f64 {
        let tan_alpha_sensor_angle = tan(estimated_alpha_sensor_angle.angle());
        let tan_beta_sensor_angle = tan(estimated_beta_sensor_angle.angle());
        let sin_beta_sensor_shaft_angle_offset = sin(self
            .assembly()
            .parameters()
            .mounted_beta_sensor_parameters()
            .shaft_angle_offset()
            .angle());
        let cos_beta_sensor_shaft_angle_offset = cos(self
            .assembly()
            .parameters()
            .mounted_beta_sensor_parameters()
            .shaft_angle_offset()
            .angle());

        tan_alpha_sensor_angle * tan_beta_sensor_angle
            - tan_alpha_sensor_angle * tan_beta_sensor_angle * cos_beta_sensor_shaft_angle_offset
            - tan_alpha_sensor_angle * sin_beta_sensor_shaft_angle_offset
    }

    pub fn estimated_shaft_angle_denominator(
        &self,
        estimated_alpha_sensor_angle: SensorAngle<Alpha>,
        estimated_beta_sensor_angle: SensorAngle<Beta>,
    ) -> f64 {
        let tan_alpha_sensor_angle = tan(estimated_alpha_sensor_angle.angle());
        let tan_beta_sensor_angle = tan(estimated_beta_sensor_angle.angle());
        let sin_beta_sensor_shaft_angle_offset = sin(self
            .assembly()
            .parameters()
            .mounted_beta_sensor_parameters()
            .shaft_angle_offset()
            .angle());
        let cos_beta_sensor_shaft_angle_offset = cos(self
            .assembly()
            .parameters()
            .mounted_beta_sensor_parameters()
            .shaft_angle_offset()
            .angle());

        tan_alpha_sensor_angle * tan_beta_sensor_angle * sin_beta_sensor_shaft_angle_offset
            - tan_alpha_sensor_angle * cos_beta_sensor_shaft_angle_offset
            + tan_beta_sensor_angle
    }
}

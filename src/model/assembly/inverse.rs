use num_traits::Float;

use crate::{
    model::assembly::AssemblyModel,
    types::{
        sensor_angle::SensorAngle,
        sensor_type::{Alpha, Beta},
        shaft_angle::ShaftAngle,
    },
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InverseAssemblyModel<'i, F: Float> {
    assembly: &'i AssemblyModel<F>,
}

impl<'i, F: Float> InverseAssemblyModel<'i, F> {
    pub fn new(assembly: &'i AssemblyModel<F>) -> Self {
        Self { assembly }
    }

    pub fn assembly(self) -> &'i AssemblyModel<F> {
        self.assembly
    }

    pub fn squared_error_estimated_displacement(self, error_estimated_displacement: F) -> F {
        error_estimated_displacement * error_estimated_displacement
    }

    pub fn error_estimated_displacement(self, estimated_displacement: F, displacement: F) -> F {
        estimated_displacement - displacement
    }

    pub fn estimated_displacement(
        self,
        estimated_alpha_sensor_angle: SensorAngle<F, Alpha>,
        estimated_beta_sensor_angle: SensorAngle<F, Beta>,
    ) -> F {
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
                (alpha_estimated_displacement + beta_estimated_displacement)
                    / F::from(2).expect("2 can always be represented with a float")
            }
            (Some(alpha_estimated_displacement), None) => alpha_estimated_displacement,
            (None, Some(beta_estimated_displacement)) => beta_estimated_displacement,
            (None, None) => unreachable!(
                "unless both sensors are occupying the exact same space, one of their estimated displacements will always be Some"
            ),
        }
    }

    pub fn estimated_shaft_angle(
        self,
        estimated_alpha_sensor_angle: SensorAngle<F, Alpha>,
        estimated_beta_sensor_angle: SensorAngle<F, Beta>,
    ) -> ShaftAngle<F> {
        ShaftAngle::from_radians_f(
            self.estimated_shaft_angle_numerator(
                estimated_alpha_sensor_angle,
                estimated_beta_sensor_angle,
            )
            .atan2(self.estimated_shaft_angle_denominator(
                estimated_alpha_sensor_angle,
                estimated_beta_sensor_angle,
            )),
        )
    }

    pub fn estimated_shaft_angle_inner(
        self,
        estimated_alpha_sensor_angle: SensorAngle<F, Alpha>,
        estimated_beta_sensor_angle: SensorAngle<F, Beta>,
    ) -> F {
        self.estimated_shaft_angle_numerator(
            estimated_alpha_sensor_angle,
            estimated_beta_sensor_angle,
        ) / self.estimated_shaft_angle_denominator(
            estimated_alpha_sensor_angle,
            estimated_beta_sensor_angle,
        )
    }

    pub fn estimated_shaft_angle_numerator(
        self,
        estimated_alpha_sensor_angle: SensorAngle<F, Alpha>,
        estimated_beta_sensor_angle: SensorAngle<F, Beta>,
    ) -> F {
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
        self,
        estimated_alpha_sensor_angle: SensorAngle<F, Alpha>,
        estimated_beta_sensor_angle: SensorAngle<F, Beta>,
    ) -> F {
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

use motor_calc_core::parameters::Parameters;

use crate::{
    model::assembly::AssemblyModel,
    types::{
        sensor_type::{Alpha, Beta},
        voltage::Voltage,
    },
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GradientAssemblyModel<'g> {
    assembly: &'g AssemblyModel,
}

impl<'g> GradientAssemblyModel<'g> {
    pub fn new(assembly: &'g AssemblyModel) -> Self {
        Self { assembly }
    }

    pub fn assembly(&self) -> &AssemblyModel {
        self.assembly
    }

    pub fn grad_estimated_alpha_displacement(
        &self,
        alpha_voltage: &Voltage<Alpha>,
        beta_voltage: &Voltage<Beta>,
    ) -> Parameters {
        let estimated_alpha_sensor_angle = &self
            .assembly()
            .alpha_sensor_ref()
            .inverse()
            .estimated_sensor_angle(alpha_voltage);

        let estimated_beta_sensor_angle = &self
            .assembly()
            .beta_sensor_ref()
            .inverse()
            .estimated_sensor_angle(beta_voltage);

        let estimated_shaft_angle = self
            .assembly()
            .inverse()
            .estimated_shaft_angle(estimated_alpha_sensor_angle, estimated_beta_sensor_angle);

        let grad_estimated_alpha_displacement_numerator =
            self.grad_estimated_alpha_displacement_numerator(alpha_voltage);

        let estimated_alpha_displacement_denominator = self
            .assembly()
            .mounted_alpha_sensor()
            .inverse()
            .estimated_displacement_numerator(estimated_alpha_sensor_angle);

        let estimated_alpha_displacement_numerator = self
            .assembly()
            .mounted_alpha_sensor()
            .inverse()
            .estimated_displacement_denominator(
                estimated_alpha_sensor_angle,
                estimated_shaft_angle,
            );

        let grad_estimated_alpha_displacement_denominator =
            self.grad_estimated_alpha_displacement_denominator(alpha_voltage, beta_voltage);

        motor_calc_core::gradient::grad_estimated_alpha_displacement(
            grad_estimated_alpha_displacement_numerator,
            estimated_alpha_displacement_denominator,
            estimated_alpha_displacement_numerator,
            grad_estimated_alpha_displacement_denominator,
        )
    }

    fn grad_estimated_alpha_displacement_numerator(
        &self,
        alpha_voltage: &Voltage<Alpha>,
    ) -> Parameters {
        let estimated_alpha_sensor_angle = self
            .assembly()
            .alpha_sensor_ref()
            .inverse()
            .estimated_sensor_angle(alpha_voltage);

        motor_calc_core::gradient::grad_estimated_alpha_displacement_numerator(
            estimated_alpha_sensor_angle.angle(),
            Parameters::just_distance_ratio(),
            self.assembly().motor().parameters().distance_ratio(),
            self.assembly()
                .alpha_sensor_ref()
                .gradient()
                .grad_tan_estimated_sensor_angle(alpha_voltage),
        )
    }

    fn grad_estimated_alpha_displacement_denominator(
        &self,
        alpha_voltage: &Voltage<Alpha>,
        beta_voltage: &Voltage<Beta>,
    ) -> Parameters {
        let estimated_alpha_sensor_angle = self
            .assembly()
            .alpha_sensor_ref()
            .inverse()
            .estimated_sensor_angle(alpha_voltage);

        let estimated_beta_sensor_angle = self
            .assembly()
            .beta_sensor_ref()
            .inverse()
            .estimated_sensor_angle(beta_voltage);

        motor_calc_core::gradient::grad_estimated_alpha_displacement_denominator(
            self.assembly()
                .inverse()
                .estimated_shaft_angle(&estimated_alpha_sensor_angle, &estimated_beta_sensor_angle)
                .angle(),
            estimated_alpha_sensor_angle.angle(),
            self.assembly()
                .alpha_sensor_ref()
                .gradient()
                .grad_tan_estimated_sensor_angle(alpha_voltage),
            self.grad_cos_estimated_shaft_angle(alpha_voltage, beta_voltage),
            self.grad_sin_estimated_shaft_angle(alpha_voltage, beta_voltage),
        )
    }

    pub fn grad_estimated_beta_displacement(
        &self,
        alpha_voltage: &Voltage<Alpha>,
        beta_voltage: &Voltage<Beta>,
    ) -> Parameters {
        let estimated_alpha_sensor_angle = &self
            .assembly()
            .alpha_sensor_ref()
            .inverse()
            .estimated_sensor_angle(alpha_voltage);

        let estimated_beta_sensor_angle = &self
            .assembly()
            .beta_sensor_ref()
            .inverse()
            .estimated_sensor_angle(beta_voltage);

        let estimated_shaft_angle = self
            .assembly()
            .inverse()
            .estimated_shaft_angle(estimated_alpha_sensor_angle, estimated_beta_sensor_angle);

        let grad_estimated_beta_displacement_numerator =
            self.grad_estimated_beta_displacement_numerator(beta_voltage);

        let estimated_beta_displacement_denominator = self
            .assembly()
            .mounted_beta_sensor()
            .inverse()
            .estimated_displacement_numerator(estimated_beta_sensor_angle);

        let estimated_beta_displacement_numerator = self
            .assembly()
            .mounted_beta_sensor()
            .inverse()
            .estimated_displacement_denominator(estimated_beta_sensor_angle, estimated_shaft_angle);

        let grad_estimated_beta_displacement_denominator =
            self.grad_estimated_beta_displacement_denominator(alpha_voltage, beta_voltage);

        motor_calc_core::gradient::grad_estimated_beta_displacement(
            grad_estimated_beta_displacement_numerator,
            estimated_beta_displacement_denominator,
            estimated_beta_displacement_numerator,
            grad_estimated_beta_displacement_denominator,
        )
    }

    fn grad_estimated_beta_displacement_numerator(
        &self,
        beta_voltage: &Voltage<Beta>,
    ) -> Parameters {
        let estimated_beta_sensor_angle = self
            .assembly()
            .beta_sensor_ref()
            .inverse()
            .estimated_sensor_angle(beta_voltage);

        motor_calc_core::gradient::grad_estimated_beta_displacement_numerator(
            estimated_beta_sensor_angle.angle(),
            Parameters::just_distance_ratio(),
            self.assembly().motor().parameters().distance_ratio(),
            self.assembly()
                .beta_sensor_ref()
                .gradient()
                .grad_tan_estimated_sensor_angle(beta_voltage),
        )
    }

    fn grad_estimated_beta_displacement_denominator(
        &self,
        alpha_voltage: &Voltage<Alpha>,
        beta_voltage: &Voltage<Beta>,
    ) -> Parameters {
        let estimated_alpha_sensor_angle = self
            .assembly()
            .alpha_sensor_ref()
            .inverse()
            .estimated_sensor_angle(alpha_voltage);

        let estimated_beta_sensor_angle = self
            .assembly()
            .beta_sensor_ref()
            .inverse()
            .estimated_sensor_angle(beta_voltage);

        let estimated_shaft_angle = self
            .assembly()
            .inverse()
            .estimated_shaft_angle(&estimated_alpha_sensor_angle, &estimated_beta_sensor_angle);

        let grad_estimated_shaft_angle =
            self.grad_estimated_shaft_angle(alpha_voltage, beta_voltage);

        motor_calc_core::gradient::grad_estimated_beta_displacement_denominator(
            self.assembly()
                .mounted_beta_sensor()
                .inverse()
                .estimated_relative_shaft_angle(estimated_shaft_angle)
                .angle(),
            estimated_beta_sensor_angle.angle(),
            self.assembly()
                .beta_sensor_ref()
                .gradient()
                .grad_tan_estimated_sensor_angle(beta_voltage),
            self.assembly()
                .mounted_beta_sensor()
                .gradient()
                .grad_cos_estimated_relative_shaft_angle(
                    estimated_shaft_angle,
                    grad_estimated_shaft_angle,
                ),
            self.assembly()
                .mounted_beta_sensor()
                .gradient()
                .grad_sin_estimated_relative_shaft_angle(
                    estimated_shaft_angle,
                    grad_estimated_shaft_angle,
                ),
        )
    }

    pub fn grad_cos_estimated_shaft_angle(
        &self,
        alpha_voltage: &Voltage<Alpha>,
        beta_voltage: &Voltage<Beta>,
    ) -> Parameters {
        let estimated_alpha_sensor_angle = self
            .assembly()
            .alpha_sensor_ref()
            .inverse()
            .estimated_sensor_angle(alpha_voltage);

        let estimated_beta_sensor_angle = self
            .assembly()
            .beta_sensor_ref()
            .inverse()
            .estimated_sensor_angle(beta_voltage);

        motor_calc_core::gradient::grad_cos_estimated_shaft_angle(
            self.assembly()
                .inverse()
                .estimated_shaft_angle(&estimated_alpha_sensor_angle, &estimated_beta_sensor_angle)
                .angle(),
            self.grad_estimated_shaft_angle(alpha_voltage, beta_voltage),
        )
    }

    pub fn grad_sin_estimated_shaft_angle(
        &self,
        alpha_voltage: &Voltage<Alpha>,
        beta_voltage: &Voltage<Beta>,
    ) -> Parameters {
        let estimated_alpha_sensor_angle = self
            .assembly()
            .alpha_sensor_ref()
            .inverse()
            .estimated_sensor_angle(alpha_voltage);

        let estimated_beta_sensor_angle = self
            .assembly()
            .beta_sensor_ref()
            .inverse()
            .estimated_sensor_angle(beta_voltage);

        motor_calc_core::gradient::grad_sin_estimated_shaft_angle(
            self.assembly()
                .inverse()
                .estimated_shaft_angle(&estimated_alpha_sensor_angle, &estimated_beta_sensor_angle)
                .angle(),
            self.grad_estimated_shaft_angle(alpha_voltage, beta_voltage),
        )
    }

    pub fn grad_estimated_shaft_angle(
        &self,
        alpha_voltage: &Voltage<Alpha>,
        beta_voltage: &Voltage<Beta>,
    ) -> Parameters {
        let estimated_alpha_sensor_angle = self
            .assembly()
            .alpha_sensor_ref()
            .inverse()
            .estimated_sensor_angle(alpha_voltage);

        let estimated_beta_sensor_angle = self
            .assembly()
            .beta_sensor_ref()
            .inverse()
            .estimated_sensor_angle(beta_voltage);

        motor_calc_core::gradient::grad_estimated_shaft_angle(
            self.grad_estimated_shaft_angle_inner(alpha_voltage, beta_voltage),
            self.assembly()
                .inverse()
                .estimated_shaft_angle(&estimated_alpha_sensor_angle, &estimated_beta_sensor_angle)
                .angle(),
        )
    }

    pub fn grad_estimated_shaft_angle_inner(
        &self,
        alpha_voltage: &Voltage<Alpha>,
        beta_voltage: &Voltage<Beta>,
    ) -> Parameters {
        let estimated_alpha_sensor_angle = self
            .assembly()
            .alpha_sensor_ref()
            .inverse()
            .estimated_sensor_angle(alpha_voltage);

        let estimated_beta_sensor_angle = self
            .assembly()
            .beta_sensor_ref()
            .inverse()
            .estimated_sensor_angle(beta_voltage);

        motor_calc_core::gradient::grad_estimated_shaft_angle_inner(
            self.grad_estimated_shaft_angle_numerator(alpha_voltage, beta_voltage),
            self.assembly().inverse().estimated_shaft_angle_denominator(
                &estimated_alpha_sensor_angle,
                &estimated_beta_sensor_angle,
            ),
            self.assembly().inverse().estimated_shaft_angle_numerator(
                &estimated_alpha_sensor_angle,
                &estimated_beta_sensor_angle,
            ),
            self.grad_estimated_shaft_angle_denominator(alpha_voltage, beta_voltage),
        )
    }

    pub fn grad_estimated_shaft_angle_numerator(
        &self,
        alpha_voltage: &Voltage<Alpha>,
        beta_voltage: &Voltage<Beta>,
    ) -> Parameters {
        let estimated_alpha_sensor_angle = self
            .assembly()
            .alpha_sensor_ref()
            .inverse()
            .estimated_sensor_angle(alpha_voltage);

        let estimated_beta_sensor_angle = self
            .assembly()
            .beta_sensor_ref()
            .inverse()
            .estimated_sensor_angle(beta_voltage);

        motor_calc_core::gradient::grad_estimated_shaft_angle_numerator(
            self.assembly()
                .alpha_sensor_ref()
                .gradient()
                .grad_tan_estimated_sensor_angle(alpha_voltage),
            estimated_beta_sensor_angle.angle(),
            estimated_alpha_sensor_angle.angle(),
            self.assembly()
                .beta_sensor_ref()
                .gradient()
                .grad_tan_estimated_sensor_angle(beta_voltage),
            self.assembly()
                .parameters()
                .mounted_beta_sensor_parameters()
                .shaft_angle_offset()
                .angle(),
            self.assembly()
                .mounted_beta_sensor()
                .gradient()
                .grad_cos_estimated_shaft_angle_offset(),
            self.assembly()
                .mounted_beta_sensor()
                .gradient()
                .grad_sin_estimated_shaft_angle_offset(),
        )
    }

    pub fn grad_estimated_shaft_angle_denominator(
        &self,
        alpha_voltage: &Voltage<Alpha>,
        beta_voltage: &Voltage<Beta>,
    ) -> Parameters {
        let estimated_alpha_sensor_angle = self
            .assembly()
            .alpha_sensor_ref()
            .inverse()
            .estimated_sensor_angle(alpha_voltage);

        let estimated_beta_sensor_angle = self
            .assembly()
            .beta_sensor_ref()
            .inverse()
            .estimated_sensor_angle(beta_voltage);

        motor_calc_core::gradient::grad_estimated_shaft_angle_denominator(
            self.assembly()
                .alpha_sensor_ref()
                .gradient()
                .grad_estimated_sensor_angle(alpha_voltage),
            estimated_beta_sensor_angle.angle(),
            self.assembly()
                .parameters()
                .mounted_beta_sensor_parameters()
                .shaft_angle_offset()
                .angle(),
            estimated_alpha_sensor_angle.angle(),
            self.assembly()
                .beta_sensor_ref()
                .gradient()
                .grad_tan_estimated_sensor_angle(beta_voltage),
            self.assembly()
                .mounted_beta_sensor()
                .gradient()
                .grad_sin_estimated_shaft_angle_offset(),
            self.assembly()
                .mounted_beta_sensor()
                .gradient()
                .grad_cos_estimated_shaft_angle_offset(),
        )
    }
}

use motor_calc_core::parameters::Parameters;

use crate::{
    model::sensor::SensorModel,
    types::{
        sensor_type::{Alpha, Beta, SensorType},
        voltage::Voltage,
    },
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GradientSensorModel<'g, T: SensorType> {
    sensor: &'g SensorModel<T>,
}

impl<'g, T: SensorType> GradientSensorModel<'g, T> {
    pub fn new(sensor: &'g SensorModel<T>) -> Self {
        Self { sensor }
    }

    pub fn sensor(&self) -> &SensorModel<T> {
        self.sensor
    }
}

impl<'g> GradientSensorModel<'g, Alpha> {
    pub fn grad_voltage(&self) -> Parameters {
        Parameters::zero()
    }

    pub fn grad_estimated_voltage_scale(&self) -> Parameters {
        Parameters::just_alpha_voltage_scale()
    }

    pub fn grad_estimated_voltage_offset(&self) -> Parameters {
        Parameters::just_alpha_voltage_offset()
    }

    pub fn grad_estimated_offset_voltage(&self) -> Parameters {
        motor_calc_core::gradient::grad_estimated_offset_alpha_voltage(
            self.grad_voltage(),
            self.grad_estimated_voltage_offset(),
        )
    }

    pub fn grad_estimated_sensor_angle(&self, alpha_voltage: &Voltage<Alpha>) -> Parameters {
        motor_calc_core::gradient::grad_estimated_alpha_sensor_angle(
            self.grad_estimated_offset_voltage(),
            self.sensor().parameters().voltage_scale(),
            alpha_voltage.voltage(),
            self.sensor().parameters().voltage_offset().voltage(),
            self.grad_estimated_voltage_scale(),
        )
    }

    pub fn grad_tan_estimated_sensor_angle(&self, alpha_voltage: &Voltage<Alpha>) -> Parameters {
        motor_calc_core::gradient::grad_tan_estimated_alpha_sensor_angle(
            self.sensor()
                .inverse()
                .estimated_sensor_angle(alpha_voltage)
                .angle(),
            self.grad_estimated_sensor_angle(alpha_voltage),
        )
    }
}

impl<'g> GradientSensorModel<'g, Beta> {
    pub fn grad_voltage(&self) -> Parameters {
        Parameters::zero()
    }

    pub fn grad_estimated_voltage_scale(&self) -> Parameters {
        Parameters::just_beta_voltage_scale()
    }

    pub fn grad_estimated_voltage_offset(&self) -> Parameters {
        Parameters::just_beta_voltage_offset()
    }

    pub fn grad_estimated_offset_voltage(&self) -> Parameters {
        motor_calc_core::gradient::grad_estimated_offset_beta_voltage(
            self.grad_voltage(),
            self.grad_estimated_voltage_offset(),
        )
    }

    pub fn grad_estimated_sensor_angle(&self, beta_voltage: &Voltage<Beta>) -> Parameters {
        motor_calc_core::gradient::grad_estimated_alpha_sensor_angle(
            self.grad_estimated_offset_voltage(),
            self.sensor().parameters().voltage_scale(),
            beta_voltage.voltage(),
            self.sensor().parameters().voltage_offset().voltage(),
            self.grad_estimated_voltage_scale(),
        )
    }

    pub fn grad_tan_estimated_sensor_angle(&self, beta_voltage: &Voltage<Beta>) -> Parameters {
        motor_calc_core::gradient::grad_tan_estimated_beta_sensor_angle(
            self.sensor()
                .inverse()
                .estimated_sensor_angle(beta_voltage)
                .angle(),
            self.grad_estimated_sensor_angle(beta_voltage),
        )
    }
}

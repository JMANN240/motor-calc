use motor_calc_core::parameters::Parameters;

use crate::{
    model::{
        Adjustable,
        assembly::{
            gradient::GradientAssemblyModel, inverse::InverseAssemblyModel,
            parameters::AssemblyParameters,
        },
        motor::MotorModel,
        mounted_sensor::MountedSensorModel,
        sensor::SensorModel,
    },
    types::sensor_type::{Alpha, Beta},
};

pub mod gradient;
pub mod inverse;
pub mod parameters;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct AssemblyModel {
    motor: MotorModel,
    alpha_sensor: SensorModel<Alpha>,
    beta_sensor: SensorModel<Beta>,
    parameters: AssemblyParameters,
}

impl AssemblyModel {
    pub fn new(
        motor: MotorModel,
        alpha_sensor: SensorModel<Alpha>,
        beta_sensor: SensorModel<Beta>,
        parameters: AssemblyParameters,
    ) -> Self {
        Self {
            motor,
            alpha_sensor,
            beta_sensor,
            parameters,
        }
    }

    pub fn motor(&self) -> MotorModel {
        self.motor
    }

    pub fn motor_ref(&self) -> &MotorModel {
        &self.motor
    }

    pub fn alpha_sensor(&self) -> SensorModel<Alpha> {
        self.alpha_sensor
    }

    pub fn alpha_sensor_ref(&self) -> &SensorModel<Alpha> {
        &self.alpha_sensor
    }

    pub fn beta_sensor(&self) -> SensorModel<Beta> {
        self.beta_sensor
    }

    pub fn beta_sensor_ref(&self) -> &SensorModel<Beta> {
        &self.beta_sensor
    }

    pub fn parameters(&self) -> AssemblyParameters {
        self.parameters
    }

    pub fn inverse(&self) -> InverseAssemblyModel<'_> {
        InverseAssemblyModel::new(self)
    }

    pub fn gradient(&self) -> GradientAssemblyModel<'_> {
        GradientAssemblyModel::new(self)
    }

    pub fn mounted_alpha_sensor(&self) -> MountedSensorModel<'_, '_, Alpha> {
        MountedSensorModel::new(
            self.motor_ref(),
            self.alpha_sensor_ref(),
            self.parameters().mounted_alpha_sensor_parameters(),
        )
    }

    pub fn mounted_beta_sensor(&self) -> MountedSensorModel<'_, '_, Beta> {
        MountedSensorModel::new(
            self.motor_ref(),
            self.beta_sensor_ref(),
            self.parameters().mounted_beta_sensor_parameters(),
        )
    }
}

impl Adjustable for AssemblyModel {
    fn adjusted(&self, gradient: &Parameters) -> Self {
        Self::new(
            self.motor().adjusted(gradient),
            self.alpha_sensor_ref().adjusted(gradient),
            self.beta_sensor_ref().adjusted(gradient),
            self.parameters().adjusted(gradient),
        )
    }
}

use motor_calc_core::parameters::Parameters;
use num_traits::{Float, FloatConst};

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
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AssemblyModel<F: Float> {
    motor: MotorModel<F>,
    alpha_sensor: SensorModel<F, Alpha>,
    beta_sensor: SensorModel<F, Beta>,
    parameters: AssemblyParameters<F>,
}

impl<F: Float> AssemblyModel<F> {
    pub fn new(
        motor: MotorModel<F>,
        alpha_sensor: SensorModel<F, Alpha>,
        beta_sensor: SensorModel<F, Beta>,
        parameters: AssemblyParameters<F>,
    ) -> Self {
        Self {
            motor,
            alpha_sensor,
            beta_sensor,
            parameters,
        }
    }

    pub fn motor(self) -> MotorModel<F> {
        self.motor
    }

    pub fn motor_ref(&self) -> &MotorModel<F> {
        &self.motor
    }

    pub fn alpha_sensor(self) -> SensorModel<F, Alpha> {
        self.alpha_sensor
    }

    pub fn alpha_sensor_ref(&self) -> &SensorModel<F, Alpha> {
        &self.alpha_sensor
    }

    pub fn beta_sensor(self) -> SensorModel<F, Beta> {
        self.beta_sensor
    }

    pub fn beta_sensor_ref(&self) -> &SensorModel<F, Beta> {
        &self.beta_sensor
    }

    pub fn parameters(self) -> AssemblyParameters<F> {
        self.parameters
    }

    pub fn parameters_mut(&mut self) -> &mut AssemblyParameters<F> {
        &mut self.parameters
    }

    pub fn inverse(&self) -> InverseAssemblyModel<'_, F> {
        InverseAssemblyModel::new(self)
    }

    pub fn gradient(&self) -> GradientAssemblyModel<'_, F> {
        GradientAssemblyModel::new(self)
    }

    pub fn mounted_alpha_sensor(&self) -> MountedSensorModel<'_, '_, F, Alpha> {
        MountedSensorModel::new(
            self.motor_ref(),
            self.alpha_sensor_ref(),
            self.parameters().mounted_alpha_sensor_parameters(),
        )
    }

    pub fn mounted_beta_sensor(&self) -> MountedSensorModel<'_, '_, F, Beta> {
        MountedSensorModel::new(
            self.motor_ref(),
            self.beta_sensor_ref(),
            self.parameters().mounted_beta_sensor_parameters(),
        )
    }
}

impl<F: Float> Adjustable<F> for AssemblyModel<F> {
    fn adjusted(&self, gradient: Parameters<F>) -> Self {
        Self::new(
            self.motor().adjusted(gradient),
            self.alpha_sensor_ref().adjusted(gradient),
            self.beta_sensor_ref().adjusted(gradient),
            self.parameters().adjusted(gradient),
        )
    }
}

impl<F: Float + FloatConst> Default for AssemblyModel<F> {
    fn default() -> Self {
        Self::new(
            MotorModel::default(),
            SensorModel::default(),
            SensorModel::default(),
            AssemblyParameters::default(),
        )
    }
}

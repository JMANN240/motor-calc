use motor_calc_core::parameters::Parameters;
use num_traits::Float;

use crate::{
    model::{
        Adjustable,
        motor::MotorModel,
        mounted_sensor::{
            forward::ForwardMountedSensorModel, gradient::GradientMountedSensorModel,
            inverse::InverseMountedSensorModel, parameters::MountedSensorParameters,
        },
        sensor::SensorModel,
    },
    types::{
        sensor_type::{Beta, SensorType},
        shaft_angle::ShaftAngle,
    },
};

pub mod forward;
pub mod gradient;
pub mod inverse;
pub mod parameters;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MountedSensorModel<'m, 's, F: Float, T: SensorType> {
    motor: &'m MotorModel<F>,
    sensor: &'s SensorModel<F, T>,
    parameters: MountedSensorParameters<F>,
}

impl<'m, 's, F: Float, T: SensorType> MountedSensorModel<'m, 's, F, T> {
    pub fn new(
        motor: &'m MotorModel<F>,
        sensor: &'s SensorModel<F, T>,
        parameters: MountedSensorParameters<F>,
    ) -> Self {
        Self {
            motor,
            sensor,
            parameters,
        }
    }

    pub fn motor(self) -> &'m MotorModel<F> {
        self.motor
    }

    pub fn sensor(self) -> &'s SensorModel<F, T> {
        self.sensor
    }

    pub fn parameters(self) -> MountedSensorParameters<F> {
        self.parameters
    }

    pub fn forward(&self) -> ForwardMountedSensorModel<'_, '_, '_, F, T> {
        ForwardMountedSensorModel::new(self)
    }

    pub fn inverse(&self) -> InverseMountedSensorModel<'_, '_, '_, F, T> {
        InverseMountedSensorModel::new(self)
    }

    pub fn gradient(&self) -> GradientMountedSensorModel<'_, '_, '_, F, T> {
        GradientMountedSensorModel::new(self)
    }
}

impl<'m, 's, F: Float> Adjustable<F> for MountedSensorModel<'m, 's, F, Beta> {
    fn adjusted(&self, gradient: Parameters<F>) -> Self {
        Self::new(
            self.motor(),
            self.sensor(),
            MountedSensorParameters::new(
                self.parameters().shaft_angle_offset()
                    + ShaftAngle::from_radians_f(gradient.beta_shaft_angle_offset()),
            ),
        )
    }
}

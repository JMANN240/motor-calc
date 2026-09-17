use core::{
    marker::PhantomData,
    ops::{Add, Deref, Div, Mul, Neg, Sub},
};

use num_traits::Float;

use crate::{model::sensor::inverse::InverseSensorModel, types::{sensor_angle::SensorAngle, sensor_type::SensorType}};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Voltage<F: Float, T: SensorType> {
    volts: F,
    sensor_type: PhantomData<T>,
    estimated_sensor_angle: SensorAngle<F, T>,
}

impl<F: Float, T: SensorType> Voltage<F, T> {
    pub fn from_volts_f(volts: F) -> Self {
        Self {
            volts,
            sensor_type: PhantomData,
            estimated_sensor_angle: SensorAngle::from_radians_f(F::infinity()),
        }
    }

    pub fn from_millivolts_u32(millivolts: u32) -> Option<Self> {
        F::from(millivolts).map(|millivolts_f| {
            Self::from_volts_f(
                millivolts_f / F::from(1000).expect("1000 can always be represented with a float"),
            )
        })
    }

    pub fn volts(self) -> F {
        self.volts
    }

    pub fn estimated_sensor_angle(&mut self, inverse_sensor_model: InverseSensorModel<'_, F, T>) -> &mut SensorAngle<F, T> {
        if self.estimated_sensor_angle.radians().is_infinite() {
            self.estimated_sensor_angle = inverse_sensor_model.estimated_sensor_angle(*self)
        }

        &mut self.estimated_sensor_angle
    }
}

impl<F: Float, T: SensorType> Deref for Voltage<F, T> {
    type Target = F;

    fn deref(&self) -> &Self::Target {
        &self.volts
    }
}

impl<F: Float, T: SensorType> Add for Voltage<F, T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_volts_f(self.volts() + rhs.volts())
    }
}

impl<F: Float, T: SensorType> Sub for Voltage<F, T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Voltage::from_volts_f(self.volts() - rhs.volts())
    }
}

impl<F: Float, T: SensorType> Mul<F> for Voltage<F, T> {
    type Output = Self;

    fn mul(self, rhs: F) -> Self::Output {
        Voltage::from_volts_f(self.volts() * rhs)
    }
}

impl<F: Float, T: SensorType> Div<F> for Voltage<F, T> {
    type Output = Self;

    fn div(self, rhs: F) -> Self::Output {
        Voltage::from_volts_f(self.volts() / rhs)
    }
}

impl<F: Float, T: SensorType> Neg for Voltage<F, T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Voltage::from_volts_f(-self.volts())
    }
}

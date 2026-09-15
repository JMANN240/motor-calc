use core::ops::{Add, Deref, Div, Mul, Neg, Sub};

use num_traits::Float;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShaftAngle<F: Float> {
    radians: F,
}

impl<F: Float> ShaftAngle<F> {
    pub fn from_radians_f(radians: F) -> Self {
        Self { radians }
    }

    pub fn from_milliradians_u32(milliradians: u32) -> Option<Self> {
        F::from(milliradians).map(|milliradians_f| {
            Self::from_radians_f(
                milliradians_f
                    / F::from(1000).expect("1000 can always be represented with a float"),
            )
        })
    }

    pub fn radians(self) -> F {
        self.radians
    }
}

impl<F: Float> Deref for ShaftAngle<F> {
    type Target = F;

    fn deref(&self) -> &Self::Target {
        &self.radians
    }
}

impl<F: Float> Add for ShaftAngle<F> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_radians_f(self.radians() + rhs.radians())
    }
}

impl<F: Float> Sub for ShaftAngle<F> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_radians_f(self.radians() - rhs.radians())
    }
}

impl<F: Float> Mul<F> for ShaftAngle<F> {
    type Output = Self;

    fn mul(self, rhs: F) -> Self::Output {
        ShaftAngle::from_radians_f(self.radians() * rhs)
    }
}

impl<F: Float> Div<F> for ShaftAngle<F> {
    type Output = Self;

    fn div(self, rhs: F) -> Self::Output {
        ShaftAngle::from_radians_f(self.radians() / rhs)
    }
}

impl<F: Float> Neg for ShaftAngle<F> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        ShaftAngle::from_radians_f(-self.radians())
    }
}

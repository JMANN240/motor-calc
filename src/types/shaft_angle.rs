use core::ops::{Add, Deref, Div, Mul, Neg, Sub};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShaftAngle {
    radians: f64,
}

impl ShaftAngle {
    pub fn from_radians_f64(radians: f64) -> Self {
        Self { radians }
    }

    pub fn from_milliradians_u32(milliradians: u32) -> Self {
        Self::from_radians_f64(milliradians as f64 / 1000.0)
    }

    pub fn radians(&self) -> f64 {
        self.radians
    }
}

impl Deref for ShaftAngle {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.radians
    }
}

impl Add for ShaftAngle {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_radians_f64(self.radians() + rhs.radians())
    }
}

impl Sub for ShaftAngle {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_radians_f64(self.radians() - rhs.radians())
    }
}

impl Mul<f64> for &ShaftAngle {
    type Output = ShaftAngle;

    fn mul(self, rhs: f64) -> Self::Output {
        ShaftAngle::from_radians_f64(self.radians() * rhs)
    }
}

impl Mul<f64> for ShaftAngle {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        &self * rhs
    }
}

impl Mul<&ShaftAngle> for f64 {
    type Output = ShaftAngle;

    fn mul(self, rhs: &ShaftAngle) -> Self::Output {
        rhs * self
    }
}

impl Mul<ShaftAngle> for f64 {
    type Output = ShaftAngle;

    fn mul(self, rhs: ShaftAngle) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for &ShaftAngle {
    type Output = ShaftAngle;

    fn div(self, rhs: f64) -> Self::Output {
        ShaftAngle::from_radians_f64(self.radians() / rhs)
    }
}

impl Div<f64> for ShaftAngle {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        &self / rhs
    }
}

impl Neg for &ShaftAngle {
    type Output = ShaftAngle;

    fn neg(self) -> Self::Output {
        ShaftAngle::from_radians_f64(-self.radians())
    }
}

impl Neg for ShaftAngle {
    type Output = Self;

    fn neg(self) -> Self::Output {
        -&self
    }
}

use core::ops::{Add, Deref, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShaftAngle {
    angle: f64,
}

impl ShaftAngle {
    pub fn new(angle: f64) -> Self {
        Self { angle }
    }

    pub fn angle(&self) -> f64 {
        self.angle
    }
}

impl Deref for ShaftAngle {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.angle
    }
}

impl Add for ShaftAngle {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.angle() + rhs.angle())
    }
}

impl Sub for ShaftAngle {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.angle() - rhs.angle())
    }
}

impl Mul<f64> for &ShaftAngle {
    type Output = ShaftAngle;

    fn mul(self, rhs: f64) -> Self::Output {
        ShaftAngle::new(self.angle() * rhs)
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
        ShaftAngle::new(self.angle() / rhs)
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
        ShaftAngle::new(-self.angle())
    }
}

impl Neg for ShaftAngle {
    type Output = Self;

    fn neg(self) -> Self::Output {
        -&self
    }
}

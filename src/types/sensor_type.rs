use core::fmt::Debug;

pub trait SensorType: Debug + Clone + Copy + PartialEq + Eq + Default {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Alpha;

impl SensorType for Alpha {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Beta;

impl SensorType for Beta {}

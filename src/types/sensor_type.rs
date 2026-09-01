pub trait SensorType: Default {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Alpha;

impl SensorType for Alpha {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Beta;

impl SensorType for Beta {}

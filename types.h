typedef struct {
    double distance_ratio;
} MotorParameters;

typedef struct {
    MotorParameters parameters;
} MotorModel;

typedef struct {
    double volts;
} Voltage;

typedef struct {
    double voltage_scale;
    Voltage voltage_offset;
} SensorParameters;

typedef struct {
    SensorParameters parameters;
} SensorModel;

typedef struct {
    double radians;
} ShaftAngle;

typedef struct {
    ShaftAngle shaft_angle_offset;
} MountedSensorParameters;

typedef struct {
    MountedSensorParameters mounted_alpha_sensor_parameters;
    MountedSensorParameters mounted_beta_sensor_parameters;
} AssemblyParameters;

typedef struct {
    MotorModel motor;
    SensorModel alpha_sensor;
    SensorModel beta_sensor;
    AssemblyParameters parameters;
} AssemblyModel;

use std::f64::consts::TAU;

use motor_calc::{
    model::{
        assembly::{AssemblyModel, parameters::AssemblyParameters},
        motor::{MotorModel, parameters::MotorParameters},
        mounted_sensor::parameters::MountedSensorParameters,
        sensor::{SensorModel, parameters::SensorParameters},
    },
    types::{
        sensor_type::{Alpha, Beta},
        shaft_angle::ShaftAngle,
        voltage::Voltage,
    },
};
use motor_calc_core::parameters::Parameters;

fn main() {
    let voltages = [
        (1800, 3719),
        (1233, 3551),
        (1122, 2894),
        (1425, 2026),
        (1956, 1503),
        (2494, 1451),
        (2915, 1627),
        (3224, 1937),
        (3379, 2313),
        (3364, 2731),
        (3109, 3213),
        (2514, 3588),
    ]
    .into_iter()
    .map(|(alpha_millivolts, beta_millivolts)| {
        (
            Voltage::<f64, Alpha>::from_millivolts_u32(alpha_millivolts).unwrap(),
            Voltage::<f64, Beta>::from_millivolts_u32(beta_millivolts).unwrap(),
        )
    })
    .collect::<Vec<_>>();

    let mut distance_ratio = 6.75;
    let mut beta_shaft_angle_offset = 2.0 * TAU / 7.0;
    let mut alpha_voltage_scale = 48.0 / TAU;
    let mut alpha_voltage_offset = 2.5;
    let mut beta_voltage_scale = 48.0 / TAU;
    let mut beta_voltage_offset = 2.5;

    let gain = 0.01;

    for i in 0..=10000000 {
        let assembly = AssemblyModel::new(
            MotorModel::new(MotorParameters::new(distance_ratio)),
            SensorModel::new(SensorParameters::new(
                alpha_voltage_scale,
                Voltage::from_volts_f(alpha_voltage_offset),
            )),
            SensorModel::new(SensorParameters::new(
                beta_voltage_scale,
                Voltage::from_volts_f(beta_voltage_offset),
            )),
            AssemblyParameters::new(
                MountedSensorParameters::new(ShaftAngle::from_radians_f(0.0)),
                MountedSensorParameters::new(ShaftAngle::from_radians_f(beta_shaft_angle_offset)),
            ),
        );

        let mut sum_gradient = Parameters::zero();
        let mut sum_of_squared_errors = 0.0;

        for &(alpha_voltage, beta_voltage) in &voltages {
            let grad_squared_error_estimated_displacement = assembly
                .gradient()
                .grad_squared_error_estimated_displacement(alpha_voltage, beta_voltage, 1.0);

            sum_gradient = sum_gradient + grad_squared_error_estimated_displacement;
            sum_of_squared_errors += assembly.inverse().squared_error_estimated_displacement(
                assembly.inverse().error_estimated_displacement(
                    assembly.inverse().estimated_displacement(
                        assembly
                            .alpha_sensor_ref()
                            .inverse()
                            .estimated_sensor_angle(alpha_voltage),
                        assembly
                            .beta_sensor_ref()
                            .inverse()
                            .estimated_sensor_angle(beta_voltage),
                    ),
                    1.0,
                ),
            )
        }

        if i % 1000 == 0 {
            println!("{}", i);
            for &(alpha_voltage, beta_voltage) in &voltages {
                println!(
                    "{:06.2}",
                    100.0
                        * assembly.inverse().estimated_displacement(
                            assembly
                                .alpha_sensor_ref()
                                .inverse()
                                .estimated_sensor_angle(alpha_voltage),
                            assembly
                                .beta_sensor_ref()
                                .inverse()
                                .estimated_sensor_angle(beta_voltage)
                        ),
                );
            }
            println!("sum of squared errors is {}", sum_of_squared_errors);
            println!("distance_ratio:          {}", distance_ratio);
            println!("beta_shaft_angle_offset: {}", beta_shaft_angle_offset);
            println!("alpha_voltage_scale:     {}", alpha_voltage_scale);
            println!("alpha_voltage_offset:    {}", alpha_voltage_offset);
            println!("beta_voltage_scale:      {}", beta_voltage_scale);
            println!("beta_voltage_offset:     {}", beta_voltage_offset);
            println!();
        }

        distance_ratio -= sum_gradient.distance_ratio() * gain;
        beta_shaft_angle_offset -= sum_gradient.beta_shaft_angle_offset() * gain;
        alpha_voltage_scale -= sum_gradient.alpha_voltage_scale() * gain;
        alpha_voltage_offset -= sum_gradient.alpha_voltage_offset() * gain;
        beta_voltage_scale -= sum_gradient.beta_voltage_scale() * gain;
        beta_voltage_offset -= sum_gradient.beta_voltage_offset() * gain;
    }
}

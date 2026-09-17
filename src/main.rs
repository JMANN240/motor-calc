use std::f64::consts::TAU;

use motor_calc::{
    model::{
        Adjustable,
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
    let mut voltages = [
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

    let mut parameters = Parameters::new(6.75, 2.0 * TAU / 7.0, 48.0 / TAU, 2.5, 48.0 / TAU, 2.5);

    let gain = 0.0001;

    for i in 0..=10000000 {
        let mut assembly = AssemblyModel::new(
            MotorModel::new(MotorParameters::new(parameters.distance_ratio())),
            SensorModel::new(SensorParameters::new(
                parameters.alpha_voltage_scale(),
                Voltage::from_volts_f(parameters.alpha_voltage_offset()),
            )),
            SensorModel::new(SensorParameters::new(
                parameters.beta_voltage_scale(),
                Voltage::from_volts_f(parameters.beta_voltage_offset()),
            )),
            AssemblyParameters::new(
                MountedSensorParameters::new(ShaftAngle::from_radians_f(0.0)),
                MountedSensorParameters::new(ShaftAngle::from_radians_f(
                    parameters.beta_shaft_angle_offset(),
                )),
            ),
        );

        let mut sum_of_squared_errors = 0.0;

        for &mut (mut alpha_voltage, mut beta_voltage) in &mut voltages {
            let gradient = assembly
                .gradient()
                .grad_squared_error_estimated_displacement(
                    &mut alpha_voltage,
                    &mut beta_voltage,
                    1.0,
                )
                * gain;
            let negative_gradient = -gradient;
            assembly.adjust(negative_gradient);

            parameters = Parameters::new(
                assembly.motor_ref().parameters().distance_ratio(),
                assembly
                    .parameters()
                    .mounted_beta_sensor_parameters()
                    .shaft_angle_offset()
                    .radians(),
                assembly.alpha_sensor_ref().parameters().voltage_scale(),
                assembly
                    .alpha_sensor_ref()
                    .parameters()
                    .voltage_offset()
                    .volts(),
                assembly.beta_sensor_ref().parameters().voltage_scale(),
                assembly
                    .beta_sensor_ref()
                    .parameters()
                    .voltage_offset()
                    .volts(),
            );

            sum_of_squared_errors += assembly.inverse().squared_error_estimated_displacement(
                assembly.inverse().error_estimated_displacement(
                    assembly.inverse().estimated_displacement(
                        alpha_voltage.estimated_sensor_angle(assembly.alpha_sensor_ref().inverse()),
                        beta_voltage.estimated_sensor_angle(assembly.beta_sensor_ref().inverse()),
                    ),
                    1.0,
                ),
            )
        }

        if i % 1000 == 0 {
            println!("{}", i);
            for &mut (mut alpha_voltage, mut beta_voltage) in &mut voltages {
                println!(
                    "{:06.2}",
                    100.0
                        * assembly.inverse().estimated_displacement(
                            alpha_voltage
                                .estimated_sensor_angle(assembly.alpha_sensor_ref().inverse()),
                            beta_voltage
                                .estimated_sensor_angle(assembly.beta_sensor_ref().inverse())
                        ),
                );
            }
            println!("sum of squared errors is {}", sum_of_squared_errors);
            println!("distance_ratio:          {}", parameters.distance_ratio());
            println!(
                "beta_shaft_angle_offset: {}",
                parameters.beta_shaft_angle_offset()
            );
            println!(
                "alpha_voltage_scale:     {}",
                parameters.alpha_voltage_scale()
            );
            println!(
                "alpha_voltage_offset:    {}",
                parameters.alpha_voltage_offset()
            );
            println!(
                "beta_voltage_scale:      {}",
                parameters.beta_voltage_scale()
            );
            println!(
                "beta_voltage_offset:     {}",
                parameters.beta_voltage_offset()
            );
            println!();
        }
    }
}

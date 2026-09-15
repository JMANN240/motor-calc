use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use motor_calc::model::{Adjustable, assembly::AssemblyModel};
use motor_calc_core::parameters::Parameters;

fn bench_adjust(c: &mut Criterion) {
    let params = Parameters::zero();

    let mut assembly = AssemblyModel::default();

    c.bench_with_input(
        BenchmarkId::new("adjust", 0),
        &params,
        |b, params| {
            b.iter(|| {
                assembly.adjust(params);
            });
        },
    );
}

criterion_group!(benches, bench_adjust,);
criterion_main!(benches);

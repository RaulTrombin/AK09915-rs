use criterion::{criterion_group, criterion_main, Criterion};
use ak09915_rs::Ak09915;
use ak09915_rs::Mode;
use ak09915_rs::Register;
use clap::Parser;
use linux_embedded_hal::I2cdev;


fn navigator_benchmark(c: &mut Criterion) {
    #[macro_export]
    macro_rules! bench {
    ($bench_fn:ident($($arg:tt)*)) => {
        let dev = I2cdev::new("/dev/i2c-1");
        let mut sensor = Ak09915::new(dev.unwrap());
        sensor.set_mode(Mode::Cont200Hz).unwrap();
        c.bench_function(stringify!($bench_fn), |b| b.iter(|| sensor.$bench_fn($($arg)*)));
    }}

    bench!(read_register(Register::ST1));
    bench!(read());
}

criterion_group!(benches, navigator_benchmark);
criterion_main!(benches);
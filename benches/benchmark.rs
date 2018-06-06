#[macro_use]
extern crate criterion;

extern crate geocoder_rs;

use criterion::Criterion;

fn criterion_benchmark(c: &mut Criterion) {
    let _len = geocoder_rs::COUNTRIES.len();
    let coord = geocoder_rs::Point::new(-122.3164206, 37.5506619);
    let coord_slow = geocoder_rs::Point::new(-75.9445286, 37.3511739);
    c.bench_function("find_country_fast", move |b| {
        b.iter(|| geocoder_rs::find_country(&coord))
    });
    c.bench_function("find_country_slow", move |b| {
        b.iter(|| geocoder_rs::find_country(&coord_slow))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn invert_binary_string(input: &String) -> String {
    let mut result = String::new();

    for c in input.chars() {
        if c.eq_ignore_ascii_case(&char::from_digit(1, 2).unwrap()) {
            result = result + "0";
        } else if c.eq_ignore_ascii_case(&char::from_digit(0, 2).unwrap()) {
            result = result + "1";
        }
    }
    result
}

fn invert_binary_string_alternative(input: &String) -> String {
    let num = !u8::from_str_radix(input, 2).unwrap();
    format!("{:b}", num)[3..].to_string()
}

fn invert_binary_string_another(input: &String) -> String {
    String::from_utf8(
        input
            .to_owned()
            .into_bytes()
            .iter()
            .map(|d| d ^ 1)
            .collect(),
    )
    .unwrap()
}

fn criterion_benchmark(c: &mut Criterion) {
    let binary_string: String = "10110".to_string();
    c.bench_with_input(
        BenchmarkId::new("invert_binary_string", &binary_string),
        &binary_string,
        |b, s| {
            b.iter(|| invert_binary_string(&s));
        },
    );

    c.bench_with_input(
        BenchmarkId::new("invert_binary_string_alternative", &binary_string),
        &binary_string,
        |b, s| {
            b.iter(|| invert_binary_string_alternative(&s));
        },
    );

    c.bench_with_input(
        BenchmarkId::new("invert_binary_string_another", &binary_string),
        &binary_string,
        |b, s| {
            b.iter(|| invert_binary_string_another(&s));
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

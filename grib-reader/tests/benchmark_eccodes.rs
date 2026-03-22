mod common;

use common::{
    benchmark_reference, benchmark_rust, collect_parity_samples, helper_path, ReferenceBenchmark,
    RustBenchmark,
};

#[test]
#[ignore = "performance comparison against ecCodes is opt-in"]
fn benchmark_against_eccodes_when_configured() {
    let Some(helper) = helper_path() else {
        eprintln!("skipping ecCodes benchmark test; GRIB_READER_ECCODES_HELPER is not set");
        return;
    };

    let files = collect_parity_samples();
    assert!(
        !files.is_empty(),
        "benchmark corpus is empty; expected bootstrap fixtures at minimum"
    );
    let iterations = std::env::var("GRIB_READER_BENCH_ITERATIONS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(25);

    let rust = benchmark_rust(&files, iterations);
    let reference = benchmark_reference(&helper, &files, iterations);

    assert_benchmark_coverage(&rust, &reference);

    let rust_ms = rust.elapsed.as_secs_f64() * 1_000.0;
    let eccodes_ms = reference.elapsed_ns as f64 / 1_000_000.0;
    let ratio = if eccodes_ms > 0.0 {
        rust_ms / eccodes_ms
    } else {
        f64::INFINITY
    };

    eprintln!(
        "ecCodes benchmark comparison: iterations={} rust_ms={:.3} eccodes_ms={:.3} ratio={:.3} messages={} values={} checksum_delta={:.12}",
        iterations,
        rust_ms,
        eccodes_ms,
        ratio,
        rust.messages,
        rust.values,
        (rust.checksum - reference.checksum).abs(),
    );

    if let Ok(max_ratio) = std::env::var("GRIB_READER_BENCH_MAX_RATIO") {
        let max_ratio = max_ratio
            .parse::<f64>()
            .unwrap_or_else(|err| panic!("invalid GRIB_READER_BENCH_MAX_RATIO: {err}"));
        assert!(
            ratio <= max_ratio,
            "Rust/ecCodes ratio {:.3} exceeded configured maximum {:.3}",
            ratio,
            max_ratio
        );
    }
}

fn assert_benchmark_coverage(rust: &RustBenchmark, reference: &ReferenceBenchmark) {
    assert_eq!(rust.iterations, reference.iterations, "iteration mismatch");
    assert_eq!(rust.messages, reference.messages, "message count mismatch");
    assert_eq!(
        rust.values, reference.values,
        "decoded value count mismatch"
    );

    let checksum_delta = (rust.checksum - reference.checksum).abs();
    let tolerance = rust.checksum.abs().max(reference.checksum.abs()).max(1.0) * 1e-12;
    assert!(
        checksum_delta <= tolerance,
        "checksum mismatch: rust={} eccodes={} delta={} tolerance={}",
        rust.checksum,
        reference.checksum,
        checksum_delta,
        tolerance
    );
}

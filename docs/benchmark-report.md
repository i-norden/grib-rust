# Benchmark Report

Date: 2026-03-21

This document summarizes the current Dockerized reference parity and comparison
benchmark suite for `grib-rust` against ecCodes. The goal is to record the
current parity status and the aggregate decode throughput shape for the current
benchmark corpus.

## System Under Test

- Machine: same local Apple M1 host used for the current `netcdf-rust` report
- CPU topology: 8 logical CPUs
- Memory: 16 GiB
- OS: macOS 13.0
- Architecture: `arm64`
- Rust toolchain: `rustc 1.92.0`
- Reference environment: Docker image with Rust plus `libeccodes-dev`

These numbers are local measurements for this machine. ecCodes ran in Docker,
but the timings still reflect the same host CPU and storage stack.

## Scope

- Dockerized parity tests against ecCodes for:
  - generated GRIB1/GRIB2 fixtures
  - the checked-in GRIB parity sample corpus
- Criterion comparison bench over the full parity sample set collected by
  `collect_parity_samples()`

## Methodology

Commands used for this report:

```sh
./scripts/run-reference-parity.sh
./scripts/run-reference-benchmarks.sh
```

Notes:

- The parity run passed both ecCodes integration tests.
- The current parity corpus contains 4 checked-in GRIB sample files across the
  bootstrap and interoperability sample directories.
- The comparison bench validates message counts, decoded value counts, and
  checksum parity before timing.
- The Criterion benchmark measures aggregate elapsed time for repeated decode of
  the whole current sample set.

## Current Results

### Parity

- `generated_fixtures_match_eccodes_when_configured`: passed
- `corpus_samples_match_eccodes_when_configured`: passed

### Summary

| workload | grib-rust | ecCodes | result |
| --- | ---: | ---: | --- |
| aggregate corpus decode | 1.62 ms | 4.24 ms | `grib-rust` 2.61x faster |

## Interpretation

- On the current 4-file corpus, `grib-rust` is ahead of ecCodes in aggregate
  decode time on this host.
- The current benchmark is intentionally checksum-guarded and corpus-driven, so
  it is a stronger signal than a pure microbenchmark with no correctness check.
- Because the corpus is still small, this result is best read as a control-path
  and current sample-shape comparison, not a definitive statement about every
  GRIB workload.

## Limits

- This report reflects one machine.
- The benchmark corpus is small and should be expanded over time if broader
  throughput claims are needed.
- Docker is appropriate here for ecCodes reproducibility, but the results are
  still host-specific.

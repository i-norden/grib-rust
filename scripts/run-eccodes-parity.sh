#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
image_name="${ECCODES_DOCKER_IMAGE:-grib-rust-eccodes}"

docker build -f "${repo_root}/docker/eccodes.Dockerfile" -t "${image_name}" "${repo_root}"
docker run --rm -v "${repo_root}:/workspace" -w /workspace "${image_name}" bash -lc '
  mkdir -p target
  cc -O2 -Wall -Wextra $(pkg-config --cflags eccodes) tools/eccodes-reference.c -o target/eccodes-reference $(pkg-config --libs eccodes)
  GRIB_READER_ECCODES_HELPER=/workspace/target/eccodes-reference /usr/local/cargo/bin/cargo test -p grib-reader --test parity_eccodes --all-features
'

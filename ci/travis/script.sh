#!/bin/bash
cargo build --examples
LD_LIBRARY_PATH="${VIRTUAL_ENV}"/lib carg run --examples test

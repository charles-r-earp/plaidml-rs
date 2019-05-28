#!/bin/bash
cargo build --examples
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$VIRTUAL_ENV/lib
cargo run --examples test

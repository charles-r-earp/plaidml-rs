# plaidml-rs
Rust bindings for PlaidML

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/charles-r-earp/plaidml-rs/LICENSE)

[![Build Status](https://travis-ci.org/charles-r-earp/plaidml-rs.svg?branch=master)](https://travis-ci.org/charles-r-earp/plaidml-rs)

## Status
In construction, not in a useful state. See examples/test.rs 

# Installing
Install PlaidML via pip in a virtualenv called plaidml

### Ubuntu
    sudo apt update
    sudo apt install python3 python3-pip
    sudo pip3 install virtualenv
    virtualenv plaidml
    source plaidml/bin/activate
    pip install plaidml

## Setup
Choose which accelerator to use:

    plaidml-setup 

## Cargo
Add the following to cargo.toml:

    [dependencies]
    plaidml = { git = "https://github.com/charles-r-earp/plaidml-rs" }

# Usage

## Unix Systems ie Linux, Mac
Currently Cargo doesn't set rpath correctly so the environmental variable LD_LIBRARY_PATH  must be passed in, either when running plaidml-rs directly, or in a dependent crate.

    LD_LIBRARY_PATH=$VIRTUAL_ENV/lib cargo run --example test
    
Alternatively, set add this to ~/.bashrc:

    export $LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$HOME/plaidml/lib
    
## Windows

    PATH=$VIRTUAL_ENV/lib cargo run --example test
    
# Tested On
- Ubuntu
  - 16.04 LTS 'Xenial'
    - stable
  - 18.04 LTS 'Bionic Beaver'
    - stable

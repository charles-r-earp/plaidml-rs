## plaidml-rs
Rust bindings for PlaidML

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/charles-r-earp/plaidml-rs/LICENSE)

[![Build Status](https://travis-ci.org/charles-r-earp/plaidml-rs.svg?branch=master)](https://travis-ci.org/charles-r-earp/plaidml-rs)

## Installing
Install PlaidML via pip in a virtualenv called plaidml

# Ubuntu
    sudo apt update
    sudo apt install python3 python3-pip
    sudo pip3 install virtualenv
    virtualenv plaidml
    source plaidml/bin/activate
    pip install plaidml

# Setup
Choose which accelerator to use:

    plaidml-setup 

# Cargo
Add the following to cargo.toml:

    plaidml = { git = "https://github.com/charles-r-earp/plaidml-rs" }

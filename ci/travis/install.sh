#!/bin/bash
echo $VIRTUAL_ENV
sudo apt update
sudo apt install -y python3-pip
sudo -H pip3 install virtualenv
virtualenv plaidml
source plaidml/bin/activate
pip install plaidml
source ~/.cargo/env || true

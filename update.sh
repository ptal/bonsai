#!/bin/sh

cargo install --force --path .
cd runtime/
./install.sh
cd ..
# cd libstd/
# ./install.sh
# cd ..

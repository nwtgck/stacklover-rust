#!/bin/bash
set -xeu

cd $(dirname "$0")

for bin_name in example1
do
  cargo expand --bin ${bin_name} > ./src/bin/${bin_name}.expanded.rs
done

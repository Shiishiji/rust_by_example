#!/usr/bin/env sh

echo "Compiling rary.rs with rustc.."
rustc --crate-type=lib rary.rs
echo "Finished"
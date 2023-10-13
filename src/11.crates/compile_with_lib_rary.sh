#!/usr/bin/env sh

echo "Compiling rary.rs with rustc.."
rustc using_lib_rary.rs --extern rary=lib_rary/library.rlib
echo "Finished"
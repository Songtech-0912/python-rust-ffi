# Python-Rust FFI demo

This repository is a simple example of integrating Python bindings with an existing Rust library. The repository consists of the following structure:

- `vmath/`, a basic Rust library that implements a 2d vector type
- `py-vmath/`, a Python wrapper around `vmath`

The Rust library is designed to be written as a typical Rust library with no considerations for implementing FFI, while the python bindings are designed to resemble a typical Python library with no trace of the underlying bindings.

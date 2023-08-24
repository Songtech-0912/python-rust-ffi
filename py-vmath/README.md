# Py-vmath

This folder contains the source code of two libraries:

- `vmath_internal`, the system-level Python bindings to the main Rust library
- `vmath`, the high-level user-facing Python API

To develop, download [maturin](https://www.maturin.rs/) and [just](https://just.systems) and run:

```sh
just mkvenv
just build
```

Then open a Python console and run:

```py
import vmath

# do what you want...
```

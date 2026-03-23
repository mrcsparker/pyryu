# PyRyu

PyRyu is a Rust-powered Python library that provides efficient conversion of floating point numbers to their shortest string representation using the Ryu algorithm. It pairs a tiny Python API with a native Rust implementation built through PyO3 and maturin.

## Features

- Fast conversion of floating point numbers to strings.
- Simple Python interface to a Rust library.
- Clear handling of edge cases like NaN and infinity.
- Module-level helpers for both `f32` and `f64` formatting.
- Optional native Ryu rendering for `NaN`, `inf`, and `-inf`.
- `uv`-native development workflow for syncing, testing, and building.

## Prerequisites

Before you begin, ensure you have met the following requirements:
- Rust Toolchain (latest stable recommended)
- Python 3.8 or higher
- `uv` for Python environment and dependency management

## Installation

Follow these steps to install PyRyu:

1. Clone the repository:
   ```bash
   git clone https://github.com/mrcsparker/pyryu.git
   cd pyryu
   ```

2. Build the project:
   ```bash
   uv sync
   uv run maturin build
   uv run pytest
   ```

These commands create a local virtual environment, install the project and development tools, build the extension module, and run the test suite.

## Usage

PyRyu supports both a stateful class API and stateless module-level helpers.

### Basic class usage

```python
from pyryu import PyRyu

ryu = PyRyu()
print(ryu.format(3.14159))      # "3.14159"
print(ryu.format_finite(1.25))  # "1.25"
```

Use this when you want to keep a `PyRyu` instance around and format many values.

### Stateless helper functions

```python
from pyryu import format_f32, format_f64

print(format_f64(3.14159))  # "3.14159"
print(format_f32(1.5))      # "1.5"
```

Use these when you just want a one-off formatting call without creating a class instance.

### Handling `NaN` and infinity

Checked APIs raise `ValueError` for non-finite values:

```python
from pyryu import format_f64

try:
    format_f64(float("inf"))
except ValueError as exc:
    print(exc)
    # Cannot format NaN or infinite values using `format`.
```

Native APIs preserve upstream Ryu behavior:

```python
from pyryu import PyRyu, format_native_f32, format_native_f64

ryu = PyRyu()

print(ryu.format_native(float("nan")))   # "NaN"
print(format_native_f64(float("inf")))   # "inf"
print(format_native_f32(float("-inf")))  # "-inf"
```

### Using `format_finite` when input is already validated

```python
from pyryu import PyRyu
import math

value = 123.456
ryu = PyRyu()

if math.isfinite(value):
    print(ryu.format_finite(value))  # "123.456"
```

Use `format_finite` when you already know the input is finite and want the explicit finite-only path.

PyRyu supports Python 3.8+ and builds cleanly on Python 3.13 with the current PyO3 toolchain.

## Development

To set up a development environment for this project:

1. Install dependencies:
   ```bash
   uv sync
   ```

2. Build the extension into the synced environment:
   ```bash
   uv run maturin develop
   ```

3. Run the tests:
   ```bash
   uv run pytest -q
   ```

## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

To contribute to PyRyu, follow these steps:

1. Fork the repository.
2. Create your feature branch (`git checkout -b feature/AmazingFeature`).
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`).
4. Push to the branch (`git push origin feature/AmazingFeature`).
5. Open a pull request.

## License

Distributed under the MIT License. See `LICENSE` for more information.

## Contact

Project Link: [https://github.com/mrcsparker/pyryu](https://github.com/mrcsparker/pyryu)

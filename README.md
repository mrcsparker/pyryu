# PyRyu

PyRyu is a Rust-powered Python library that provides efficient conversion of floating point numbers to their shortest string representation using the Ryu algorithm. This project demonstrates how to build Python bindings using PyO3.

## Features

- Fast conversion of floating point numbers to strings.
- Simple Python interface to a Rust library.
- Handles edge cases like NaN and infinity.

## Prerequisites

Before you begin, ensure you have met the following requirements:
- Rust Toolchain (latest stable recommended)
- Python 3.7 or higher
- Poetry for Python dependency management

## Installation

Follow these steps to install PyRyu:

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/pyryu.git
   cd pyryu
   ```

2. Build the project:
   ```bash
   poetry install
   poetry run maturin build
   poetry run pytest
   ```

This command builds the Rust code and installs the Python package into a virtual environment managed by Poetry.

## Usage

Here is a simple example of how to use PyRyu in a Python script:

```python
from pyryu import PyRyu

ryu = PyRyu()
number = 3.14159
print("Formatted number:", ryu.format(number))
```

This script creates an instance of `PyRyu` and formats a floating point number.

## Development

To set up a development environment for this project:

1. Install dependencies:
   ```bash
   poetry install
   ```

2. To build and test locally:
   ```bash
   poetry run pytest
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

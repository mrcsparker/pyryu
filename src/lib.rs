use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

/// This class provides an interface to the Ryu library, which efficiently converts floating point numbers to strings.
#[pyclass(name = "PyRyu")]
struct PyRyu {
    buffer: ryu::Buffer,
}

#[pymethods]
impl PyRyu {
    /// Creates a new instance of PyRyu.
    #[new]
    fn new() -> Self {
        PyRyu {
            buffer: ryu::Buffer::new(),
        }
    }

    /// Formats a floating point number to the shortest possible string representation.
    ///
    /// Arguments:
    ///     value: The floating point number to format.
    ///
    /// Returns:
    ///     The shortest string representation of the floating point number.
    fn format(&mut self, value: f64) -> PyResult<String> {
        if value.is_nan() || value.is_infinite() {
            Err(PyValueError::new_err(
                "Cannot format NaN or infinite values using `format`. Use `format_finite` instead.",
            ))
        } else {
            Ok(self.buffer.format(value).to_string())
        }
    }

    /// Formats a finite floating point number to the shortest possible string representation.
    ///
    /// Arguments:
    ///     value: The floating point number to format (must be finite).
    ///
    /// Returns:
    ///     The shortest string representation of the finite floating point number.
    ///     If the number is NaN or infinite, raises a ValueError.
    fn format_finite(&mut self, value: f64) -> PyResult<String> {
        if value.is_nan() || value.is_infinite() {
            Err(PyValueError::new_err(
                "Cannot format NaN or infinite values.",
            ))
        } else {
            Ok(self.buffer.format_finite(value).to_string())
        }
    }
}

#[pymodule]
fn pyryu(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyRyu>()?;
    Ok(())
}

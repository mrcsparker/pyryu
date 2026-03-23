use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

fn format_f64_checked_with(buffer: &mut ryu::Buffer, value: f64) -> PyResult<String> {
    if value.is_nan() || value.is_infinite() {
        Err(PyValueError::new_err(
            "Cannot format NaN or infinite values using `format`. Use `format_native` instead.",
        ))
    } else {
        Ok(buffer.format(value).to_string())
    }
}

fn format_f32_checked(value: f32) -> PyResult<String> {
    if value.is_nan() || value.is_infinite() {
        Err(PyValueError::new_err(
            "Cannot format NaN or infinite values using `format_f32`. Use `format_native_f32` instead.",
        ))
    } else {
        Ok(ryu::Buffer::new().format(value).to_string())
    }
}

fn format_f64_finite_with(buffer: &mut ryu::Buffer, value: f64) -> PyResult<String> {
    if value.is_nan() || value.is_infinite() {
        Err(PyValueError::new_err(
            "Cannot format NaN or infinite values.",
        ))
    } else {
        Ok(buffer.format_finite(value).to_string())
    }
}

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
        format_f64_checked_with(&mut self.buffer, value)
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
        format_f64_finite_with(&mut self.buffer, value)
    }

    /// Formats a floating point number using ryu's native non-finite rendering.
    ///
    /// This method returns "NaN", "inf", or "-inf" for non-finite values instead
    /// of raising an error.
    fn format_native(&mut self, value: f64) -> String {
        self.buffer.format(value).to_string()
    }
}

/// Format an f64 and reject NaN and infinite values.
#[pyfunction]
fn format_f64(value: f64) -> PyResult<String> {
    let mut buffer = ryu::Buffer::new();
    format_f64_checked_with(&mut buffer, value)
}

/// Format an f32 and reject NaN and infinite values.
#[pyfunction]
fn format_f32(value: f32) -> PyResult<String> {
    format_f32_checked(value)
}

/// Format an f64 using ryu's native non-finite rendering.
#[pyfunction]
fn format_native_f64(value: f64) -> String {
    ryu::Buffer::new().format(value).to_string()
}

/// Format an f32 using ryu's native non-finite rendering.
#[pyfunction]
fn format_native_f32(value: f32) -> String {
    ryu::Buffer::new().format(value).to_string()
}

#[pymodule]
fn pyryu(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyRyu>()?;
    m.add_function(wrap_pyfunction!(format_f64, m)?)?;
    m.add_function(wrap_pyfunction!(format_f32, m)?)?;
    m.add_function(wrap_pyfunction!(format_native_f64, m)?)?;
    m.add_function(wrap_pyfunction!(format_native_f32, m)?)?;
    Ok(())
}

import pytest
from pyryu import PyRyu, format_f32, format_f64, format_native_f32, format_native_f64

def test_new_instance():
    """Test creating a new PyRyu instance."""
    ryu = PyRyu()
    assert ryu is not None

def test_format_valid_number():
    """Test formatting a valid floating point number."""
    ryu = PyRyu()
    assert ryu.format(3.14159) == '3.14159'

def test_format_nan():
    """Test formatting NaN values, which should raise an error."""
    ryu = PyRyu()
    with pytest.raises(ValueError) as exc_info:
        ryu.format(float('nan'))
    assert "Cannot format NaN or infinite values using `format`." in str(exc_info.value)

def test_format_infinite():
    """Test formatting infinite values, which should also raise an error."""
    ryu = PyRyu()
    with pytest.raises(ValueError) as exc_info:
        ryu.format(float('inf'))
    assert "Cannot format NaN or infinite values using `format`." in str(exc_info.value)

def test_format_finite_nan():
    """Test the format_finite method with NaN, expecting an error."""
    ryu = PyRyu()
    with pytest.raises(ValueError):
        ryu.format_finite(float('nan'))

def test_format_finite_infinite():
    """Test the format_finite method with infinity, expecting an error."""
    ryu = PyRyu()
    with pytest.raises(ValueError):
        ryu.format_finite(float('inf'))

def test_format_finite_valid_number():
    """Test formatting a finite number."""
    ryu = PyRyu()
    result = ryu.format_finite(123.456)
    assert result == '123.456'

def test_format_native_uses_ryu_nonfinite_strings():
    """Test native formatting preserves ryu's non-finite strings."""
    ryu = PyRyu()
    assert ryu.format_native(float('nan')) == 'NaN'
    assert ryu.format_native(float('inf')) == 'inf'
    assert ryu.format_native(float('-inf')) == '-inf'

def test_module_level_format_f64():
    """Test the convenience formatter for f64 values."""
    assert format_f64(3.14159) == '3.14159'

def test_module_level_format_f32():
    """Test the convenience formatter for f32 values."""
    assert format_f32(1.5) == '1.5'

def test_module_level_format_rejects_nonfinite():
    """Test module-level formatters reject non-finite inputs by default."""
    with pytest.raises(ValueError):
        format_f64(float('nan'))
    with pytest.raises(ValueError):
        format_f32(float('inf'))

def test_module_level_native_formatters():
    """Test native module-level formatters expose ryu's special values."""
    assert format_native_f64(float('nan')) == 'NaN'
    assert format_native_f64(float('inf')) == 'inf'
    assert format_native_f32(float('-inf')) == '-inf'

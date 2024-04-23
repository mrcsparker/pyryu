import pytest
from pyryu import PyRyu

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

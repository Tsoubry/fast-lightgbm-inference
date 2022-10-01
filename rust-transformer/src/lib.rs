use numpy::convert::ToPyArray;
use numpy::{IntoPyArray, PyArrayDyn, PyReadonlyArrayDyn};
use pyo3::types::{PyModule, PyTuple, PyUnicode};
use pyo3::{pymodule, Py, PyAny, PyObject, PyResult, Python};

use feature_pipe::transformers::{long_to_f64, string_to_f64};

#[pymodule]
#[pyo3(name = "_odt")]
fn rust_ext(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    #[pyfn(m)]
    #[pyo3(name = "transform_i64")]
    fn transform_i64<'py>(py: Python<'py>, x: PyReadonlyArrayDyn<i64>) -> &'py PyArrayDyn<f64> {
        x.as_array().map(|i| long_to_f64(*i)).into_pyarray(py)
    }

    #[pyfn(m)]
    #[pyo3(name = "transform_object")]
    fn transform_object<'py>(
        py: Python<'py>,
        x: PyReadonlyArrayDyn<PyObject>,
    ) -> &'py PyArrayDyn<f64> {
        x.as_array()
            .map(|i| string_to_f64(i.cast_as::<PyUnicode>(py).unwrap().to_str().unwrap()))
            .into_pyarray(py)
    }

    fn check_null<'py>(py: Python<'py>, data: &'py Py<PyAny>) -> (f64, f64) {
        if data.is_none(py) {
            (0.0, 0.0)
        } else {
            (
                1.0,
                string_to_f64(data.cast_as::<PyUnicode>(py).unwrap().to_str().unwrap()),
            )
        }
    }

    #[pyfn(m)]
    #[pyo3(name = "transform_nullable_object")]
    fn transform_nullable_object<'py>(
        py: Python<'py>,
        x: PyReadonlyArrayDyn<PyObject>,
    ) -> &'py PyTuple {
        let result = x.as_array().map(|i| check_null(py, i));

        let nullables = result.map(|i| i.0).to_pyarray(py);
        let converted = result.map(|i| i.1).to_pyarray(py);

        PyTuple::new(py, vec![nullables, converted])
    }

    Ok(())
}

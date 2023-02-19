use std::io::Result;
use std::{fs::read_dir, path::Path};

use humansize::{format_size, BINARY};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

mod highlight;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

fn get_size(path: &Path) -> Result<u64> {
    let mut total_size = 0;
    let path_metadata = path.symlink_metadata()?;

    if path_metadata.is_dir() {
        for entry in read_dir(&path)? {
            let entry = entry?;
            let entry_metadata = entry.metadata()?;

            if entry_metadata.is_dir() {
                total_size += get_size(&entry.path())?;
            } else {
                total_size += entry_metadata.len();
            }
        }
    } else {
        total_size = path_metadata.len();
    }

    Ok(total_size)
}

fn format(size: u64) -> String {
    format_size(size, BINARY)
}

#[pyfunction]
fn get_directory_size(path: String) -> PyResult<String> {
    let p = Path::new(&path);
    let result = match get_size(&p) {
        Ok(value) => Ok(value),
        Err(e) => Err(PyValueError::new_err(e.to_string())),
    }?;
    Ok(format(result))
}

#[pyfunction]
fn code_highlight(code: String) -> PyResult<String> {
    match highlight::code_highlight(code) {
        Ok(value) => Ok(value),
        Err(e) => Err(PyValueError::new_err(e.to_string())),
    }
}

#[pymodule]
fn tobira(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(get_directory_size, m)?)?;
    m.add_function(wrap_pyfunction!(code_highlight, m)?)?;
    Ok(())
}

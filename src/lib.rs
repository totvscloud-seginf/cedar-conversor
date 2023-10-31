use pyo3::prelude::*;

/// Echo (return) the input string
#[pyfunction]
#[pyo3(signature = (s))]
fn echo(s: String) -> PyResult<String> {
    Ok(s)
}

#[pyfunction]
#[pyo3(signature = (policies))]
fn convert_policies_from_json(policies: String) -> PyResult<String> {
    let policies_json = serde_json::from_str(policies.as_str()).unwrap();
    let conditions = cedar_policy::Policy::from_json(None, policies_json);
    match conditions {
        Ok(conditions) => Ok(conditions.to_string()),
        Err(e) => {
            println!("❌ error converting conditions from json: {:?}", e);
            Err(pyo3::exceptions::PyValueError::new_err(e.to_string()))
        }
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn _internal(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(echo, m)?)?;
    m.add_function(wrap_pyfunction!(convert_policies_from_json, m)?)?;
    Ok(())
}

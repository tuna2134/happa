use pyo3::{exceptions::PyValueError, prelude::*};

#[pyclass]
pub struct Message {
    pub msg: tungstenite::Message,
}

#[pymethods]
impl Message {
    fn is_text(&self) -> PyResult<bool> {
        Ok(self.msg.is_text())
    }

    fn is_binary(&self) -> PyResult<bool> {
        Ok(self.msg.is_binary())
    }

    fn text(&self) -> PyResult<&str> {
        if self.msg.is_text() {
            Ok(self.msg.to_text().unwrap())
        } else {
            Err(PyValueError::new_err("Not text message"))
        }
    }

    // fn binary(&mut self) -> PyResult<Vec<u8>> {
    // if self.msg.is_binary() {
    // let data = self.msg.into_data();
    // Ok(data)
    // } else {
    // Err(PyValueError::new_err("Not binary message"))
    // }
    // }
}

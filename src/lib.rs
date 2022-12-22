use pyo3::prelude::*;
use std::net::TcpListener;
use tungstenite::accept;

mod message;
mod stream;

#[pyclass]
struct Server {
    func: PyObject,
}

#[pymethods]
impl Server {
    #[new]
    fn new(func: PyObject) -> Self {
        Server { func }
    }

    fn start(&mut self, py: Python, addr: &str) -> PyResult<()> {
        let listener = TcpListener::bind(addr).unwrap();
        for stream in listener.incoming() {
            let func = self.func.clone_ref(py);
            let ws = accept(stream.unwrap()).unwrap();
            let wsstream = stream::WebsocketStream { ws };
            func.call1(py, (wsstream,)).unwrap();
        }
        Ok(())
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn happa(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Server>()?;
    m.add_class::<stream::WebsocketStream>()?;
    m.add_class::<message::Message>()?;
    Ok(())
}

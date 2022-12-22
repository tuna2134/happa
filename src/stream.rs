use pyo3::prelude::*;
use std::net::TcpStream;
use tungstenite::protocol::WebSocket;

#[derive(FromPyObject)]
enum Message {
    #[pyo3(transparent, annotation = "str")]
    Text(String),
    #[pyo3(transparent, annotation = "bytes")]
    Binary(Vec<u8>),
}

#[pyclass]
pub struct WebsocketStream {
    pub ws: WebSocket<TcpStream>,
}

#[pymethods]
impl WebsocketStream {
    fn send(&mut self, msg: Message) -> PyResult<()> {
        let message = match msg {
            Message::Text(text) => tungstenite::Message::Text(text),
            Message::Binary(binary) => tungstenite::Message::Binary(binary),
        };
        self.ws.write_message(message).unwrap();
        Ok(())
    }

    fn recv(&mut self) -> PyResult<crate::message::Message> {
        Ok(crate::message::Message {
            msg: self.ws.read_message().unwrap().clone(),
        })
    }

    fn recv_text(&mut self) -> PyResult<String> {
        let msg = self.ws.read_message().unwrap();
        if msg.is_text() {
            let data = msg.to_text().unwrap().to_string();
            Ok(data)
        } else {
            Err(pyo3::exceptions::PyValueError::new_err("Not text message"))
        }
    }
}

use super::Credentials;
use super::{Request, TransportError};
use http::{Request as HttpRequest, Uri};
use log::{debug, error, trace};
use std::borrow::Cow;
use std::str::FromStr;
use thiserror::Error;
use tungstenite::client::AutoStream;
use tungstenite::protocol::frame::coding::CloseCode;
use tungstenite::protocol::CloseFrame;
use tungstenite::{connect as ws_connect, Message, WebSocket as WebSocketTungstenite};

/// Wraps a websocket connection
pub struct WebSocket {
    /// The endpoint of the websocket connection
    pub address: String,
    pub credentials: Option<Credentials>,
    ws: WebSocketTungstenite<AutoStream>,
}

impl WebSocket {
    pub(crate) fn new(
        address: String,
        credentials: Option<Credentials>,
    ) -> Result<WebSocket, WebSocketError> {
        debug!("Initiating websocket connection to {}", address);
        let uri = Uri::from_str(&address)?;
        let handshake_request = create_handshake_request(&uri, credentials.clone())?;
        let ws = ws_connect(handshake_request)?;
        trace!("Handshake Response: {:?}", ws.1);
        Ok(WebSocket {
            address,
            credentials,
            ws: ws.0,
        })
    }

    pub(crate) fn close(&mut self) -> Result<(), WebSocketError> {
        debug!("Closing websocket connection");
        let close_frame = CloseFrame {
            code: CloseCode::Normal,
            reason: Cow::from("Finished"),
        };
        self.ws.close(Some(close_frame))?;
        self.ws.write_pending().map_err(WebSocketError::from)
    }

    pub(crate) fn read_message(&mut self) -> Result<String, WebSocketError> {
        match self.read() {
            Ok(Message::Text(response)) => Ok(response),
            Ok(_) => self.read_message(),
            Err(err) => Err(err),
        }
    }

    fn read(&mut self) -> Result<Message, WebSocketError> {
        let message = self.ws.read_message()?;
        trace!("Reading from websocket: {}", &message);
        Ok(message)
    }

    fn write(&mut self, message: Message) -> Result<(), WebSocketError> {
        trace!("Writing to websocket: {}", message);
        self.ws.write_message(message)?;
        Ok(())
    }
}

impl Request for WebSocket {
    fn request(&mut self, cmd: String) -> Result<String, TransportError> {
        let _write = self.write(Message::Text(cmd))?;
        self.read_message().map_err(TransportError::from)
    }
}

impl Drop for WebSocket {
    fn drop(&mut self) {
        let close = self.close();
        if let Err(err) = close {
            error!("{}", err);
        }
    }
}

fn create_handshake_request(
    uri: &Uri,
    credentials: Option<Credentials>,
) -> Result<HttpRequest<()>, WebSocketError> {
    let mut req_builder = HttpRequest::get(uri);
    if let Some(credentials) = credentials {
        let auth_string_base64 = String::from("Basic ")
            + &base64::encode(credentials.username + ":" + &credentials.password);
        let headers = req_builder.headers_mut().ok_or(WebSocketError::Handshake)?;
        headers.insert("Authorization", auth_string_base64.parse()?);
    }
    let request = req_builder.body(())?;
    trace!("Built websocket handshake request: {:?}", &request);
    Ok(request)
}

/// An error type collecting what can go wrong with a websocket
#[derive(Debug, Error)]
pub enum WebSocketError {
    #[error("WebSocketError: {0}")]
    Tungstenite(#[from] tungstenite::Error),
    #[error("WebSocketError: {0}")]
    Http(#[from] http::Error),
    #[error("WebSocketError: {0}")]
    Url(#[from] http::uri::InvalidUri),
    #[error("WebSocketError: HandshakeError")]
    Handshake,
    #[error("WebSocketError: {0}")]
    InvalidHeader(#[from] http::header::InvalidHeaderValue),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{SocketAddr, TcpStream};
    use tungstenite::{accept, Message};

    fn spawn_websocket_server<F>(mut handle_ws_stream: F, port: u16)
    where
        F: FnMut(&mut WebSocketTungstenite<TcpStream>) + Send + 'static,
    {
        let tcp_listener =
            std::net::TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], port))).unwrap();

        let _thread = std::thread::Builder::new()
            .name("Websocket Server".to_string())
            .spawn(move || loop {
                match tcp_listener.accept() {
                    Ok((tcp_stream, _address)) => match accept(tcp_stream) {
                        Ok(mut websocket) => handle_ws_stream(&mut websocket),
                        Err(err) => panic!("{}", err),
                    },
                    Err(err) => panic!("{}", err),
                }
            })
            .is_ok();
    }

    fn ping_pong(ws_stream: &mut WebSocketTungstenite<TcpStream>) {
        match ws_stream.read_message() {
            Ok(message) => match message {
                Message::Text(echo) => ws_stream
                    .write_message(Message::Text(echo + " Pong"))
                    .unwrap(),
                _ => panic!("Received other message type."),
            },
            Err(err) => panic!(err),
        }
    }

    #[test]
    fn test_create_handshake_request_with_credentials() {
        let uri = Uri::from_static("localhost");
        let credentials = Credentials {
            username: String::from("abc"),
            password: String::from("123"),
        };
        let request = create_handshake_request(&uri, Some(credentials)).unwrap();
        assert_eq!(
            request.headers().get("Authorization").unwrap(),
            "Basic YWJjOjEyMw=="
        );
    }

    #[test]
    fn test_create_handshake_request_without_credentials() {
        let uri = Uri::from_static("localhost");
        let request = create_handshake_request(&uri, None).unwrap();
        assert_eq!(request.method(), http::method::Method::GET);
        assert_eq!(request.uri(), &uri);
    }

    #[test]
    fn test_new() {
        spawn_websocket_server(ping_pong, 3001);
        let mut ws_client = WebSocket::new(String::from("ws://localhost:3001"), None).unwrap();
        ws_client
            .write(Message::Text(String::from("Ping")))
            .unwrap();
        match ws_client.read() {
            Ok(Message::Text(text)) => assert_eq!(text, "Ping Pong"),
            _ => assert!(false),
        };
    }
}
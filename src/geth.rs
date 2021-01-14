use crate::rpc::{Call, Response, Rpc};
use crate::transport::ws::{WebSocket, WebSocketError};
use crate::transport::Request;
use crate::Credentials;
use serde::de::DeserializeOwned;
use std::error::Error;
use std::fmt::Debug;

pub struct GethConnector<T: Request>(T);

impl GethConnector<WebSocket> {
    pub fn ws(domain: &str, credentials: Option<Credentials>) -> Result<Self, WebSocketError> {
        Ok(GethConnector(WebSocket::new(domain, credentials)?))
    }

    pub fn close(&mut self) -> Result<(), WebSocketError> {
        self.0.close()
    }
}

impl<T: Request> Call for GethConnector<T> {
    fn call<U: DeserializeOwned + Debug>(&mut self, rpc: Rpc<U>) -> Result<U, Box<dyn Error>> {
        let response = self.0.request(rpc.command)?;
        match serde_json::from_str::<Response<U>>(&response) {
            Ok(Response {
                error: Some(err), ..
            }) => Err(Box::from(err)),
            Ok(other) => Ok(other.result),
            Err(err) => Err(Box::new(err)),
        }
    }
}

#[cfg(test)]
mod tests;

pub mod response;

pub use client::response::*;
use std::io::Read;
use message::Message;
use hyper::client::Client as HttpClient;
use hyper::header::{Authorization, ContentType, Connection};
use hyper::status::StatusCode;
use rustc_serialize::json::{self, ToJson};
use std::env;

pub struct Client {
    http_client: HttpClient,
    fcm_uri: String,
}

impl Client {
    /// Get a new instance of Client.
    pub fn new() -> Client {
        let fcm_uri = match env::var("FCM_URI") {
            Ok(val) => String::from(val),
            Err(_) => String::from("https://fcm.googleapis.com/fcm/send")
        };

        Client {
            http_client: HttpClient::new(),
            fcm_uri: fcm_uri,
        }
    }

    /// Send a message using your FCM API Key.
    /// # Examples:
    /// ```no_run
    /// use fcm::{MessageBuilder, Client};
    /// use std::collections::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("message", "Howdy!");
    ///
    /// let message = MessageBuilder::new("<registration id>").data(map).finalize();
    /// let client = Client::new();
    /// let result = client.send(message, "<FCM API Key>");
    /// ```
    pub fn send(&self, message: Message, api_key: &str) -> Result<FcmResponse, FcmError> {
        let payload = message.to_json().to_string();
        let auth_header = format!("key={}", api_key);

        let response = self.http_client.
            post(&self.fcm_uri).
            header(Connection::keep_alive()).
            header(ContentType::json()).
            body(&payload).
            header(Authorization(auth_header)).
            header(ContentType::json()).
            send();

        match response {
            Ok(mut response) => {
                let mut body = String::new();
                response.read_to_string(&mut body).unwrap();

                Client::parse_response(response.status, &body)
            },
            Err(_) => {
                Client::parse_response(StatusCode::InternalServerError, "Server Error")
            }
        }
    }

    fn parse_response(status: StatusCode, body: &str) -> Result<response::FcmResponse, response::FcmError> {
        use hyper::status::StatusCode::*;
        match status {
            Ok =>
                Result::Ok(json::decode(body).unwrap()),
            Unauthorized =>
                Err(response::FcmError::Unauthorized),
            BadRequest =>
                Err(response::FcmError::InvalidMessage(body.to_string())),
            status if status.is_server_error() =>
                Err(response::FcmError::ServerError),
            _ =>
                Err(response::FcmError::InvalidMessage("Unknown Error".to_string())),
        }
    }
}

pub mod response;

pub use client::response::*;
use std::str;
use std::fmt;
use message::Message;
use hyper::client::Client as HttpClient;
use hyper::client::{Request, Response};
use hyper_tls::HttpsConnector;
use hyper::header::{Authorization, ContentType, ContentLength, RetryAfter};
use hyper::Post;
use hyper::status::StatusCode;
use rustc_serialize::json::{self, ToJson};
use futures::{Future, Poll};
use futures::future::{ok, err};
use futures::stream::Stream;
use tokio_core::reactor::Handle;
pub use tokio_service::Service;

pub struct Client {
    http_client: HttpClient<HttpsConnector>,
}

pub struct FutureResponse(Box<Future<Item=FcmResponse, Error=FcmError> + 'static>);

impl fmt::Debug for FutureResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad("Future<FcmResponse>")
    }
}

impl Future for FutureResponse {
    type Item = FcmResponse;
    type Error = FcmError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.0.poll()
    }
}

impl Client {
    /// Get a new instance of Client.
    pub fn new(handle: &Handle) -> Client {
        let http_client = HttpClient::configure()
            .connector(HttpsConnector::new(1, handle))
            .keep_alive(true)
            .build(handle);

        Client {
            http_client: http_client,
        }
    }

    #[inline]
    pub fn send(&self, message: Message) -> FutureResponse {
        self.call(message)
    }
}

impl Service for Client {
    type Request = Message;
    type Response = FcmResponse;
    type Error = response::FcmError;
    type Future = FutureResponse;

    fn call(&self, message: Self::Request) -> Self::Future {
        let payload = message.to_json().to_string();
        let auth_header = format!("key={}", message.api_key);

        let mut request = Request::new(Post, "https://fcm.googleapis.com/fcm/send".parse().unwrap());
        request.headers_mut().set(ContentType::json());
        request.headers_mut().set(Authorization(auth_header));
        request.set_body(payload.into_bytes());

        let request_f = self.http_client.request(request).map_err(|_| { response::FcmError::ServerError(None) });

        let fcm_f = request_f.and_then(move |response: Response| {
            let retry_after = response.headers().get::<RetryAfter>().map(|ra| *ra);

            let body_vec: Vec<u8> = match response.headers().get::<ContentLength>()
                .map(|cs| cs.0 as usize) {

                Some(content_size) => Vec::with_capacity(content_size),
                None => Vec::new()
            };

            let response_status = response.status().clone();

            response.body().map_err(|_| { response::FcmError::ServerError(None) })
                .fold(body_vec, |mut acc, chunk| {
                    acc.extend_from_slice(chunk.as_ref());
                    ok(acc)
                }).and_then(move |body_vec| {
                    if let Ok(body) = String::from_utf8(body_vec.clone()) {
                        match response_status {
                            StatusCode::Ok => {
                                let fcm_response: FcmResponse = json::decode(&body).unwrap();

                                match fcm_response.error.as_ref().map(String::as_ref) {
                                    Some("Unavailable") =>
                                        err(response::FcmError::ServerError(retry_after.clone())),
                                    Some("InternalServerError") =>
                                        err(response::FcmError::ServerError(retry_after.clone())),
                                    _ =>
                                        ok(fcm_response)
                                }
                            },
                            StatusCode::Unauthorized =>
                                err(response::FcmError::Unauthorized),
                            StatusCode::BadRequest =>
                                err(response::FcmError::InvalidMessage("Bad Request".to_string())),
                            status if status.is_server_error() =>
                                err(response::FcmError::ServerError(retry_after.clone())),
                            _ =>
                                err(response::FcmError::InvalidMessage("Unknown Error".to_string()))
                        }
                    } else {
                        err(response::FcmError::InvalidMessage("Unknown Error".to_string()))
                    }
                })
        });

        FutureResponse(Box::new(fcm_f))
    }
}

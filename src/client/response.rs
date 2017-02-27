use hyper::header::RetryAfter;

#[derive(RustcDecodable, Debug)]
pub struct FcmResponse {
    pub message_id: Option<u64>,
    pub error: Option<String>,
    pub multicast_id: Option<i64>,
    pub success: Option<u64>,
    pub failure: Option<u64>,
    pub canonical_ids: Option<u64>,
    pub results: Option<Vec<MessageResult>>,
}

#[derive(RustcDecodable, Debug)]
pub struct MessageResult {
    pub message_id: Option<String>,
    pub registration_id: Option<String>,
    pub error: Option<String>,
}

#[derive(PartialEq, Debug)]
pub enum FcmError {
    Unauthorized,
    InvalidMessage(String),
    ServerError(Option<RetryAfter>),
}

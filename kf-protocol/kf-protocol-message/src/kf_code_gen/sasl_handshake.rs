/// WARNING: CODE GENERATED FILE
/// * This file is generated by kfspec2code.
/// * Any changes applied to this file will be lost when a new spec is generated.
use serde::{Deserialize, Serialize};

use kf_protocol_api::ErrorCode;
use kf_protocol_api::Request;

use kf_protocol_derive::Decode;
use kf_protocol_derive::Encode;
use kf_protocol_derive::KfDefault;

// -----------------------------------
// KfSaslHandshakeRequest
// -----------------------------------

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct KfSaslHandshakeRequest {
    /// The SASL mechanism chosen by the client.
    pub mechanism: String,
}

// -----------------------------------
// KfSaslHandshakeResponse
// -----------------------------------

#[derive(Encode, Decode, Serialize, Deserialize, KfDefault, Debug)]
pub struct KfSaslHandshakeResponse {
    /// The error code, or 0 if there was no error.
    pub error_code: ErrorCode,

    /// The mechanisms enabled in the server.
    pub mechanisms: Vec<String>,
}

// -----------------------------------
// Implementation - KfSaslHandshakeRequest
// -----------------------------------

impl Request for KfSaslHandshakeRequest {
    const API_KEY: u16 = 17;

    const MIN_API_VERSION: i16 = 0;
    const MAX_API_VERSION: i16 = 1;
    const DEFAULT_API_VERSION: i16 = 1;

    type Response = KfSaslHandshakeResponse;
}
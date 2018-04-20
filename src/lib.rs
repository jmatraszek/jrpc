//! Crate to define the jsonrpc spec datatypes using serde -- that is it.
//!
//! This crate never touches the network, filesystem, etc.
//!
//! http://www.jsonrpc.org/specification_v2

#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate std_prelude;

mod serialize;

use std_prelude::*;
use serde::ser::Serialize;
use serde::de::{Deserialize, DeserializeOwned};

/// The `jsonrpc` version. Will serialize/deserialize to/from `"2.0"`.
pub struct V2_0;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
/// The jsonrpc `id` field. Can be a string, integer or null.
///
/// # Examples
///
/// ```rust
/// # extern crate jrpc;
/// extern crate serde_json;
/// use jrpc::Id;
///
/// # fn main() {
/// assert_eq!(Id::from(4), Id::Int(4));
/// assert_eq!(
///     serde_json::from_str::<Id>("4").unwrap(),
///     Id::Int(4),
/// );
/// assert_eq!(
///     serde_json::from_str::<Id>("\"foo\"").unwrap(),
///     Id::String("foo".into()),
/// );
/// # }
/// ```
pub enum Id {
    String(String),
    Int(u64),
    Null,
}

impl From<String> for Id {
    fn from(s: String) -> Self {
        Id::String(s)
    }
}

impl From<u64> for Id {
    fn from(v: u64) -> Self {
        Id::Int(v)
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// The jsonrpc Request object.
///
/// # Examples
///
/// ```rust
/// # extern crate jrpc;
/// extern crate serde_json;
/// use jrpc::{Id, Request, V2_0};
///
/// # fn main() {
/// let value: Vec<u32> = vec![1, 2, 3];
/// let request = Request::with_params(
///     Id::from(4),
///     "CreateFoo".into(),
///     Some(value.clone()),
/// );
/// let json = r#"
/// {"jsonrpc": "2.0",
///     "method": "CreateFoo",
///     "params": [1,2,3],
///     "id": 4
/// }
/// "#;
/// let json = json.replace("\n", "").replace(" ", "");
/// let result = serde_json::to_string(&request).unwrap();
/// assert_eq!(json, result);
/// # }
/// ```
pub struct Request<T> {
    pub jsonrpc: V2_0,
    pub method: String,
    pub params: Option<T>,
    pub id: Id,
}

impl<T: Serialize+DeserializeOwned> Request<T> {
    pub fn new(id: Id, method: String) -> Self {
        Self {
            jsonrpc: V2_0,
            method: method,
            params: None,
            id: id,
        }
    }

    pub fn with_params(id: Id, method: String, params: T) -> Self
    {
        Self {
            jsonrpc: V2_0,
            method: method,
            params: Some(params),
            id: id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
/// The jsonrpc Result response, indicating a successful result.
pub struct Result<T> {
    pub jsonrpc: V2_0,
    pub result: T,
    pub id: Id,
}

// #[derive(Debug, Serialize, Deserialize)]
/// The jsonrpc Error response, indicating an error.
pub struct Error<T> {
    pub jsonrpc: V2_0,
    pub error: ErrorObject<T>,
    pub id: Id,
}


#[derive(Debug, Serialize, Deserialize)]
/// The jsonrpc Error object, with details of the error.
///
/// Typically you may want to deserialze this with `T == serde_json::Value`
/// to first inspect the value of the `ErrorCode`.
pub struct ErrorObject<T> {
    pub code: ErrorCode,
    pub message: String,
    pub data: T,
}

#[derive(Debug)]
/// An error code.
pub enum ErrorCode {
    /// - `-32700`: Parse error. Invalid JSON was received by the server.
    ///   An error occurred on the server while parsing the JSON text.
    ParseError,
    /// - `-32600`: Invalid Request. The JSON sent is not a valid Request object.
    InvalidRequest,
    /// - `-32601`: Method not found. The method does not exist / is not available.
    MethodNotFound,
    /// - `-32602`: Invalid params. Invalid method parameter(s).
    InvalidParams,
    /// - `-32603`: Internal error. Internal JSON-RPC error.
    InternalError,
    /// - `-32000 to -32099`: Server error. Reserved for implementation-defined server-errors.
    ServerError(i64),
}

impl ErrorCode {
    /// Return whether the ErrorCode is correct.
    ///
    /// This will only return `false` if this is ServerError and is outside of the range of -32000
    /// to -32099.
    fn is_valid(&self) -> bool {
        match *self {
            ErrorCode::ServerError(value) => {
                if (-32099 <= value) && (value <= -32000) {
                    true
                } else {
                    false
                }
            }
            _ => true,
        }
    }
}

impl From<i64> for ErrorCode {
    fn from(v: i64) -> ErrorCode {
        match v {
            -32700 => ErrorCode::ParseError,
            -32600 => ErrorCode::InvalidRequest,
            -32601 => ErrorCode::MethodNotFound,
            -32602 => ErrorCode::InvalidParams,
            -32603 => ErrorCode::InternalError,
            _ => ErrorCode::ServerError(v),
        }
    }
}

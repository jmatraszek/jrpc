extern crate jrpc;
extern crate serde_json;

use jrpc::*;

#[test]
fn test_id() {
    let id: Id = serde_json::from_str("1").unwrap();
    assert_eq!(id, Id::Int(1));

    let id: Id = serde_json::from_str("\"1\"").unwrap();
    assert_eq!(id, Id::String("1".into()));

    let id: Id = serde_json::from_str("null").unwrap();
    assert_eq!(id, Id::Null);
}

#[test]
fn test_jsonrpc_v1() {
    let jsonrpc_1 = r#"{"result":"1","error":null,"id":1}"#;
    let response: Response<String> = serde_json::from_str(jsonrpc_1).unwrap();
    let result = serde_json::to_string(&response).unwrap();
    assert_eq!(jsonrpc_1, result);
}

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

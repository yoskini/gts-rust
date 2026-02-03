//! Test: Nested types should not have gts_instance_json methods
//!
//! This tests that nested types (those with base = ParentType) do not have
//! the gts_instance_json* methods, which prevents direct serialization.
//! Users should serialize the complete parent type instead.

use gts::GtsInstanceId;
use gts_macros::struct_to_gts_schema;

// Define a base type with an id field
#[struct_to_gts_schema(
    dir_path = "schemas",
    base = true,
    schema_id = "gts.x.core.events.type.v1~",
    description = "Base event type",
    properties = "id,payload"
)]
#[derive(Debug)]
pub struct BaseEventV1<P> {
    pub id: GtsInstanceId,
    pub payload: P,
}

// Define a nested type (has base = ParentType)
#[struct_to_gts_schema(
    dir_path = "schemas",
    base = BaseEventV1,
    schema_id = "gts.x.core.events.type.v1~x.core.nested.v1~",
    description = "Nested payload type",
    properties = "message"
)]
#[derive(Debug)]
pub struct NestedPayloadV1 {
    pub message: String,
}

fn main() {
    let nested = NestedPayloadV1 {
        message: "test".to_string(),
    };

    // This should fail: nested types don't have gts_instance_json method
    // Users should serialize the complete parent type instead:
    //   let event = BaseEventV1 { id: ..., payload: nested };
    //   event.gts_instance_json();
    let _ = nested.gts_instance_json();
}

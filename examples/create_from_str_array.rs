use json_pointer_simd::{JsonPointer,JsonPointerTarget};
use simd_json::json; // or serde_json::json

fn main() {
    let ptr = JsonPointer::new([
        "foo",
        "bar",
    ]);

    assert_eq!(&ptr.to_string(), "/foo/bar");

    let document = json!({
        "foo": {
            "bar": 0,
            "baz": 1,
        },
        "quux": "xyzzy"
    });

    let indexed = document.get(&ptr).unwrap();

    assert_eq!(indexed, &json!(0));
}

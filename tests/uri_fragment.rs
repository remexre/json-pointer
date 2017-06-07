extern crate json_pointer;

use json_pointer::JsonPointer;

macro_rules! assert_unparse {
    ($expr:expr) => {
        let ptr = $expr.parse::<JsonPointer<_, _>>().unwrap();
        assert_eq!(ptr.uri_fragment(), $expr);
    };
}

#[test]
fn uri_fragment_unparses() {
    assert_unparse!("#/");
    assert_unparse!("#/per%25/%25cent");
}

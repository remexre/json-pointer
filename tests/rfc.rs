extern crate json_pointer;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_json;

use json_pointer::JsonPointer;
use serde_json::Value;

macro_rules! rfc_tests {
    ($($ptr:expr => $json:tt;)*) => {
        /// The tests in [Section 5 of RFC 6901](https://tools.ietf.org/html/rfc6901#section-5).
        #[test]
        fn rfc_tests() {
            lazy_static! {
                static ref JSON: Value = json!({
                    "foo": ["bar", "baz"],
                    "": 0,
                    "a/b": 1,
                    "c%d": 2,
                    "e^f": 3,
                    "g|h": 4,
                    "i\\j": 5,
                    "k\"l": 6,
                    " ": 7,
                    "m~n": 8,
                });
            }

            $({
                let ptr = $ptr.parse::<JsonPointer<_, _>>().unwrap();
                assert_eq!(ptr.get(&JSON).unwrap(), &json!($json));
            })*
        }
    }
}

rfc_tests! {
    "" => {
        "foo": ["bar", "baz"],
        "": 0,
        "a/b": 1,
        "c%d": 2,
        "e^f": 3,
        "g|h": 4,
        "i\\j": 5,
        "k\"l": 6,
        " ": 7,
        "m~n": 8,
    };
    "/foo"   => ["bar", "baz"];
    "/foo/0" => "bar";
    "/"     => 0;
    "/a~1b" => 1;
    "/c%d"  => 2;
    "/e^f"  => 3;
    "/g|h"  => 4;
    "/i\\j" => 5;
    "/k\"l" => 6;
    "/ "    => 7;
    "/m~0n" => 8;
}

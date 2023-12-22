use json_pointer_simd::JsonPointer;
use json_pointer_simd::JsonPointerTarget;
use once_cell::sync::Lazy;
use simd_json::OwnedValue;
use simd_json::json;

static JSON: Lazy<OwnedValue> = Lazy::new(||
    json!({
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
    })
);

macro_rules! rfc_tests {
    ($($ptr:expr => $json:tt;)*) => {
        /// The tests in Sections [5](https://tools.ietf.org/html/rfc6901#section-5)
        /// and [6](https://tools.ietf.org/html/rfc6901#section-6) of RFC 6901.
        #[test]
        fn rfc_tests() {
            $({
                let ptr = $ptr.parse::<JsonPointer<_, _>>().unwrap();
                assert_eq!(ptr.get(Lazy::force(&JSON)).unwrap(), &json!($json));
            })*
        }
    }
}

rfc_tests! {
    // Section 5
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

    // Section 6
    "#" => {
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
    "#/foo"   => ["bar", "baz"];
    "#/foo/0" => "bar";
    "#/"      => 0;
    "#/a~1b"  => 1;
    "#/c%25d" => 2;
    "#/e%5Ef" => 3;
    "#/g%7Ch" => 4;
    "#/i%5Cj" => 5;
    "#/k%22l" => 6;
    "#/%20"   => 7;
    "#/m~0n"  => 8;
}

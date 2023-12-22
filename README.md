# json-pointer

## Preamble
This crate is a generalization of the [json-pointer](https://github.com/remexre/json-pointer) crate.

It opens up the target of the JSON pointer to anything that can be adapted using the `JsonPointerTarget`
trait. The odd name of the crate (_.simd_) comes from the first use case and first attempt at implementation -
- using JSON Pointers with [simd-json](https://docs.rs/simd-json/latest/simd_json) Values.

But one learns in these efforts and the back-implementation of the `JsonPointerTarget` trait to re-include 
[serde_json] values became pretty obvious pretty quickly!

HOPEfully, then, this crate is a stop-gap to getting all this merged back into `json-pointer` at some point in
the future. Before then there is a lot to do -features, tests, docs, better semantics ...

Apart from the `JsonPointerTarget`-related refactoring, I have also made some updates to the code to use the 2021 
semantics of Rust.

Otherwise, all the code, examples, and tests are those of the original author.

## Read me
A crate for parsing and using JSON pointers, as specified in [RFC
6901](https://tools.ietf.org/html/rfc6901). Unlike the `pointer` method
built into `serde_json`, this handles both validating JSON Pointers before
use and the URI Fragment Identifier Representation.

## Creating a JSON Pointer

JSON pointers can be created with a literal `[&str]`, or parsed from a `String`.

```rust
let from_strs = JsonPointer::new([
    "foo",
    "bar",
]);
let parsed = "/foo/bar".parse::<JsonPointer<_, _>>().unwrap();
let from_dotted_notation = JsonPointer::new("foo.bar".split('.').collect::<Vec<&str>>());

assert_eq!(from_strs.to_string(), parsed.to_string());
```

## Using a JSON Pointer

The `JsonPointer` type provides `.get()` and `.get_mut()`, to get references
and mutable references to the appropriate value, respectively.

```rust
let ptr = "/foo/bar".parse::<JsonPointer<_, _>>().unwrap();

let document = json!({
    "foo": {
        "bar": 0,
        "baz": 1,
    },
    "quux": "xyzzy"
});

let indexed = document.get(&ptr).unwrap();

assert_eq!(indexed, &json!(0));
```

## URI Fragment Identifier Representation

JSON Pointers can be embedded in the fragment portion of a URI. This is the
reason why most JSON pointer libraries require a `#` character at the beginning
of a JSON pointer. The crate will detect the leading `#` as an indicator to
parse in URI Fragment Identifier Representation. Note that this means that this
crate does not support parsing full URIs.

```rust
let str_ptr = "/f%o".parse::<JsonPointer<_, _>>().unwrap();
let uri_ptr = "#/f%25o".parse::<JsonPointer<_, _>>().unwrap();

assert_eq!(str_ptr, uri_ptr);
```

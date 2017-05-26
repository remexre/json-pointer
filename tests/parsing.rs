extern crate json_pointer;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate quickcheck;
extern crate regex;

use json_pointer::JsonPointer;
use quickcheck::TestResult;
use regex::Regex;

quickcheck! {

/// Essentially, `unparse(parse("..."))` should be a no-op.
fn faithful_parse(s: String) -> TestResult {
    match s.parse::<JsonPointer<_, _>>() {
        Ok(ptr) => if s == ptr.to_string() {
            TestResult::passed()
        } else {
            TestResult::failed()
        },
        Err(_) => TestResult::discard(),
    }
}

/// Ensuring that parsing succeeds for all strings that match the regex for
/// JSON pointers.
fn parses_all_valid(s: String) -> bool {
    lazy_static! {
        static ref JSON_POINTER_REGEX: Regex = Regex::new("^(/([^/~]|~[01])*)*$").unwrap();
    }

    let matches_regex = JSON_POINTER_REGEX.is_match(&s);
    let parses = s.parse::<JsonPointer<_, _>>().is_ok();

    (matches_regex == parses)
}

}

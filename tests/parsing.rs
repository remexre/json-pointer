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
    let ok = match s.parse::<JsonPointer<_, _>>() {
        Ok(ptr) => if s.chars().next() == Some('#') {
            (s == ptr.uri_fragment())
        } else {
            (s == ptr.to_string())
        },
        Err(_) => return TestResult::discard(),
    };
    if ok {
        TestResult::passed()
    } else {
        TestResult::failed()
    }
}

/// Ensuring that parsing succeeds for all strings that match the regex for
/// JSON pointers.
fn parses_all_valid(s: String) -> bool {
    lazy_static! {
        static ref JSON_POINTER_REGEX: Regex = Regex::new("^(/([^/~]|~[01])*)*$").unwrap();
        static ref URI_FRAGMENT_REGEX: Regex = Regex::new("^#(/([^A-Za-z0-9._!$&'()*+,;=@/?-]|~[01]|%[0-9a-fA-F]{2})*)*$").unwrap();
    }

    let matches_regex = JSON_POINTER_REGEX.is_match(&s) || URI_FRAGMENT_REGEX.is_match(&s);
    let parses = s.parse::<JsonPointer<_, _>>().is_ok();

    (matches_regex == parses)
}

}

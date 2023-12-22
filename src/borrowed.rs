use std::ops::{Index, IndexMut};
use crate::{JsonPointer, JsonPointerTarget, IndexError};
use simd_json::value::borrowed::Value;

/// Implement getting for SIMD JSON Borrowed values
///
impl<'a> JsonPointerTarget for Value<'a> 
    where Self: Sized {
    fn get<'json, S: AsRef<str>, C: AsRef<[S]>>(&'json self, ptr: &JsonPointer<S,C>) -> Result<&'json Self, IndexError> {
        ptr.ref_toks.as_ref().iter().try_fold(self, |val, tok| {
            let tok = tok.as_ref();
            match *val {
                Value::Object(ref obj) => obj.get(tok).ok_or_else(|| IndexError::NoSuchKey(tok.to_owned())),
                Value::Array(ref arr) => {
                    let idx = if tok == "-" {
                        arr.len()
                    } else if let Ok(idx) = tok.parse() {
                        idx
                    } else {
                        return Err(IndexError::NoSuchKey(tok.to_owned()));
                    };
                    arr.get(idx).ok_or(IndexError::OutOfBounds(idx))
                },
                _ => Err(IndexError::NotIndexable),
            }
        })
    }

    fn get_mut<'json, S: AsRef<str>, C: AsRef<[S]>>(&'json mut self, ptr: &JsonPointer<S,C>) -> Result<&'json mut Self, IndexError> {
        ptr.ref_toks.as_ref().iter().try_fold(self, |val, tok| {
            let tok = tok.as_ref();
            match *val {
                Value::Object(ref mut obj) => obj.get_mut(tok).ok_or_else(|| IndexError::NoSuchKey(tok.to_owned())),
                Value::Array(ref mut arr) => {
                    let idx = if tok == "-" {
                        arr.len()
                    } else if let Ok(idx) = tok.parse() {
                        idx
                    } else {
                        return Err(IndexError::NoSuchKey(tok.to_owned()));
                    };
                    arr.get_mut(idx).ok_or(IndexError::OutOfBounds(idx))
                },
                _ => Err(IndexError::NotIndexable),
            }
        })
    }

    fn get_owned<'json, S: AsRef<str>, C: AsRef<[S]>>(self, ptr: &JsonPointer<S,C>) -> Result<Self, IndexError> {
        ptr.ref_toks.as_ref().iter().try_fold(self, |val, tok| {
            let tok = tok.as_ref();
            match val {
                Value::Object(mut obj) => obj.remove(tok).ok_or_else(|| IndexError::NoSuchKey(tok.to_owned())),
                Value::Array(mut arr) => {
                    let idx = if tok == "-" {
                        arr.len()
                    } else if let Ok(idx) = tok.parse() {
                        idx
                    } else {
                        return Err(IndexError::NoSuchKey(tok.to_owned()));
                    };
                    if idx >= arr.len() {
                        Err(IndexError::OutOfBounds(idx))
                    } else {
                        Ok(arr.swap_remove(idx))
                    }
                },
                _ => Err(IndexError::NotIndexable),
            }
        })
    }
}
impl<'a, S: AsRef<str>, C: AsRef<[S]>> Index<&'a JsonPointer<S, C>> for Value<'a> {
    type Output = Value<'a>;
    fn index(&self, ptr: &'a JsonPointer<S, C>) -> &Value<'a> {
        self.get(ptr).unwrap()
    }
}

impl<'a, S: AsRef<str>, C: AsRef<[S]>> IndexMut<&'a JsonPointer<S, C>> for Value<'a> {
    fn index_mut(&mut self, ptr: &'a JsonPointer<S, C>) -> &mut Value<'a> {
        self.get_mut(ptr).unwrap()
    }
}

use std::ops::{Index, IndexMut};
use crate::{JsonPointer, JsonPointerTarget, IndexError};
use serde_json::Value;

/// Implement getting for SIMD JSON Owned values
///
impl<S: AsRef<str>, C: AsRef<[S]>> JsonPointerTarget<Value> for JsonPointer<S, C> {

    /// Attempts to get a reference to a value from the given JSON value,
    /// returning an error if it can't be found.
    fn get<'json>(&self, val: &'json Value) -> Result<&'json Value, IndexError> {
        self.ref_toks.as_ref().iter().try_fold(val, |val, tok| {
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

    /// Attempts to get a mutable reference to a value from the given JSON
    /// value, returning an error if it can't be found.
    fn get_mut<'json>(&self, val: &'json mut Value) -> Result<&'json mut Value, IndexError> {
        self.ref_toks.as_ref().iter().try_fold(val, |val, tok| {
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

    /// Attempts to get an owned value from the given JSON value, returning an
    /// error if it can't be found.
    fn get_owned(&self, val: Value) -> Result<Value, IndexError> {
        self.ref_toks.as_ref().iter().try_fold(val, |val, tok| {
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

impl<'a, S: AsRef<str>, C: AsRef<[S]>> Index<&'a JsonPointer<S, C>> for Value {
    type Output = Value;
    fn index(&self, ptr: &'a JsonPointer<S, C>) -> &Value {
        ptr.get(self).unwrap()
    }
}

impl<'a, S: AsRef<str>, C: AsRef<[S]>> IndexMut<&'a JsonPointer<S, C>> for Value {
    fn index_mut(&mut self, ptr: &'a JsonPointer<S, C>) -> &mut Value {
        ptr.get_mut(self).unwrap()
    }
}

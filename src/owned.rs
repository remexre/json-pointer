use std::ops::{Index, IndexMut};
use crate::{JsonPointer, JsonPointerValueGetter, IndexError};

/// Implement getting for SIMD JSON Owned values
///
impl<S: AsRef<str>, C: AsRef<[S]>> JsonPointerValueGetter<simd_json::OwnedValue> for JsonPointer<S, C> {

    /// Attempts to get a reference to a value from the given JSON value,
    /// returning an error if it can't be found.
    fn get<'json>(&self, val: &'json simd_json::OwnedValue) -> Result<&'json simd_json::OwnedValue, IndexError> {
        self.ref_toks.as_ref().iter().fold(Ok(val), |val, tok| val.and_then(|val| {
            let tok = tok.as_ref();
            match *val {
                simd_json::OwnedValue::Object(ref obj) => obj.get(tok).ok_or_else(|| IndexError::NoSuchKey(tok.to_owned())),
                simd_json::OwnedValue::Array(ref arr) => {
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
        }))
    }

    /// Attempts to get a mutable reference to a value from the given JSON
    /// value, returning an error if it can't be found.
    fn get_mut<'json>(&self, val: &'json mut simd_json::OwnedValue) -> Result<&'json mut simd_json::OwnedValue, IndexError> {
        self.ref_toks.as_ref().iter().fold(Ok(val), |val, tok| val.and_then(|val| {
            let tok = tok.as_ref();
            match *val {
                simd_json::OwnedValue::Object(ref mut obj) => obj.get_mut(tok).ok_or_else(|| IndexError::NoSuchKey(tok.to_owned())),
                simd_json::OwnedValue::Array(ref mut arr) => {
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
        }))
    }

    /// Attempts to get an owned value from the given JSON value, returning an
    /// error if it can't be found.
    fn get_owned(&self, val: simd_json::OwnedValue) -> Result<simd_json::OwnedValue, IndexError> {
        self.ref_toks.as_ref().iter().fold(Ok(val), |val, tok| val.and_then(|val| {
            let tok = tok.as_ref();
            match val {
                simd_json::OwnedValue::Object(mut obj) => obj.remove(tok).ok_or_else(|| IndexError::NoSuchKey(tok.to_owned())),
                simd_json::OwnedValue::Array(mut arr) => {
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
        }))
    }
}

impl<'a, S: AsRef<str>, C: AsRef<[S]>> Index<&'a JsonPointer<S, C>> for simd_json::OwnedValue {
    type Output = simd_json::OwnedValue;
    fn index(&self, ptr: &'a JsonPointer<S, C>) -> &simd_json::OwnedValue {
        ptr.get(self).unwrap()
    }
}

impl<'a, S: AsRef<str>, C: AsRef<[S]>> IndexMut<&'a JsonPointer<S, C>> for simd_json::OwnedValue {
    fn index_mut(&mut self, ptr: &'a JsonPointer<S, C>) -> &mut simd_json::OwnedValue {
        ptr.get_mut(self).unwrap()
    }
}

//! Provides `ByteResult` trait.
//!
//! The trait is mainly used as source of Encoding filters.
//!


use std::error;
use std::marker;
use std::result;

use mm_errors;


/// Byte result.
///
/// The trait provides one function `byte_result`, which converts `self` to a `std::result::Result`.
///
/// For `u8`, `byte_result` returns `mm_errors::Result(u8)`, and the result is always `Ok`.
///
/// For `std::result::Result`, `byte_result` simple returns the value itself (`self`).
///
/// We can use `Iterator<u8>` and `Iterator<Result<u8, _>>` as a source of encoding filters
/// in the same manner  by the trait.
///
pub trait ByteResult
    where Self::Error: error::Error + marker::Send + marker::Sync + 'static {
    type Error;
    fn byte_result(self) -> result::Result<u8, Self::Error>;
}

impl ByteResult for u8 {
    type Error = mm_errors::Error;

    fn byte_result(self) -> result::Result<u8, <Self as ByteResult>::Error> {
        Ok(self)
    }
}

impl<E> ByteResult for result::Result<u8, E>
    where E: error::Error + marker::Send + marker::Sync + 'static {
    type Error = E;

    fn byte_result(self) -> result::Result<u8, <Self as ByteResult>::Error> {
        self
    }
}

//! Provides `CharResult` trait.
//!
//! The trait is mainly used as source of Decoding filters.
//!


use std::error;
use std::marker;
use std::result;

use mm_errors;


/// Byte result.
///
/// The trait provides one function `char_result`, which converts `self` to a `std::result::Result`.
///
/// For `char`, `char_result` returns `mm_errors::Result(char)`, and the result is always `Ok`.
///
/// For `std::result::Result`, `char_result` simple returns the value itself (`self`).
///
/// We can use `Iterator<char>` and `Iterator<Result<char, _>>` as a source of encoding filters
/// in the same manner  by the trait.
///
pub trait CharResult
    where Self::Error: error::Error + marker::Send + marker::Sync + 'static {
    type Error;
    fn char_result(self) -> result::Result<char, Self::Error>;
}

impl CharResult for char {
    type Error = mm_errors::Error;

    fn char_result(self) -> result::Result<char, <Self as CharResult>::Error> {
        Ok(self)
    }
}

impl<E> CharResult for result::Result<char, E>
    where E: error::Error + marker::Send + marker::Sync + 'static {

    type Error = E;
    fn char_result(self) -> result::Result<char, <Self as CharResult>::Error> {
        self
    }
}

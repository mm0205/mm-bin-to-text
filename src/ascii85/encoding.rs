//! Provides features for Encoding Ascii85.

extern crate mm_errors;

use byte_result::ByteResult;

use mm_errors::Result;

use ascii85::constants::*;


/// Iterator for Ascii85 encoding.
pub struct Ascii85Encoding<I>
    where I: Iterator, I::Item: ByteResult {
    /// Source to encode.
    source: Box<I>,

    /// Temporary encoded block.
    block: [char; CHARACTER_BLOCK_SIZE],

    /// Output index.
    char_index_in_block: usize,

    /// Omit flags.
    omission_flags: [bool; CHARACTER_BLOCK_SIZE],
}

impl<I> Ascii85Encoding<I>
    where I: Iterator, I::Item: ByteResult {
    /// Returns a new instance of `Ascii85Encoding`.
    pub fn new<T>(iterator: T) -> Ascii85Encoding<T>
        where T: Iterator, T::Item: ByteResult {
        Ascii85Encoding {
            source: Box::new(iterator),
            block: ['\0'; CHARACTER_BLOCK_SIZE],
            char_index_in_block: 0,
            omission_flags: [false; CHARACTER_BLOCK_SIZE],
        }
    }

    /// Converts the `byte_index` to `omission_index`.
    fn byte_index_to_omission_index(&self, byte_index: usize) -> usize {
        match byte_index {
            0 => 0,
            1 => 2,
            2 => 3,
            3 => 4,
            5 => 5,
            _ => panic!("Invalid byte index"),
        }
    }

    /// Encodes one block.
    fn encode_block(&mut self) -> Result<()> {
        let byte_block_length = BINARY_BLOCK_SIZE;

        let mut total_bytes = 0_u32;
        for i in 0..byte_block_length {
            let b = match self.source.next() {
                Some(x) => {
                    match x.byte_result() {
                        Ok(v) => v as u32,
                        Err(e) => return Err(wrap_error!(e)),
                    }
                }
                None => {
                    if i == 0 {
                        self.omission_flags[0] = true;
                        return Ok(());
                    }
                    self.omission_flags[self.byte_index_to_omission_index(i)] = true;
                    0_u32
                }
            };
            total_bytes += b << (8 * (byte_block_length - 1 - i));
        }

        let total = total_bytes;
        if total == 0 {
            self.block[0] = CHAR_FOR_4_ZEROS;
            return Ok(());
        }

        let c1 = total / C1_BIAS;
        let c2_total = total - c1 * C1_BIAS;

        let c2 = c2_total / C2_BIAS;
        let c3_total = c2_total - c2 * C2_BIAS;

        let c3 = c3_total / C3_BIAS;
        let c4_total = c3_total - c3 * C3_BIAS;

        let c4 = c4_total / C4_BIAS;
        let c5 = c4_total - c4 * C4_BIAS;

        self.block[0] = (c1 as u8 + BINARY_TO_CHAR_BIAS).into();
        self.block[1] = (c2 as u8 + BINARY_TO_CHAR_BIAS).into();
        self.block[2] = (c3 as u8 + BINARY_TO_CHAR_BIAS).into();
        self.block[3] = (c4 as u8 + BINARY_TO_CHAR_BIAS).into();
        self.block[4] = (c5 as u8 + BINARY_TO_CHAR_BIAS).into();

        Ok(())
    }
}

impl<I> Iterator for Ascii85Encoding<I>
    where I: Iterator, I::Item: ByteResult {
    type Item = Result<char>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.char_index_in_block == 0 {
            match self.encode_block() {
                Err(e) => return Some(Err(e)),
                _ => (),
            };
        }

        if self.omission_flags[self.char_index_in_block] {
            return None
        }

        let result = self.block[self.char_index_in_block];
        match result {
            CHAR_FOR_4_ZEROS => {
                self.char_index_in_block = 0;
            },
            _ => {
                self.char_index_in_block = (self.char_index_in_block + 1) % self.block.len();
            }
        }

        return Some(Ok(result));
    }
}


/// The trait extends iterators and provides a function to create a new instance of `Ascii85Encoding`.
pub trait Ascii85EncodingExtension<T>
    where T: Iterator, T::Item: ByteResult {
    fn ascii85_encoding(self) -> Ascii85Encoding<T>;
}

impl<T> Ascii85EncodingExtension<T> for T
    where T: Iterator, T::Item: ByteResult {
    fn ascii85_encoding(self) -> Ascii85Encoding<T> {
        Ascii85Encoding::<T>::new(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::*;

    #[test]
    fn test_ascii85_encode() {
        let test_data = vec![0_u8, 1, 2, 3];
        let source = test_data
            .clone().into_iter();
        let encoded = source.ascii85_encoding().collect::<mm_errors::Result<String>>().unwrap();
        assert_eq!("!!*-'", encoded);
    }

    #[test]
    fn test_ascii85_encode2() {
        let test_data = vec![Ok(0_u8), new_result!("test"), Ok(2), Ok(3)];
        let source = test_data
            .clone().into_iter();
        let encoded = source.ascii85_encoding().collect::<mm_errors::Result<String>>();
        match encoded {
            Err(x) => println!("error: {:?}", x),
            _ => panic!("should be an error"),
        };
    }

    #[test]
    fn test_ascii85_encode_string() {
        let test_data = "test string";
        let source = Cursor::new(test_data).bytes();
        let encoded = source.ascii85_encoding().collect::<mm_errors::Result<String>>().unwrap();

        assert_eq!("FCfN8+EMXFBl7P", encoded);
    }

    #[test]
    fn test_ascii85_encode_zeros() {
        let test_data = "test\0\0\0\0string";
        let source = test_data.bytes();
        let encoded = source.ascii85_encoding().collect::<mm_errors::Result<String>>().unwrap();

        assert_eq!("FCfN8zF*)G:DJ&", encoded);
    }
}

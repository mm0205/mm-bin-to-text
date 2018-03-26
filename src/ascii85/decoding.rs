//! Provides features for Decoding Ascii85.

use ascii85::constants::*;
use char_result::*;
use mm_errors::Result;

/// Implements `Iterator` trait, which returns a Ascii85-decoded byte for each iteration.
pub struct Ascii85Decoding<I>
    where I: Iterator, I::Item: CharResult {
    /// Source to encode.
    source: Box<I>,

    /// Temporary decoded block.
    block: [u8; BINARY_BLOCK_SIZE],

    /// Output index.
    byte_index_in_block: usize,

    /// Omit flags.
    omission_flags: [bool; BINARY_BLOCK_SIZE],
}

impl<I> Ascii85Decoding<I>
    where I: Iterator, I::Item: CharResult {

    /// Returns a new instance of `Ascii85Encoding`.
    pub fn new<T>(iterator: T) -> Ascii85Decoding<T>
        where T: Iterator, T::Item: CharResult {
        Ascii85Decoding {
            source: Box::new(iterator),
            block: [0; BINARY_BLOCK_SIZE],
            byte_index_in_block: 0,
            omission_flags: [false; BINARY_BLOCK_SIZE],
        }
    }

    /// Decodes a block.
    fn decode_block(&mut self) -> Result<()> {
        let mut total_bytes = 0_u32;
        for i in 0..CHARACTER_BLOCK_SIZE {
            let b = match self.source.next() {
                None => if i == 0 {
                    self.omission_flags[0] = true;
                    return Ok(())
                } else {
                    self.omission_flags[self.byte_index_to_omission_index(i)] = true;
                    ((LAST_CHAR as u8) - BINARY_TO_CHAR_BIAS) as u32
                },
                Some(x) => match x.char_result() {
                    Ok(x) => if (x < FIRST_CHAR || x > LAST_CHAR) && (x != CHAR_FOR_4_ZEROS) {
                        return new_result!(&format!("Invalid character: {}, a character must be >= '!' and <= 'u' or equal to 'z'", x));
                    } else {
                        if x == CHAR_FOR_4_ZEROS && i != 0 {
                            return new_result!(&format!("Invalid character: {}, 'z' must be a first character in block", x));
                        }
                        if x == CHAR_FOR_4_ZEROS {
                            total_bytes = 0;
                            break;
                        }
                        ((x as u8) - BINARY_TO_CHAR_BIAS) as u32
                    },
                    Err(e) => return Err(wrap_error!(e)),
                }
            };

            total_bytes += b * C4_BIAS.pow((CHARACTER_BLOCK_SIZE - i - 1) as u32);
        }

        for i in 0..BINARY_BLOCK_SIZE {
            let current_byte = ((total_bytes >> (8 * (BINARY_BLOCK_SIZE - i - 1))) & 0x00FF_u32) as u8;
            self.block[i] = current_byte;
        }
        Ok(())
    }

    fn byte_index_to_omission_index(&self, byte_index: usize) -> usize {
        match byte_index {
            0 => 0,
            1 => 1,
            2 => 1,
            3 => 2,
            4 => 3,
            _ => panic!("Invalid Index"),
        }
    }
}

impl<I> Iterator for Ascii85Decoding<I>
    where I: Iterator, I::Item: CharResult {
    type Item = Result<u8>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.byte_index_in_block == 0 {
            match self.decode_block() {
                Err(x) => return Some(Err(wrap_error!(x))),
                _ => (),
            }
            self.byte_index_in_block = 0;
        }

        if self.omission_flags[self.byte_index_in_block] {
            return None
        }

        let result = self.block[self.byte_index_in_block];
        self.byte_index_in_block = (self.byte_index_in_block + 1) % self.block.len();

        Some(Ok(result))
    }
}

/// The trait extends iterators and provides a function to create a new instance of `Ascii85Encoding`.
pub trait Ascii85DecodingExtension<T>
    where T: Iterator, T::Item: CharResult {
    fn ascii85_decoding(self) -> Ascii85Decoding<T>;
}

impl<T> Ascii85DecodingExtension<T> for T
    where T: Iterator, T::Item: CharResult {
    fn ascii85_decoding(self) -> Ascii85Decoding<T> {
        Ascii85Decoding::<T>::new(self)
    }
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_decode() {
        let test_data = "!!*-'";
        let result = test_data.chars().ascii85_decoding().collect::<Result<Vec<u8>>>().unwrap();
        assert_eq!(4, result.len());
        assert_eq!(0, result[0]);
        assert_eq!(1, result[1]);
        assert_eq!(2, result[2]);
        assert_eq!(3, result[3]);
    }

    #[test]
    fn test_decode_2() {
        let test_data = "FCfN8+EMXFBl7P";
        let expected = "test string";

        let result = test_data.chars().ascii85_decoding().collect::<Result<Vec<u8>>>().unwrap();
        assert_eq!(expected.len(), result.len());
        for (x, y) in expected.bytes().zip(result) {
            assert_eq!(x, y);
        }
    }

    #[test]
    fn test_decode_3() {
        let test_data = "FCfN8zF*)G:DJ&";
        let expected = "test\0\0\0\0string";

        let result = test_data.chars().ascii85_decoding().collect::<Result<Vec<u8>>>().unwrap();
        assert_eq!(expected.len(), result.len());
        for (x, y) in expected.bytes().zip(result) {
            assert_eq!(x, y);
        }
    }
}

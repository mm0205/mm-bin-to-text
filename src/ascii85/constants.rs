//! (internal) Provides constants for Ascii85 encodings.

/// BIAS for Encoded character 1 ( 85 ** 4).
pub const C1_BIAS: u32 = 52200625;

/// BIAS for Encoded character 2 ( 85 ** 3).
pub const C2_BIAS: u32 = 614125;

/// BIAS for Encoded character 3 ( 85 ** 2).
pub const C3_BIAS: u32 = 7225;

/// BIAS for Encoded character 4 ( 85 ** 1).
pub const C4_BIAS: u32 = 85;

/// Character block size.
pub const CHARACTER_BLOCK_SIZE: usize = 5;

/// Binary block size.
pub const BINARY_BLOCK_SIZE: usize = 4;

/// Bias for converting from a binary value to character.
pub const BINARY_TO_CHAR_BIAS: u8 = 33;

/// The character stands for 4-bytes '\0'.
pub const CHAR_FOR_4_ZEROS: char = 'z';

/// First character.
pub const FIRST_CHAR: char = '!';

/// Last character.
pub const LAST_CHAR: char = 'u';
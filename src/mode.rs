//! Encoding modes for a QR code.

use bitvec::*;

/// Encoding modes.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Mode {
    /// Byte mode supports the ISO-8859-1 character set.
    Byte,
}

impl Mode {
    /// Create Mode from string, decide from content.
    pub fn from_str(s: &str) -> Mode {
        if Mode::in_byte(s) {
            Mode::Byte
        } else {
            // Should never happen.
            panic!("Unsupported mode for string {}", s);
        }
    }

    /// Is this a valid mode for a string?
    pub fn matches(&self, s: &str) -> bool {
        match self {
            Mode::Byte => Mode::in_byte(s),
        }
    }

    /// BitVec representation.
    pub fn to_bitvec(&self) -> BitVec {
        match self {
            Mode::Byte => bitvec![0, 1, 0, 0],
        }
    }

    /// Returns true if contents can be represented by the byte mode.
    pub fn in_byte(_s: &str) -> bool {
        true
    }
}

use std::fmt;

/// A struct representing a 64-bit array of bits stored in a u64.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitArray(pub u64);

impl fmt::Display for BitArray {
    /// Format the BitArray for display, printing bits as a sequence of 1s and 0s.
    /// Bits are printed from left to right, with spaces inserted every 8 bits.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in (0..64).rev() {
            write!(f, "{}", (self.0 >> i) & 1)?; // Print each bit
            if i % 8 == 0 && i != 0 {
                write!(f, " ")?; // Add space after every 8 bits (byte boundary)
            }
        }
        Ok(())
    }
}

impl BitArray {
    /// Creates a new BitArray with all bits set to 0.
    ///
    /// # Returns
    /// A new `BitArray` instance with 0 value.
    pub fn new() -> Self {
        Self(0)
    }

    /// Sets a specific bit to 1 at the given position (0-63).
    ///
    /// # Arguments
    /// - `pos`: The bit position to set (0-based index).
    pub fn set_bit(&mut self, pos: u8) {
        self.0 |= 1 << pos;
    }

    /// Clears a specific bit (sets it to 0) at the given position (0-63).
    ///
    /// # Arguments
    /// - `pos`: The bit position to clear (0-based index).
    pub fn clear_bit(&mut self, pos: u8) {
        self.0 &= !(1 << pos);
    }

    /// Toggles a specific bit (flips it between 1 and 0) at the given position (0-63).
    ///
    /// # Arguments
    /// - `pos`: The bit position to toggle (0-based index).
    pub fn toggle_bit(&mut self, pos: u8) {
        self.0 ^= 1 << pos;
    }

    /// Retrieves the value of a specific bit at the given position (0-63).
    ///
    /// # Arguments
    /// - `pos`: The bit position to retrieve (0-based index).
    ///
    /// # Returns
    /// - `true` if the bit is 1, `false` if the bit is 0.
    pub fn get_bit(&self, pos: u8) -> bool {
        (self.0 >> pos) & 1 == 1
    }

    /// Retrieves all the bits as a `Vec<bool>`.
    ///
    /// # Returns
    /// A vector of `bool` values representing the bits in the `BitArray`.
    pub fn get_all_bits(&self) -> Vec<bool> {
        (0..64).map(|i| self.get_bit(i)).collect()
    }

    /// Retrieves the raw `u64` value representing the `BitArray`.
    ///
    /// # Returns
    /// The raw `u64` value that stores the bits.
    pub fn get_raw(&self) -> u64 {
        self.0
    }
}

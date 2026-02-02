#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HammingStatus {
    Clean,
    Corrected(u8), // The index of the corrected bit (0-7)
    DoubleError,   // Unrecoverable
}

/// Encodes a 4-bit nibble into an 8-bit Hamming(8,4) SECDED codeword.
/// The input `nibble` should be in the lower 4 bits (0x0F).
pub fn encode(nibble: u8) -> u8 {
    let d1 = (nibble >> 0) & 1;
    let d2 = (nibble >> 1) & 1;
    let d3 = (nibble >> 2) & 1;
    let d4 = (nibble >> 3) & 1;

    // Calculate parity bits for Hamming(7,4)
    // p1 checks 1, 3, 5, 7 (p1, d1, d2, d4)
    // p2 checks 2, 3, 6, 7 (p2, d1, d3, d4)
    // p4 checks 4, 5, 6, 7 (p4, d2, d3, d4)

    // Note on bit positions in our byte:
    // 0: p0 (overall)
    // 1: p1
    // 2: p2
    // 3: d1
    // 4: p4
    // 5: d2
    // 6: d3
    // 7: d4

    let p1 = d1 ^ d2 ^ d4;
    let p2 = d1 ^ d3 ^ d4;
    let p4 = d2 ^ d3 ^ d4;

    let mut byte = 0u8;
    byte |= p1 << 1;
    byte |= p2 << 2;
    byte |= d1 << 3;
    byte |= p4 << 4;
    byte |= d2 << 5;
    byte |= d3 << 6;
    byte |= d4 << 7;

    // Calculate overall parity p0 (checks all bits 0..7)
    // Currently p0 is 0. So parity of byte is parity of 1..7.
    let p0 = byte.count_ones() as u8 % 2;

    byte |= p0 << 0;

    byte
}

/// Decodes an 8-bit Hamming(8,4) SECDED codeword.
/// Returns the decoded 4-bit data and the status.
pub fn decode(mut byte: u8) -> (u8, HammingStatus) {
    // Extract bits
    let get_bit = |b: u8, i: u8| (b >> i) & 1;

    let _p0 = get_bit(byte, 0);
    let p1 = get_bit(byte, 1);
    let p2 = get_bit(byte, 2);
    let d1 = get_bit(byte, 3);
    let p4 = get_bit(byte, 4);
    let d2 = get_bit(byte, 5);
    let d3 = get_bit(byte, 6);
    let d4 = get_bit(byte, 7);

    // Calculate syndrome
    // s1 checks 1, 3, 5, 7
    let s1 = p1 ^ d1 ^ d2 ^ d4;
    // s2 checks 2, 3, 6, 7
    let s2 = p2 ^ d1 ^ d3 ^ d4;
    // s4 checks 4, 5, 6, 7
    let s4 = p4 ^ d2 ^ d3 ^ d4;

    let syndrome = (s4 << 2) | (s2 << 1) | s1;

    // Calculate overall parity of the received byte
    let p_all = byte.count_ones() as u8 % 2;

    if syndrome == 0 {
        if p_all == 0 {
            // Clean
            let data = (d4 << 3) | (d3 << 2) | (d2 << 1) | d1;
            return (data, HammingStatus::Clean);
        } else {
            // Parity error in p0 itself (odd errors, but syndrome says 0 implies error is at 0)
            // Wait, if p0 flipped, p_all becomes !original_p_all.
            // Original p_all should be 0 (even parity).
            // So if p_all is 1, there is a single error.
            // Since syndrome is 0, the error is at position 0.
            // Correct p0.
            // Data is fine.
             let data = (d4 << 3) | (d3 << 2) | (d2 << 1) | d1;
             return (data, HammingStatus::Corrected(0));
        }
    } else {
        // Syndrome != 0
        if p_all == 1 {
            // Single error at position `syndrome`
            let error_pos = syndrome;
            // Flip the bit
            byte ^= 1 << error_pos;

            // Re-extract data
            let d1 = get_bit(byte, 3);
            let d2 = get_bit(byte, 5);
            let d3 = get_bit(byte, 6);
            let d4 = get_bit(byte, 7);

            let data = (d4 << 3) | (d3 << 2) | (d2 << 1) | d1;
            return (data, HammingStatus::Corrected(error_pos));
        } else {
            // Double error (or even number of errors)
            // Cannot correct reliably.
            // Return whatever data we have, but flag it.
            let d1 = get_bit(byte, 3);
            let d2 = get_bit(byte, 5);
            let d3 = get_bit(byte, 6);
            let d4 = get_bit(byte, 7);
            let data = (d4 << 3) | (d3 << 2) | (d2 << 1) | d1;
            return (data, HammingStatus::DoubleError);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_clean_decode() {
        for i in 0..16 {
            let encoded = encode(i);
            let (decoded, status) = decode(encoded);
            assert_eq!(decoded, i, "Data mismatch for {}", i);
            assert_eq!(status, HammingStatus::Clean, "Status mismatch for {}", i);
        }
    }

    #[test]
    fn test_single_bit_error() {
        let original = 0b1011; // 11
        let encoded = encode(original);

        // Try flipping every bit 0..7
        for pos in 0..8 {
            let corrupted = encoded ^ (1 << pos);
            let (decoded, status) = decode(corrupted);

            assert_eq!(decoded, original, "Failed to correct error at pos {}", pos);
            match status {
                HammingStatus::Corrected(p) => assert_eq!(p, pos, "Incorrect error position identified"),
                _ => panic!("Expected Corrected status for pos {}", pos),
            }
        }
    }

    #[test]
    fn test_double_bit_error() {
        let original = 0b0110;
        let encoded = encode(original);

        // Flip pos 1 and 2
        let corrupted = encoded ^ (1 << 1) ^ (1 << 2);
        let (_, status) = decode(corrupted);

        assert_eq!(status, HammingStatus::DoubleError);
    }
}

/// Constant time base64 encoding and decoding.
use super::{ErrorKind, Result};

/// Encode lower six bits of a byte into their equivalent base64 char.
#[inline(always)]
fn encode_base64_char(six_bits: u8) -> u8 {
    let mut offset = 0x41_u16;

    // if (six_bits > 25) offset += 0x61 - 0x41 - 26; // 6
    offset = offset.wrapping_add(25_u16.wrapping_sub(six_bits.into()) >> 8 & 6u16);
    // if (six_bits > 51) offset += 0x30 - 0x61 - 26; // -75
    offset = offset.wrapping_sub(51_u16.wrapping_sub(six_bits.into()) >> 8 & 75u16);
    // if (six_bits > 61) offset += 0x2b - 0x30 - 10; // -15
    offset = offset.wrapping_sub(61_u16.wrapping_sub(six_bits.into()) >> 8 & 15u16);
    // if (six_bits > 62) offset += 0x2f - 0x2b - 1; // 3
    offset = offset.wrapping_add(62_u16.wrapping_sub(six_bits.into()) >> 8 & 3u16);

    (six_bits.wrapping_add(offset as u8))
}

/// Decode six bit encoded base64 char into a six-bit byte.
#[inline(always)]
fn decode_base64_char(chr: u8) -> u8 {
    let mut ret = -1;
    let src = i32::from(chr as i8);

    ret += (((0x40 - src) & (src - 0x5b)) >> 8) & (src - 64);

    ret += (((0x60 - src) & (src - 0x7b)) >> 8) & (src - 70);

    ret += (((0x2f - src) & (src - 0x3a)) >> 8) & (src + 5);

    ret += (((0x2a - src) & (src - 0x2c)) >> 8) & 63;

    ret += (((0x2e - src) & (src - 0x30)) >> 8) & 64;

    ret as u8
}

/// Give the upper bound for the size of the buffer needed to contain the
/// result of encoding a bytestring `sz` bytes long into base64.
#[inline(always)]
pub fn base64_encoded_max_size(sz: usize) -> usize {
    (sz / 3 + 1) * 4
}

/// Give the upper bound for the size of the buffer needed to contain the
/// result of decoding a base64 string `sz` bytes long.
#[inline(always)]
pub fn base64_decoded_max_size(sz: usize) -> usize {
    (sz / 4 + 1) * 3
}

/// Encode a bytestring into base64 in constant-time.
pub fn base64_encode(bytestring: &[u8], add_padding: bool) -> Vec<u8> {
    // Guesstimate needed space (overprovision to avoid reallocs)
    let mut dest = Vec::<u8>::with_capacity(base64_encoded_max_size(bytestring.len()));

    for chunk in bytestring.chunks(3) {
        let b0 = chunk[0];

        match chunk.len() {
            3 => {
                // Process section where no padding needed
                let b1 = chunk[1];
                let b2 = chunk[2];

                dest.push(encode_base64_char(b0 >> 2));
                dest.push(encode_base64_char((b0 << 4 | b1 >> 4) & 63));
                dest.push(encode_base64_char((b1 << 2 | b2 >> 6) & 63));
                dest.push(encode_base64_char(b2 & 63));
            }
            2 => {
                let b1 = chunk[1];

                dest.push(encode_base64_char(b0 >> 2));
                dest.push(encode_base64_char((b0 << 4 | b1 >> 4) & 63));
                dest.push(encode_base64_char((b1 << 2) & 63));
                if add_padding {
                    dest.push(b'=');
                }
            }
            _ => {
                dest.push(encode_base64_char(b0 >> 2));
                dest.push(encode_base64_char(b0 << 4 & 63));
                if add_padding {
                    dest.push(b'=');
                    dest.push(b'=');
                }
            }
        }
    }

    dest
}

/// Decode a base64 encoded bytestring in constant time.
pub fn base64_decode(encoded: &[u8], strict_padding: bool) -> Result<Vec<u8>> {
    if encoded.is_empty() {
        return Ok(vec![]);
    }

    let mut out = Vec::<u8>::with_capacity(base64_decoded_max_size(encoded.len()));

    let encoded_unpad = if strict_padding {
        let mut end_len = encoded.len();
        if end_len & 3 == 0 && encoded[end_len - 1] == b'=' {
            end_len -= 1;
            if encoded[end_len - 1] == b'=' {
                end_len -= 1;
            }
        }
        if end_len & 3 == 1 {
            return Err(ErrorKind::BadPadding);
        }
        if encoded[end_len - 1] == b'=' {
            return Err(ErrorKind::BadPadding);
        }

        &encoded[0..end_len]
    } else {
        let diff = encoded.iter().rev().skip_while(|b| **b == b'=').count();
        let end_len = encoded.len();

        &encoded[0..end_len - (end_len - diff)]
    };

    let mut err = 0;

    for chunk in encoded_unpad.chunks(4) {
        let c0 = decode_base64_char(chunk[0]);

        match chunk.len() {
            4 => {
                // Process section where no padding needed
                let c1 = decode_base64_char(chunk[1]);
                let c2 = decode_base64_char(chunk[2]);
                let c3 = decode_base64_char(chunk[3]);

                out.push((c0 << 2) | (c1 >> 4));
                out.push((c1 << 4) | (c2 >> 2));
                out.push((c2 << 6) | c3);

                err |= i32::from((c0 | c1 | c2 | c3) as i8) >> 8;
            }
            // The last chunk where padding may be needed
            3 => {
                let c1 = decode_base64_char(chunk[1]);
                let c2 = decode_base64_char(chunk[2]);

                out.push((c0 << 2) | (c1 >> 4));
                out.push((c1 << 4) | (c2 >> 2));
                err |= i32::from((c0 | c1 | c2) as i8) >> 8;
            }
            2 => {
                let c1 = decode_base64_char(chunk[1]);

                out.push((c0 << 2) | (c1 >> 4));
                err |= i32::from((c0 | c1) as i8) >> 8;
            }
            _ => if strict_padding {
                err |= 1;
            },
        }
    }

    if err != 0 {
        Err(ErrorKind::InvalidEncodingChar)
    } else {
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use base64::{base64_decode, base64_encode, decode_base64_char, encode_base64_char};

    #[test]
    fn test_encoding() {
        assert_eq!(
            (0u8..64u8)
                .map(|x| encode_base64_char(x))
                .collect::<Vec<u8>>(),
            ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
                .as_bytes()
                .to_owned())
        );
    }

    #[test]
    fn test_decoding() {
        assert_eq!(
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
                .to_owned()
                .iter()
                .map(|x| decode_base64_char(*x))
                .collect::<Vec<u8>>(),
            (0u8..64u8).map(|x| x).collect::<Vec<u8>>()
        );
    }

    #[test]
    fn test_can_encode() {
        assert_eq!(
            &base64_encode(b"Hello, world!", true),
            b"SGVsbG8sIHdvcmxkIQ=="
        );
    }

    #[test]
    fn test_can_decode() {
        assert_eq!(
            &*base64_decode(b"SGVsbG8sIHdvcmxkIQ==", true).unwrap(),
            b"Hello, world!"
        );
    }

    #[test]
    fn test_can_encode_empty() {
        assert_eq!(&base64_encode(b"", true), b"");
    }

    #[test]
    fn test_can_decode_empty() {
        assert_eq!(*base64_decode(b"", true).unwrap(), []);
    }

    quickcheck! {
        fn test_encode_always_correctly_padded(bytes: Vec<u8>) -> bool {
            println!("Trying {:?}", bytes);
            base64_encode(&bytes, true).len() % 4 == 0
        }

        fn test_encode_decode_is_identity(bytes: Vec<u8>) -> bool {
            println!("Trying {:?}", bytes);
            bytes == base64_decode(&base64_encode(&bytes, true), true).unwrap()
        }
    }
}

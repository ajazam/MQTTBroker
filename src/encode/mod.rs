use crate::mqttbroker::primitive_types::{FourByteInteger, TwoByteInteger, VariableByteInteger};
use bytes::{BufMut, BytesMut};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("Number is too large, greater than 268,435,455, to convert to a variable integer")]
    NumberTooLarge,
}

fn encode_two_byte_integer(i: TwoByteInteger, b: &mut BytesMut) {
    b.put_u16(*i.as_ref());
}

fn encode_four_byte_integer(i: FourByteInteger, b: &mut BytesMut) {
    b.put_u32(*i.as_ref());
}

pub fn utf8_encoded_string(s: &str, b: &mut BytesMut) {
    b.put_u16(s.len() as u16);
    b.put_slice(s.as_bytes());
}

pub fn variable_byte_integer(i: &VariableByteInteger, b: &mut BytesMut) -> Result<(), Error> {
    if i.as_ref() > &268_435_455 {
        return Err(Error::NumberTooLarge);
    }
    let mut encoded_byte: u8;
    let mut to_encode = *i.as_ref();
    loop {
        encoded_byte = to_encode.rem_euclid(128u32) as u8;
        to_encode = to_encode.div_euclid(128u32);
        if to_encode > 0 {
            encoded_byte |= 128
        }
        b.put_u8(encoded_byte);
        if to_encode == 0 {
            break;
        }
    }
    Ok(())
}

pub fn binary_data(binary_data: &BytesMut, buffer: &mut BytesMut) {
    let size: u16 = binary_data.len() as u16;
    buffer.put_u16(size);
    buffer.put_slice(binary_data);
}

pub fn utf8_string_pair(key: &str, value: &str, buf: &mut BytesMut) {
    utf8_encoded_string(key, buf);
    utf8_encoded_string(value, buf);
}

#[cfg(test)]
mod test {
    use crate::encode;
    use crate::encode::Error;

    use crate::mqttbroker::primitive_types::VariableByteInteger;
    use bytes::{Buf, BytesMut};

    #[test]
    fn test_encode_128() {
        let mut b = BytesMut::with_capacity(2);
        assert_eq!(
            Ok(()),
            encode::variable_byte_integer(&VariableByteInteger::new(128), &mut b)
        );
        assert_eq!(b.to_vec(), vec![0x80, 1]);
    }

    #[test]
    fn test_encode_256() {
        let mut b = BytesMut::with_capacity(2);
        assert_eq!(
            Ok(()),
            encode::variable_byte_integer(&VariableByteInteger::new(256), &mut b)
        );
        assert_eq!(b.to_vec(), vec![0x80, 2]);
    }

    #[test]
    fn test_encode_32767() {
        let mut b = BytesMut::with_capacity(2);
        assert_eq!(
            Ok(()),
            encode::variable_byte_integer(&VariableByteInteger::new(32767), &mut b)
        );
        assert_eq!(b.to_vec(), vec![0xff, 0xff, 1]);
    }

    #[test]
    fn test_encode_number_too_large() {
        let mut b = BytesMut::with_capacity(4);
        let result = encode::variable_byte_integer(&VariableByteInteger::new(300_000_000), &mut b);
        assert_eq!(Error::NumberTooLarge, result.err().unwrap());
    }

    #[test]
    fn test_string() {
        let mut b = BytesMut::with_capacity(20);
        let s = "hello world";
        encode::utf8_encoded_string(s, &mut b);
        let length = b.get_u16();
        assert_eq!(s.as_bytes(), b.to_vec());
        assert_eq!(length as usize, s.len());
    }
}

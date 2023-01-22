use crate::mqttbroker::primitive_types::{FourByteInteger, TwoByteInteger, VariableByteInteger};
use bytes::{BufMut, BytesMut};
use thiserror::Error;
use tracing::trace;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum EncodeError {
    #[error("Number is too large, greater than 268,435,455, to convert to a variable integer")]
    NumberTooLarge,
}

fn encode_two_byte_integer(name: &str, i: TwoByteInteger, b: &mut BytesMut) {
    b.put_u16(*i.as_ref());
    let integer_bytes = b.split_off(b.len() - 3);
    trace!("encoding two byte integer {name}, value = {i:?}, encoded = {integer_bytes:?}");
    b.unsplit(integer_bytes);
}

fn encode_four_byte_integer(name: &str, i: FourByteInteger, b: &mut BytesMut) {
    b.put_u32(*i.as_ref());
    trace!("encoding four byte integer {name}, value = {i:?}");
}

pub fn utf8_encoded_string(name: &str, s: &str, b: &mut BytesMut) {
    b.put_u16(s.len() as u16);
    b.put_slice(s.as_bytes());
    trace!("encoding utf8_encoded_string {name}, value = {s}");
}

pub fn variable_byte_integer(
    name: &str,
    i: &VariableByteInteger,
    b: &mut BytesMut,
) -> Result<(), EncodeError> {
    if i.as_ref() > &268_435_455 {
        return Err(EncodeError::NumberTooLarge);
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
    trace!("encoding variable_byte_integer {name}, value: {i:?}");
    Ok(())
}

pub fn binary_data(name: &str, binary_data: &BytesMut, buffer: &mut BytesMut) {
    let size: u16 = binary_data.len() as u16;
    buffer.put_u16(size);
    buffer.put_slice(binary_data);
    trace!("encoding binary_data {name}, value = {binary_data:?}");
}

pub fn utf8_string_pair(
    key_name: &str,
    value_name: &str,
    key: &str,
    value: &str,
    buf: &mut BytesMut,
) {
    utf8_encoded_string(key_name, key, buf);
    utf8_encoded_string(value_name, value, buf);

    trace!("encoded utf8_string_pair key = {key_name}, value = {value_name}")
}

#[cfg(test)]
mod test {
    use crate::encode;
    use crate::encode::EncodeError;
    use crate::mqttbroker::primitive_types::VariableByteInteger;
    use bytes::{Buf, BytesMut};

    #[test]
    fn test_encode_128() {
        let mut b = BytesMut::with_capacity(2);
        assert_eq!(
            Ok(()),
            encode::variable_byte_integer("", &VariableByteInteger::new(128), &mut b)
        );
        assert_eq!(b.to_vec(), vec![0x80, 1]);
    }

    #[test]
    fn test_encode_256() {
        let mut b = BytesMut::with_capacity(2);
        assert_eq!(
            Ok(()),
            encode::variable_byte_integer("", &VariableByteInteger::new(256), &mut b)
        );
        assert_eq!(b.to_vec(), vec![0x80, 2]);
    }

    #[test]
    fn test_encode_32767() {
        let mut b = BytesMut::with_capacity(2);
        assert_eq!(
            Ok(()),
            encode::variable_byte_integer("", &VariableByteInteger::new(32767), &mut b)
        );
        assert_eq!(b.to_vec(), vec![0xff, 0xff, 1]);
    }

    #[test]
    fn test_encode_number_too_large() {
        let mut b = BytesMut::with_capacity(4);
        let result =
            encode::variable_byte_integer("", &VariableByteInteger::new(300_000_000), &mut b);
        assert_eq!(EncodeError::NumberTooLarge, result.err().unwrap());
    }

    #[test]
    fn test_string() {
        let mut b = BytesMut::with_capacity(20);
        let s = "hello world";
        encode::utf8_encoded_string("", s, &mut b);
        let length = b.get_u16();
        assert_eq!(s.as_bytes(), b.to_vec());
        assert_eq!(length as usize, s.len());
    }
}

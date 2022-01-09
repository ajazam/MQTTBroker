mod decode;
mod encode;
mod mqttbroker;

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

#[macro_use]
extern crate lazy_static;

use bytes::{Buf, BufMut, BytesMut};

fn main() {
    println!("Hello, world!");
    let b = BytesMut::with_capacity(100);

    let mut b1 = BytesMut::with_capacity(100);
    b1.put_u8(1);
    b1.put_u8(2);
    b1.put_u8(3);
    b1.put_u8(4);
    b1.put_u8(5);
    b1.put_u8(6);
    let v1 = b1.get_u8();
    let v2 = b1.get_u8();
    let mut iterator = b1.iter();
    println!("{:?}", iterator.next());
    println!("{:?}", iterator.next());
    iterator.next();
    iterator.next();
    println!("buffer is {:?}", b1);
    //let mut reader = b1.reader();
    let v3 = b1.get_u8();
    //println!("reader is {:?}", reader);
    println!("v3 is {:?}", v3);
}

#[cfg(test)]
mod tests {
    use bytes::{Buf, BufMut, BytesMut};

    #[cfg(test)]
    extern crate quickcheck;

    #[test]
    fn test() {
        let mut buf = BytesMut::with_capacity(20);
        buf.put_u32(1024000);
        let i = buf.get_u32();

        assert_eq!(i, 1024000);
    }

    #[quickcheck]
    fn test2(i: u32) -> bool {
        let mut buf = BytesMut::with_capacity(4);
        buf.put_u32(i);
        i == buf.get_u32()
    }
}

mod decode;
mod encode;
mod mqttbroker;
mod server;

#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

#[macro_use]
extern crate lazy_static;

use bytes::{Buf, BufMut, BytesMut};
use tracing::{debug, event, span, Level};
use tracing_subscriber::fmt;

extern crate core;

extern crate tracing;

fn main() {
    let format = fmt::format()
        .with_level(false) // don't include levels in formatted output
        .with_target(false) // don't include targets
        .with_thread_ids(true) // include the thread ID of the current thread
        .with_thread_names(true) // include the name of the current thread
        .compact(); // use the `Compact` formatting style.
    tracing_subscriber::fmt().event_format(format).init();
    let span = span!(Level::INFO, "my first span");
    let _guard = span.enter();
    event!(Level::INFO, "Hello, world!");
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
    debug!("{:?}", iterator.next());
    debug!("{:?}", iterator.next());
    iterator.next();
    iterator.next();
    event!(Level::INFO, "buffer is {:?}", b1);
    //let mut reader = b1.reader();
    let v3 = b1.get_u8();
    //debug!("reader is {:?}", reader);
    event!(Level::INFO, "v3 is {:?}", v3);
    print!("yello");
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

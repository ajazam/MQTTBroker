pub fn add(left: usize, right: usize) -> usize {
    left + right
}

mod processors {
    mod actors {
        mod control_packet {
            use std::sync::mpsc;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

use tokio::net::TcpListener;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub enum ClientError {}

struct Client {
    addr: String,
    connection: TcpListener,
}

impl Client {
    pub fn connect(addr: String, user_name: String, password: String) -> Result<(), ClientError> {
        todo!()
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

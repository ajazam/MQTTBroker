struct Broker {}

#[cfg(test)]
mod test {

    #[tokio::test]
    async fn test_login() {
        // send connect packets to broker
        // let (tx, mut rx) = mpsc::channel();
        // tokio::spawn(async move {});
        // // wait for connack
        // assert!(false);
    }

    #[tokio::test]
    async fn test_disconnect() {
        assert!(true)
    }
}

// packet specific tests

//

// global tests

mod actor {

    pub(crate) mod actor_template {
        use tokio::sync::mpsc;

        pub trait Actor {
            // enum type which will hold the different types of messages
            type Message;
            fn new(receiver: mpsc::Receiver<Self::Message>) -> Self;
            fn getReceiver(&self) -> mpsc::Receiver<Self::Message>;
            fn handle_message(&mut self, msg: Self::Message);
        }

        pub async fn run_my_actor(mut actor: impl Actor) {
            while let Some(msg) = actor.getReceiver().recv().await {
                actor.handle_message(msg);
            }
        }

        pub trait ActorHandler {
            fn new() -> Self;
        }
    }

    // receive connect packet, authenticates user and returns connack packet.
    // requires access to an Authentication actor
    pub mod connect {
        // use crate::actor::actor_template::Actor;
        // use deser::packets::connect::Connect;
        // use tokio::sync::mpsc::Receiver;
        // use tokio::sync::oneshot;
        //
        // struct ConnectActor {
        //     receiver: Receiver<ConnectActorMessage>,
        //     // some internal state here
        // }
        //
        // enum ConnectActorMessage {
        //     Connect {
        //         packet: Connect,
        //         respond_to: oneshot::Sender<()>,
        //     },
        // }
        //
        // impl Actor for ConnectActor {
        //     type Message = ConnectActorMessage;
        //
        //     fn new(receiver: Receiver<ConnectActorMessage>) -> Self {
        //         ConnectActor {
        //             receiver, /*, state here */
        //         }
        //     }
        //
        //     fn getReceiver(self) -> Receiver<ConnectActorMessage> {
        //         self.receiver
        //     }
        //
        //     fn handle_message(&mut self, msg: ConnectActorMessage) {
        //         match msg {
        //             ConnectActorMessage::Connect { .. } => {}
        //         }
        //     }
        // }

        // connect packet task.
        // check for valid username/password and return connack
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {

        // assert_eq!(result, 4);
    }
}

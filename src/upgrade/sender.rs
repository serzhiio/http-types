use crate::upgrade::Connection;

/// The sending half of a channel to send an upgraded connection.
///
/// Unlike `async_std::sync::channel` the `send` method on this type can only be
/// called once, and cannot be cloned. That's because only a single instance of
/// `Connection` should be created.
#[derive(Debug)]
pub struct Sender {
    sender: async_channel::Sender<Connection>,
}

impl Sender {
    /// Create a new instance of `Sender`.
    #[doc(hidden)]
    pub fn new(sender: async_channel::Sender<Connection>) -> Self {
        Self { sender }
    }

    /// Send a `Trailer`.
    ///
    /// The channel will be consumed after having sent trailers.
    pub async fn send(self, trailers: Connection) {
        self.sender.send(trailers).await
    }
}

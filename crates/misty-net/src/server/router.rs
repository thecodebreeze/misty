use crate::{Frame, MistyCodec, MistyService, NetError, Status, TailFrame};
use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use tokio_util::codec::Framed;

/// A router is a registry of services that can be dispatched based on the service name.
///
/// This is used by the MistyServer implementation to announce services to peers.
pub struct Router {
    /// Registry of services that are routable.
    services: HashMap<[u8; 32], Box<dyn MistyService>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
        }
    }

    /// Registers a new service into the router.
    pub fn register<S>(&mut self, service_id: [u8; 32], service: S)
    where
        S: MistyService + 'static,
    {
        self.services.insert(service_id, Box::new(service));
    }

    #[tracing::instrument(
        skip(self, stream, sender),
        fields(
            stream = ?stream.id(),
            sender = ?sender.id()
        )
    )]
    pub async fn route(
        &self,
        stream: quinn::RecvStream,
        sender: quinn::SendStream,
    ) -> Result<(), NetError> {
        // Wrap stream and sender in our framed codec, which is Length-Delimited.
        let mut stream = Framed::new(stream, MistyCodec::default());
        let mut sender = Framed::new(sender, MistyCodec::default());

        // Read the [HeadFrame] from the stream. It must be a [HeadFrame] otherwise we have a
        // HandshakeViolation error.
        let head = stream.next().await;
        let head = match head {
            Some(Ok(Frame::Head(head))) => head,
            Some(Ok(_)) => {
                return Self::send_error(
                    &mut sender,
                    Status::HandshakeViolation,
                    "Handshake Violation: Expected HeadFrame, got something else.",
                )
                .await;
            }
            Some(Err(error)) => {
                let status = match error {
                    NetError::StdIo(error) => {
                        tracing::debug!(?error, "Internal error while processing the HeadFrame");
                        Status::InternalServerError
                    }
                    NetError::Status(status) => {
                        tracing::debug!(error = ?status, "Internal protocol error while processing the HeadFrame");
                        status
                    }
                };

                return Self::send_error(
                    &mut sender,
                    status,
                    "Handshake Violation: Error reading HeadFrame.",
                )
                .await;
            }
            None => {
                tracing::trace!("Stream closed before HeadFrame arrived");
                return Err(NetError::Status(Status::HandshakeViolation));
            }
        };

        // Execute the routing.
        if let Some(service) = self.services.get(&head.service_id) {
            service.handle(head, stream, sender).await;
        } else {
            tracing::trace!(service_id = ?head.service_id, "Service not found");
            Self::send_error(&mut sender, Status::ServiceNotFound, "Service not found").await?;
        }

        Ok(())
    }

    /// Helper function to send an error response to the peer.
    #[tracing::instrument(skip(framed_sender))]
    async fn send_error(
        framed_sender: &mut Framed<quinn::SendStream, MistyCodec>,
        status: Status,
        message: &str,
    ) -> Result<(), NetError> {
        tracing::trace!(status = ?status, message = ?message, "Sending error response to peer");

        // Build the TailFrame to send back to the client.
        let tail = Frame::Tail(TailFrame {
            status: status.as_u16(),
        });

        // Send the TailFrame.
        framed_sender.send(tail).await?;
        framed_sender.close().await?;

        Ok(())
    }
}

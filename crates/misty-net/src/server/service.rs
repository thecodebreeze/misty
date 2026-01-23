use crate::{HeadFrame, MistyCodec, Status};
use tokio_util::codec::Framed;

/// This trait is used in code generation and by the router to register services and dispatch
/// function calls.
#[async_trait::async_trait]
pub trait MistyService {
    /// Handle a request stream from the remote peer.
    ///
    /// This method receives the streams that are already framed by the [MistyCodec].
    async fn handle(
        &self,
        head: HeadFrame,
        stream: Framed<quinn::RecvStream, MistyCodec>,
        sender: Framed<quinn::SendStream, MistyCodec>,
    ) -> Status;
}

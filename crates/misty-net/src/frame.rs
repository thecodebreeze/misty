use serde::{Deserialize, Serialize};

/// Frame definition for QUIC messages.
#[derive(Debug, Serialize, Deserialize)]
pub enum Frame {
    /// Very first frame of any stream, contains the data needed to route the data sent.
    Head(HeadFrame),

    /// Data required to run the processing of the remote call.
    Data(bytes::Bytes),

    /// Very last frame of any stream, contains the status of the remote call.
    Tail(TailFrame),
}

/// The head frame contains the service and method IDs.
///
/// This must only be sent once to make the handshake and establish a stream between the peers.
#[derive(Debug, Serialize, Deserialize)]
pub struct HeadFrame {
    /// Unique discriminator of the service to be called.
    pub service_id: [u8; 32],

    /// Unique discriminator of the method to be called.
    pub method_id: [u8; 32],
}

/// The tail frame contains the status of the remote call.
///
/// This must only be sent once at the end of the interaction; by protocol when a peer acknowledges
/// this frame, the stream will be closed.
#[derive(Debug, Serialize, Deserialize)]
pub struct TailFrame {
    /// Status of the remote call.
    pub status: u16,
}

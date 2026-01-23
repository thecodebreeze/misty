use crate::{Frame, NetError, Status};
use bytes::Bytes;
use tokio_util::bytes::BytesMut;
use tokio_util::codec::{Decoder, Encoder, LengthDelimitedCodec};

#[derive(Default)]
pub struct MistyCodec {
    inner: LengthDelimitedCodec,
}

impl Decoder for MistyCodec {
    type Item = Frame;
    type Error = NetError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        // Let the inner coded handle the length prefix chopping.
        let packet_bytes = match self.inner.decode(src)? {
            Some(packet_bytes) => packet_bytes,
            None => return Ok(None),
        };

        // Deserialize the bytes into our Frame enum.
        rmp_serde::from_slice(&packet_bytes)
            .inspect_err(|error| {
                tracing::debug!(?error, "Failed to deserialize packet bytes.");
            })
            .map_err(|_| NetError::Status(Status::MalformedFrame))
            .map(Some)
    }
}

impl Encoder<Frame> for MistyCodec {
    type Error = NetError;

    fn encode(&mut self, item: Frame, dst: &mut BytesMut) -> Result<(), Self::Error> {
        // Serialize the frame into bytes.
        let payload = rmp_serde::to_vec(&item)
            .inspect_err(|error| {
                tracing::debug!(?error, "Failed to serialize packet bytes.");
            })
            .map_err(|_| NetError::Status(Status::MalformedFrame))?;

        // Wrap it with the length prefix and write to the destination.
        let payload_bytes = Bytes::from(payload);
        self.inner
            .encode(payload_bytes, dst)
            .inspect_err(|error| {
                tracing::debug!(?error, "Failed to encode packet bytes.");
            })
            .map_err(|_| NetError::Status(Status::MalformedFrame))
    }
}

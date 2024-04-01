use crate::common::error::P2PError;
use reth_ecies::stream::ECIESStream;
use reth_primitives::{pk2id, NodeRecord};
use secp256k1::{SecretKey, SECP256K1};
use std::{net::IpAddr, time::Duration};
use tokio::net::TcpStream;
use tracing::{debug, error, info, instrument, trace};

// stream related imports
use alloy_rlp::{Decodable, Encodable};
use futures::{SinkExt, StreamExt};
use reth_eth_wire::{
    errors::{P2PHandshakeError, P2PStreamError},
    DisconnectReason, HelloMessage, P2PMessage,
};
use reth_primitives::{bytes::BytesMut, hex};

/// [`MAX_PAYLOAD_SIZE`] is the maximum size of an uncompressed message payload.
/// This is defined in [EIP-706](https://eips.ethereum.org/EIPS/eip-706).
pub const MAX_PAYLOAD_SIZE: usize = 16 * 1024 * 1024;

#[derive(Debug)]
pub struct NodeConfig {
    pub timeout: u64,
    pub peer: NodeRecord,
}

impl NodeConfig {
    /// Perform a P2P handshake with a peer
    #[instrument(level = "trace", skip_all, fields(peer=&*format!("{:?}", self.peer.address)))]
    pub async fn handshake(&self) -> Result<IpAddr, P2PError> {
        info!("[{:?}] Perform a P2P handshake", self.peer.address);

        let key = SecretKey::new(&mut rand::thread_rng());

        debug!(
            "[{:?}] Send and Parse the ECIES auth message",
            self.peer.address
        );
        let ecies_stream = {
            // Connect to the peer and perform the ECIES handshake
            let outgoing = tokio::time::timeout(
                Duration::from_millis(self.timeout),
                TcpStream::connect((self.peer.address, self.peer.tcp_port)),
            )
            .await??;
            ECIESStream::connect(outgoing, key, self.peer.id).await?
        };

        // Send, Parse the P2P Hello message and perform the initial handshake
        debug!(
            "[{:?}] Send, Parse the P2P Hello message and perform the initial handshake",
            self.peer.address
        );
        let hello_msg = Self::create_hello_msg(key);

        Self::handle_ecies_stream(&self, ecies_stream, hello_msg).await?;

        Ok(self.peer.address)
    }

    /// Create a P2P Hello message
    pub fn create_hello_msg(key: SecretKey) -> HelloMessage {
        let our_peer_id = pk2id(&key.public_key(SECP256K1));
        HelloMessage::builder(our_peer_id).build().into_message()
    }

    pub async fn handle_ecies_stream(
        &self,
        mut stream: ECIESStream<TcpStream>,
        hello_msg: HelloMessage,
    ) -> Result<(), P2PError> {
        // Convert the hello msg into raw bytes
        let mut raw_hello_bytes = BytesMut::new();
        hello_msg.encode(&mut raw_hello_bytes);
        // Send our hello msg
        debug!(?hello_msg, "Sending Hello:");
        stream.send(raw_hello_bytes.into()).await?;

        // Receive the msg from the peer
        trace!("waiting for message from peer");
        let first_message_bytes =
            tokio::time::timeout(Duration::from_millis(self.timeout), stream.next())
                .await
                .or(Err(P2PStreamError::HandshakeError(
                    P2PHandshakeError::Timeout,
                )))?
                .ok_or(P2PStreamError::HandshakeError(
                    P2PHandshakeError::NoResponse,
                ))??;

        // let's check the compressed length first, we will need to check again once confirming
        // that it contains snappy-compressed data (this will be the case for all non-p2p messages).
        if first_message_bytes.len() > MAX_PAYLOAD_SIZE {
            return Err(P2PStreamError::MessageTooBig {
                message_size: first_message_bytes.len(),
                max_size: MAX_PAYLOAD_SIZE,
            })?;
        }

        // The first message sent MUST be a hello OR disconnect message
        // to finalize the handshake.
        trace!(?first_message_bytes, "received first message from peer");
        match P2PMessage::decode(&mut &first_message_bytes[..]) {
            Ok(P2PMessage::Hello(hello)) => {
                debug!("Received Hello: {:?}", hello);
                Ok(hello)
            }
            Ok(P2PMessage::Disconnect(reason)) => {
                debug!(?reason, "Disconnected by peer during handshake:");
                Err(P2PStreamError::HandshakeError(
                    P2PHandshakeError::Disconnected(reason),
                ))
            }
            Err(err) => {
                error!(
                    "Failed to decode first message from peer: {}, msg={}",
                    err,
                    hex::encode(&first_message_bytes)
                );
                Err(P2PStreamError::HandshakeError(err.into()))
            }
            Ok(msg) => {
                debug!(?msg, "expected hello message but received:");
                Err(P2PStreamError::HandshakeError(
                    P2PHandshakeError::NonHelloMessageInHandshake,
                ))
            }
        }?;

        // Send disconnect message to avoid keeping the connection alive with peer
        trace!("sending disconnect message to peer");
        let mut raw_disconnect_byte = BytesMut::new();
        P2PMessage::Disconnect(DisconnectReason::ClientQuitting).encode(&mut raw_disconnect_byte);
        stream.send(raw_disconnect_byte.into()).await?;

        Ok(())
    }
}

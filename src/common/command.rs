use crate::{
    common::error::{P2PError, P2PHandshakeErr},
    p2p::eth::NodeConfig,
};
use clap::{command, Parser, Subcommand};
use reth_primitives::NodeRecord;
use std::net::IpAddr;
use tokio::task::JoinHandle;
use tracing::{error, info};

/// [`HANDSHAKE_TIMEOUT`] sets the time to wait before determining that the handshake has timed out.
pub const HANDSHAKE_TIMEOUT: u64 = 1000;

#[derive(Parser, Debug)]
pub struct CmdArgs {
    #[command(subcommand)]
    pub commands: Commands,

    #[arg(
        default_value_t = HANDSHAKE_TIMEOUT,
        help = "handshake operation maximum time (in ms)"
    )]
    pub timeout: u64,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Perform a P2P handshake with the ethereum network nodes
    Eth { nodes_addrs: Vec<NodeRecord> },
}

impl CmdArgs {
    /// Perform a P2P handshake with a peer for each node in the network
    pub async fn handshake(self) -> Result<(), P2PError> {
        let tasks: Vec<JoinHandle<Result<IpAddr, P2PError>>> = match self.commands {
            Commands::Eth { nodes_addrs } => nodes_addrs
                .into_iter()
                .map(|node| {
                    let eth_handshake = NodeConfig {
                        timeout: self.timeout,
                        peer: node,
                    };
                    tokio::spawn(async move {
                        eth_handshake.handshake().await.map_err(|err| {
                            P2PError::P2PHandshakeError(P2PHandshakeErr::new(
                                err,
                                node.address.clone().to_string(),
                            ))
                        })
                    })
                })
                .collect(),
        };

        // Wait for all the tasks to complete
        for task in tasks {
            match task.await? {
                Ok(addr) => info!("[successful] [{:?}] ", addr),
                Err(err) => error!("{}", err),
            }
        }
        Ok(())
    }
}

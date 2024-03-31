use crate::{
    common::error::{P2PError, P2PHandshakeErr},
    p2p::{
        command::{CmdArgs, Commands},
        eth::NodeConfig,
    },
};
use futures_util::TryFutureExt;
use std::net::IpAddr;
use tokio::task::JoinHandle;
use tracing::{error, info};

/// Perform a P2P handshake with a peer for each node in the network
pub async fn handshake(cmd_args: CmdArgs) -> Result<(), P2PError> {
    let tasks: Vec<JoinHandle<Result<IpAddr, P2PError>>> = match cmd_args.commands {
        Commands::Eth { nodes_addrs } => nodes_addrs
            .into_iter()
            .map(|node| {
                let eth_handshake = NodeConfig {
                    timeout: cmd_args.timeout,
                    peer: node,
                };
                tokio::spawn(async move {
                    eth_handshake
                        .handshake()
                        .map_err(move |err| {
                            P2PError::P2PHandshakeError(P2PHandshakeErr::new(
                                err,
                                node.address.clone().to_string(),
                            ))
                        })
                        .await
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

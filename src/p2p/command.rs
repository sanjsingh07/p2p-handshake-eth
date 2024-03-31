use clap::{command, Parser, Subcommand};
use reth_primitives::NodeRecord;

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

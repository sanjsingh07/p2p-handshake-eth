use clap::Parser;
use p2p_handshake_eth::{
    common::error::P2PError,
    p2p::{command::CmdArgs, handshake::handshake},
};
use tracing::info;
use tracing_subscriber::{filter::LevelFilter, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), P2PError> {
    // Initiate tracing with info as default
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    let cmd_args = CmdArgs::parse();

    info!("loaded cmd args: {:?}", &cmd_args);

    handshake(cmd_args).await?;

    Ok(())
}

use p2p_handshake_eth::{common::command::HANDSHAKE_TIMEOUT, p2p::eth::NodeConfig};
use reth_primitives::NodeRecord;
use std::str::FromStr;
// use reth_primitives::holesky_nodes; // incase we don't want to bother mainnet nodes.
use tracing_subscriber::{filter::LevelFilter, EnvFilter};

// mainnet nodes
const TEST_NODES_ENODES: [&str; 5] = [
    "enode://2d941dcc0f17769e1d93acaa9412eef5a03411b26b72561d0debd6cf11948019cf52fd71334a67c7c9830beef030e5644e6fa5ef97275404e5f3ce0823b7dfcc@136.243.102.80:30303",
    "enode://37e79d9bdb017a0dbbc1f4851f280881f1fba6110c078144279b879d7235c0b80ccc84fd8cb1080e7897fbf0f322cdeeaa545de65116f84b46ca1a97203720d9@167.235.12.71:15701",
    "enode://233f296f0de5f652fd6435b51e43846250c6cea4480f233cafd735ee5d94ae540be133760b767277e8a226eaaa80cca1ae5d41f09c3dd3bc4e3e0ea1eb5bb4d6@15.204.197.100:30403",
    "enode://3fa4486f8d27f1a351b832f0e172691b69ddb19ff216f15052dff1e6af5994ae8789c723227788cf72263998dc42d6dd8df7b7893479ae7b5741c352bc6f6b20@76.17.9.80:30404",
    "enode://d160ac836263e521f50589ff27caf7cfeeef6e0b977987b9fbfe4c551454b84972f3a655ab30d3f97c7ca2e6e3212339888443710f1254df5ad93d49d6ef917f@68.201.65.25:30303"
];

#[tokio::test]
async fn handshake_test() {
    // Initiate tracing with info as default
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::DEBUG.into())
        .from_env_lossy();
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    // We can also use the holesky nodes to avoid bothering the Ethereum mainnet
    // let nodes_addrs = holesky_nodes();

    // converting encode to NodeRecord
    let nodes_addrs: Vec<NodeRecord> = TEST_NODES_ENODES
        .into_iter()
        .map(|test_node| NodeRecord::from_str(test_node).unwrap())
        .collect();

    // Iterate over the nodes and perform the P2P handshake
    for peer in nodes_addrs {
        let node_config = NodeConfig {
            timeout: HANDSHAKE_TIMEOUT,
            peer,
        };
        let _ = node_config.handshake().await;
    }
}

# P2P node handshake
## problem statement :
_Spec v3 (2023-03-30)_

Pick a publicly available P2P node (e.g. a blockchain one) implementation - which itself doesn't need to be written in Rust - and write a [network handshake](https://en.wikipedia.org/wiki/Handshaking) for it in Rust, and instructions on how to test it.

## Requirements

- Both the target node and the handshake code should compile at least on Linux.
- The solution has to perform a full **protocol-level** (post-TCP/etc.) handshake with the target node.
- The provided **instructions** should include information on how to verify that the handshake has concluded.
- The solution can not depend on the code of the target node (but it can share some of its dependencies).
- The submitted code can not reuse entire preexisting handshake implementations like `libp2p_noise/XX`.

### Non-requirements

- A lot of parameters can potentially be exchanged during the handshake, but only the mandatory ones need to be included.
- The solution can ignore any post-handshake traffic from the target node, and it doesn't have to keep the connection alive.

## Evaluation

- **Quality**: the solution should be idiomatic and adhere to Rust coding conventions.
- **Performance**: the solution should be as fast as the handshake protocol allows, and it shouldn't block resources.
- **Security**: the network is an inherently untrusted environment, and it should be taken into account.
- **Minimalism**: any number of dependencies can be used, but they should be tailored to the task.
- **Versatility**: the more broadly applicable the solution is (bi-directional, unfixed values, etc.), the better.
- **Uniqueness**: bonus points for non-Bitcoin implementations.

## Solution :
## Usage

You can obtain a list of Enode ID's from the following here:
[Ethereum nodes](https://etherscan.io/nodetracker/nodes)

```shell
$ cargo run <timeout in milliseconds> eth enode://<node_id@ip_address:port> enode://<node_id@ip_address:port>
```

If a timeout is not supplied, the default request timeout is set 1000 milliseconds.

## Possible Output

You can set the log level to `debug` or `trace` by using the RUST_LOG environment variable to get more detailed information about the handshake process, by default it set to `info`.

```shell
$ export RUST_LOG=debug
$ cargo run eth enode://2d941dcc0f17769e1d93acaa9412eef5a03411b26b72561d0debd6cf11948019cf52fd71334a67c7c9830beef030e5644e6fa5ef97275404e5f3ce0823b7dfcc@136.243.102.80:30303 enode://37e79d9bdb017a0dbbc1f4851f280881f1fba6110c078144279b879d7235c0b80ccc84fd8cb1080e7897fbf0f322cdeeaa545de65116f84b46ca1a97203720d9@167.235.12.71:15701 enode://233f296f0de5f652fd6435b51e43846250c6cea4480f233cafd735ee5d94ae540be133760b767277e8a226eaaa80cca1ae5d41f09c3dd3bc4e3e0ea1eb5bb4d6@15.204.197.100:30403
    Finished dev [unoptimized + debuginfo] target(s) in 0.64s
     Running `target/debug/p2p-handshake-eth eth 'enode://2d941dcc0f17769e1d93acaa9412eef5a03411b26b72561d0debd6cf11948019cf52fd71334a67c7c9830beef030e5644e6fa5ef97275404e5f3ce0823b7dfcc@136.243.102.80:30303' 'enode://37e79d9bdb017a0dbbc1f4851f280881f1fba6110c078144279b879d7235c0b80ccc84fd8cb1080e7897fbf0f322cdeeaa545de65116f84b46ca1a97203720d9@167.235.12.71:15701' 'enode://233f296f0de5f652fd6435b51e43846250c6cea4480f233cafd735ee5d94ae540be133760b767277e8a226eaaa80cca1ae5d41f09c3dd3bc4e3e0ea1eb5bb4d6@15.204.197.100:30403'`
2024-04-01T10:19:46.922915Z  INFO p2p_handshake_eth: loaded cmd args: CmdArgs { commands: Eth { nodes_addrs: [NodeRecord { address: 136.243.102.80, tcp_port: 30303, udp_port: 30303, id: 0x2d941dcc0f17769e1d93acaa9412eef5a03411b26b72561d0debd6cf11948019cf52fd71334a67c7c9830beef030e5644e6fa5ef97275404e5f3ce0823b7dfcc }, NodeRecord { address: 167.235.12.71, tcp_port: 15701, udp_port: 15701, id: 0x37e79d9bdb017a0dbbc1f4851f280881f1fba6110c078144279b879d7235c0b80ccc84fd8cb1080e7897fbf0f322cdeeaa545de65116f84b46ca1a97203720d9 }, NodeRecord { address: 15.204.197.100, tcp_port: 30403, udp_port: 30403, id: 0x233f296f0de5f652fd6435b51e43846250c6cea4480f233cafd735ee5d94ae540be133760b767277e8a226eaaa80cca1ae5d41f09c3dd3bc4e3e0ea1eb5bb4d6 }] }, timeout: 1000 }
2024-04-01T10:19:46.923273Z  INFO p2p_handshake_eth::p2p::eth: [136.243.102.80] Perform a P2P handshake
2024-04-01T10:19:46.923275Z  INFO p2p_handshake_eth::p2p::eth: [15.204.197.100] Perform a P2P handshake
2024-04-01T10:19:46.923274Z  INFO p2p_handshake_eth::p2p::eth: [167.235.12.71] Perform a P2P handshake
2024-04-01T10:19:46.923636Z DEBUG p2p_handshake_eth::p2p::eth: [136.243.102.80] Send and Parse the ECIES auth message
2024-04-01T10:19:46.923649Z DEBUG p2p_handshake_eth::p2p::eth: [167.235.12.71] Send and Parse the ECIES auth message
2024-04-01T10:19:46.923653Z DEBUG p2p_handshake_eth::p2p::eth: [15.204.197.100] Send and Parse the ECIES auth message
2024-04-01T10:19:47.392292Z DEBUG p2p_handshake_eth::p2p::eth: [167.235.12.71] Send, Parse the P2P Hello message and perform the initial handshake
2024-04-01T10:19:47.392749Z DEBUG p2p_handshake_eth::p2p::eth: Sending Hello: HelloMessage { protocol_version: V5, client_version: "reth/v0.2.0-beta.4", capabilities: [Capability { name: "eth", version: 68 }, Capability { name: "eth", version: 67 }, Capability { name: "eth", version: 66 }], port: 30303, id: 0x2aa90f4613df36fdd7ac4ea9122e3fc4d6232cec3aaa33f260e43c49fa8e0f19d64d72f7f01f1d300567be0664d757028ab40770570df71297dc254bdcbbb49e }
2024-04-01T10:19:47.393735Z DEBUG p2p_handshake_eth::p2p::eth: Received Hello: HelloMessage { protocol_version: V5, client_version: "Nethermind/v1.25.4+20b10b35/linux-x64/dotnet8.0.2", capabilities: [Capability { name: "eth", version: 66 }, Capability { name: "nodedata", version: 1 }, Capability { name: "eth", version: 67 }, Capability { name: "eth", version: 68 }, Capability { name: "snap", version: 1 }], port: 15701, id: 0x37e79d9bdb017a0dbbc1f4851f280881f1fba6110c078144279b879d7235c0b80ccc84fd8cb1080e7897fbf0f322cdeeaa545de65116f84b46ca1a97203720d9 }
2024-04-01T10:19:47.403852Z DEBUG p2p_handshake_eth::p2p::eth: [136.243.102.80] Send, Parse the P2P Hello message and perform the initial handshake
2024-04-01T10:19:47.404150Z DEBUG p2p_handshake_eth::p2p::eth: Sending Hello: HelloMessage { protocol_version: V5, client_version: "reth/v0.2.0-beta.4", capabilities: [Capability { name: "eth", version: 68 }, Capability { name: "eth", version: 67 }, Capability { name: "eth", version: 66 }], port: 30303, id: 0xa952b26ff4475189bff403002de5fd565104d383e0e4a13cc938f82287adb912f794baf981071a4822b997ac25b551be96edf40611a26bdb45cfc934e545026c }
2024-04-01T10:19:47.404921Z DEBUG p2p_handshake_eth::p2p::eth: Received Hello: HelloMessage { protocol_version: V5, client_version: "Nethermind/v1.25.4+20b10b35/linux-x64/dotnet8.0.2", capabilities: [Capability { name: "eth", version: 66 }, Capability { name: "nodedata", version: 1 }, Capability { name: "eth", version: 67 }, Capability { name: "eth", version: 68 }, Capability { name: "snap", version: 1 }], port: 30303, id: 0x2d941dcc0f17769e1d93acaa9412eef5a03411b26b72561d0debd6cf11948019cf52fd71334a67c7c9830beef030e5644e6fa5ef97275404e5f3ce0823b7dfcc }
2024-04-01T10:19:47.405308Z  INFO p2p_handshake_eth::p2p::handshake: [successful] [136.243.102.80] 
2024-04-01T10:19:47.405322Z  INFO p2p_handshake_eth::p2p::handshake: [successful] [167.235.12.71] 
2024-04-01T10:19:47.446306Z DEBUG p2p_handshake_eth::p2p::eth: [15.204.197.100] Send, Parse the P2P Hello message and perform the initial handshake
2024-04-01T10:19:47.446607Z DEBUG p2p_handshake_eth::p2p::eth: Sending Hello: HelloMessage { protocol_version: V5, client_version: "reth/v0.2.0-beta.4", capabilities: [Capability { name: "eth", version: 68 }, Capability { name: "eth", version: 67 }, Capability { name: "eth", version: 66 }], port: 30303, id: 0xee5a7c52d93beaa439fe4f721bc468b4a7e589f6ca1982eded60c75ab43798473e2e7e681d6152a163f70a8ff2e841c4bb3723a8f4908053c0545d15fb5bb017 }
2024-04-01T10:19:47.447420Z DEBUG p2p_handshake_eth::p2p::eth: Received Hello: HelloMessage { protocol_version: V5, client_version: "erigon/v1.1.15-dev-9a42b90a/linux-amd64/go1.20.2", capabilities: [Capability { name: "eth", version: 66 }], port: 0, id: 0x233f296f0de5f652fd6435b51e43846250c6cea4480f233cafd735ee5d94ae540be133760b767277e8a226eaaa80cca1ae5d41f09c3dd3bc4e3e0ea1eb5bb4d6 }
2024-04-01T10:19:47.447707Z  INFO p2p_handshake_eth::p2p::handshake: [successful] [15.204.197.100]
```

## Success and Failure Response

If handshake from any node is un-successful then you will see output similar to the following:

```shell
$ export RUST_LOG=info
$ cargo run eth enode://2d941dcc0f17769e1d93acaa9412eef5a03411b26b72561d0debd6cf11948019cf52fd71334a67c7c9830beef030e5644e6fa5ef97275404e5f3ce0823b7dfcc@136.243.102.80:30303 enode://37e79d9bdb017a0dbbc1f4851f280881f1fba6110c078144279b879d7235c0b80ccc84fd8cb1080e7897fbf0f322cdeeaa545de65116f84b46ca1a97203720d9@167.235.12.71:15701 enode://233f296f0de5f652fd6435b51e43846250c6cea4480f233cafd735ee5d94ae540be133760b767277e8a226eaaa80cca1ae5d41f09c3dd3bc4e3e0ea1eb5bb4d6@15.204.197.100:30403 enode://3fa4486f8d27f1a351b832f0e172691b69ddb19ff216f15052dff1e6af5994ae8789c723227788cf72263998dc42d6dd8df7b7893479ae7b5741c352bc6f6b20@76.17.9.80:30404 enode://d160ac836263e521f50589ff27caf7cfeeef6e0b977987b9fbfe4c551454b84972f3a655ab30d3f97c7ca2e6e3212339888443710f1254df5ad93d49d6ef917f@68.201.65.25:30303
--- snip ----
2024-04-01T10:56:10.695543Z  INFO p2p_handshake_eth::p2p::handshake: [successful] [136.243.102.80] 
2024-04-01T10:56:10.695576Z  INFO p2p_handshake_eth::p2p::handshake: [successful] [167.235.12.71] 
2024-04-01T10:56:10.825054Z ERROR p2p_handshake_eth::p2p::handshake: P2P handshake error: [failed] [15.204.197.100] error: ECIES error make a retry again: stream closed due to not being readable
2024-04-01T10:56:10.845692Z  INFO p2p_handshake_eth::p2p::handshake: [successful] [76.17.9.80] 
2024-04-01T10:56:10.845729Z  INFO p2p_handshake_eth::p2p::handshake: [successful] [68.201.65.25]
```

### How to run the integration test

We have integration tests which located in [tests](tests) folder.
However, since the results returned by a P2P handshake are not always successful, it is not possible to use `assert!()` although you will see the output.


```shell
cargo test --test handshake_test
```

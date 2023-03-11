//
// node_rpc_connection_credentials
//

pub mod nrcc {
    use crate::constants::constants::{
        NODE_RPC_URL,
        RPC_CLIENT_USERNAME,
        RPC_CLIENT_PASSWORD,
    };

    use crate::my_errors::NodeRpcConnectionError;

    #[derive(Debug)]
    pub struct NodeRpcConnectionCredentials {
        pub url: String,
        pub username: String,
        pub password: String,
    }

    impl Default for NodeRpcConnectionCredentials {
        fn default() -> Self {
            NodeRpcConnectionCredentials {
                url: NODE_RPC_URL.to_string(),
                username: RPC_CLIENT_USERNAME.to_string(),
                password: RPC_CLIENT_PASSWORD.to_string(),
            }
        }
    }

    impl NodeRpcConnectionCredentials {
        pub fn new() -> Self {
            Default::default()
        }
    }
}
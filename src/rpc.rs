//
// node_rpc_connection_credentials
//

pub mod rpc {
    
    use crate::constants::constants::{
        NODE_RPC_URL,
        RPC_CLIENT_USERNAME,
        RPC_CLIENT_PASSWORD,
        ERROR_RPC_AUTHENTICATION,
    };

    #[derive(Debug)]
    pub struct RpcAuthentication {
        pub url: &'static str,
        pub username: &'static str,
        pub password: &'static str,
        pub error: &'static str,
    }

    impl Default for RpcAuthentication {
        fn default() -> Self {
            RpcAuthentication {
                url: NODE_RPC_URL,
                username: RPC_CLIENT_USERNAME,
                password: RPC_CLIENT_PASSWORD,
                error: ERROR_RPC_AUTHENTICATION,
            }
        }
    }

    impl RpcAuthentication {
        pub fn new() -> Self {
            Default::default()
        }
    }
}
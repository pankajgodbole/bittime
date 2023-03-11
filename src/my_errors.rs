//
// my_errors.rs
//


use thiserror::Error;

use crate::constants::constants::{
    ERROR_RPC_BITCOIN_NODE_AUTHENTICATION,
};

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum NodeRpcConnectionError {
    #[error ("ERROR! could not authenticate with the Bitcoin node. Exiting ...")]
    ConnectionNotEstablished,
}


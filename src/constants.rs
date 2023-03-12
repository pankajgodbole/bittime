//
// constants.rs
//

pub mod constants {
    // Bitcoin Core RPC connection details
    pub const NODE_RPC_URL: &str = "http://localhost:8332";
    pub const RPC_CLIENT_USERNAME: &str = "t580";
    pub const RPC_CLIENT_PASSWORD: &str = "g7-oP?3USrjv-cyEz3^z%wEvTXv23i";

    // Bitcoin RPC errors
    pub const ERROR_RPC_AUTHENTICATION: &str = "ERROR! could not authenticate with the Bitcoin node. Exiting ...";
    pub const ERROR_RPC_BLOCKCOUNT_RETRIEVAL: &str = "ERROR! could not get the block count";
    pub const ERROR_RPC_BLOCKCHAIN_INFO_RETRIEVAL: &str = "ERROR! could not get the blockchain info";
    pub const ERROR_RPC_BLOCKHASH_RETRIEVAL: &str = "ERROR! could not get the block hash";
    pub const ERROR_RPC_BLOCKHEADER_INFO_RETRIEVAL: &str = "ERROR! could not get the block header by hash";

    // Redis details
    pub const DB_URL: &str = "redis://127.0.0.1/";
    pub const REDIS_COMMAND_SET: &str = "SET";
    pub const REDIS_COMMAND_GET: &str = "GET";
    pub const REDIS_COMMAND_HSET: &str = "HSET";

    // Database key names
    pub const DB_KEY_BLOCKCHAIN_NUM_HEADERS: &str = "blockchain_number_of_headers";
    pub const DB_KEY_LAST_NODE_VERIFIED_HEIGHT: &str = "last_node_verified_height";
    pub const DB_KEY_NEXT_TO_PROCESS_HEIGHT: &str = "next_to_process_height";

    // Database errors
    pub const ERROR_DB_SET: &str = "ERROR! could not store the value for the given key";
    pub const ERROR_DB_GET: &str = "ERROR! could not get the value for the given key";
    pub const ERROR_DB_HSET: &str = "ERROR! could not store the hash set value for the given key";
    pub const ERROR_DB_CONNECTION: &str = "ERROR! could not connect with the database";
    pub const ERROR_DB_STORAGE_VALUE: &str = "ERROR! could not store the value";
    pub const ERROR_DB_STORAGE_HEIGHT: &str = "ERROR! could not store the block height";
    pub const ERROR_DB_STORAGE_LAST_VERIFIED_HEIGHT: &str = "ERROR! could not store the number of headers in the chain";
    pub const ERROR_DB_STORAGE_HEIGHT_TO_VERIFY_NEXT :&str = "ERROR! could not store the height of the block to verify next";
    //pub const ERROR_DB_STORAGE_NUM_HEADERS: &str = "ERROR! could not store the number of headers in the chain";

    // ZMQ connection details for blocks and transactions
    pub const ZMQ_URI_PUBRAWBLOCK: &str = "tcp://127.0.0.1:28332";
    //pub const ZMQ_URI_PUBRAWTX     :&str = "tcp://127.0.0.1:28332";
    //pub const ZMQ_URI_PUBHASHBLOCK :&str = "tcp://127.0.0.1:28332";
    //pub const ZMQ_URI_PUBHASHTX    :&str = "tcp://127.0.0.1:28332";

    // ZMQ topics for blocks and transactions
    pub const ZMQ_TOPIC_PUBRAWBLOCK: &str = "rawblock";
    //pub const ZMQ_TOPIC_PUBRAWTX     :&str = "rawtx";
    //pub const ZMQ_TOPIC_PUBHASHBLOCK :&str = "hashblock";
    //pub const ZMQ_TOPIC_PUBHASHTX    :&str = "hashtx";

    // BTreeMap key names
    pub const MAP_KEY_HASH: &str = "hash";
    pub const MAP_KEY_TIME_AS_SECONDS: &str = "time_as_seconds";
    pub const MAP_KEY_TIME_AS_UTC: &str = "time_as_utc";
    // pub const MAP_KEY_NUM_CONFIRMATIONS         :&str = "number_of_confirmations";
    // pub const MAP_KEY_SIZE                      :&str = "size";
    // pub const MAP_KEY_WEIGHT                    :&str = "weight";
    // pub const MAP_KEY_VERSION                   :&str = "protocol_version";
    // pub const MAP_KEY_MERKLE_ROOT               :&str = "merkle_root";
    // pub const MAP_KEY_DIFFICULTY                :&str = "difficulty";
    // pub const MAP_KEY_NUM_TRANSACTIONS          :&str = "number_of_transactions";
    // pub const MAP_KEY_TRANSACTIONS              :&str = "transactions";
    // pub const MAP_KEY_PREV_BLOCK_HASH           :&str = "hash_of_previous_block";
    // pub const MAP_KEY_NEXT_BLOCK_HASH           :&str = "hash_of_next_block";
}
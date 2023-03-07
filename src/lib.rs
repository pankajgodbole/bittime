
// For concurrent and/or parallel processing
//use tokio::task::{spawn, JoinHandle};

use redis::{cmd, Connection, RedisResult};

//
// To interact with the Bitcoin Core node via its RPC interface
//
use bitcoincore_rpc::{
    Client,
    RpcApi,
    bitcoin::BlockHash,
    json::{GetBlockHeaderResult, GetBlockchainInfoResult},
};

// To get notified of new blocks
use async_zmq::{subscribe, Result};

// To convert from/to the UNIX epoch
use chrono::{TimeZone, Utc};

use std::{collections::BTreeMap};

// Redis details
const REDIS_URL: &str = "redis://127.0.0.1/";
const REDIS_COMMAND_SET: &str = "SET";
const REDIS_COMMAND_GET: &str = "GET";
const REDIS_COMMAND_HSET: &str = "HSET";

//
// BTreeMap key names
//
const MAP_KEY_HASH: &str = "hash";
const MAP_KEY_TIME_AS_SECONDS: &str = "time_as_seconds";
const MAP_KEY_TIME_AS_UTC: &str = "time_as_utc";
/*
const MAP_KEY_NUM_CONFIRMATIONS         :&str = "number_of_confirmations";
const MAP_KEY_SIZE                      :&str = "size";
const MAP_KEY_WEIGHT                    :&str = "weight";
const MAP_KEY_VERSION                   :&str = "protocol_version";
const MAP_KEY_MERKLE_ROOT               :&str = "merkle_root";
const MAP_KEY_DIFFICULTY                :&str = "difficulty";
const MAP_KEY_NUM_TRANSACTIONS          :&str = "number_of_transactions";
const MAP_KEY_TRANSACTIONS              :&str = "transactions";
const MAP_KEY_PREV_BLOCK_HASH           :&str = "hash_of_previous_block";
const MAP_KEY_NEXT_BLOCK_HASH           :&str = "hash_of_next_block";
*/

//
// panic! error messages
//
pub const ERROR_RPC_BLOCKCOUNT_RETRIEVAL: &str = "ERROR! could not get the block count";
pub const ERROR_RPC_BLOCKCHAIN_INFO_RETRIEVAL: &str = "ERROR! could not get the blockchain info";
const ERROR_RPC_BLOCKHASH_RETRIEVAL: &str = "ERROR! could not get the block hash";
const ERROR_RPC_BLOCKHEADER_INFO_RETRIEVAL: &str = "ERROR! could not get the block header by hash";

const ERROR_DB_SET: &str = "ERROR! could not store the value for the given key";
const ERROR_DB_GET: &str = "ERROR! could not get the value for the given key";
const ERROR_DB_HSET: &str = "ERROR! could not store the hash set value for the given key";
//const ERROR_DB_STORAGE_NUM_HEADERS: &str = "ERROR! could not store the number of headers in the chain";

///
/// Fetches the height of the current tip of the longest chain
///
pub fn get_longest_chain_height(rpc_client: &Client, db_conn: &mut Connection) -> Result<u64> {
    let bc_info: GetBlockchainInfoResult =
        rpc_client
            .get_blockchain_info()
            .expect(ERROR_RPC_BLOCKCHAIN_INFO_RETRIEVAL);
    Ok(bc_info.headers) }

///
/// Fetches the height of the last block verified by our node
///
pub fn get_last_verified_block_height(rpc_client: &Client, db_conn: &mut Connection) -> Result<u64> {
    let hv: u64 =
        rpc_client
            .get_block_count()
            .expect(ERROR_RPC_BLOCKCOUNT_RETRIEVAL);
    Ok(hv) }


///
/// Saves the supplied block height to the database
///
pub fn store_height(
    db_connection :&mut Connection,
    key :&str,
    val :u64,
    err_msg :&str
) -> RedisResult<()> {
    cmd(REDIS_COMMAND_SET)
        .arg(key)
        .arg(val)
        .query::<()>(db_connection)
        .expect(err_msg);
    Ok(()) }

///
/// Fetches the requested block height value from the database and returns it
///
pub fn fetch_height(c: &mut Connection, k: &str) -> u64 {
    match cmd(REDIS_COMMAND_GET).arg(k).query(c).expect(ERROR_DB_GET) {
        Some(k) => k,
        _ => 0,
    }
}

///
/// Writes the given block hash to Redis
///
pub fn store_info(
    c: &mut redis::Connection,
    k: String,
    i: BTreeMap<String, String>,
) -> RedisResult<()> {
    cmd(REDIS_COMMAND_HSET)
        .arg(k)
        .arg(i)
        .query::<()>(c)
        .expect(ERROR_DB_HSET);
    Ok(())
}

///
/// Returns the value associated with a given key
///
/*fn fetch_value (
    c :&mut redis::Connection
  , k :&str)
-> RedisResult<>*/

///
/// Returns a data structure populated with the desired details of a block
///
pub fn get_info(c: &mut Client, h: u64) -> BTreeMap<String, String> {
    let hash: BlockHash = c.get_block_hash(h).expect(ERROR_RPC_BLOCKHASH_RETRIEVAL);

    // Some block header fields
    //let fields :BlockHeader = c.get_block_header(&hash).expect("ERROR! could not get the block header by hash");

    // All block header fields (no transactions)
    let fields: GetBlockHeaderResult = c
        .get_block_header_info(&hash)
        .expect(ERROR_RPC_BLOCKHEADER_INFO_RETRIEVAL);

    // All block fields including all transaction hashes
    //let fields :GetBlockResult =  c.get_block_info(&hash).expect("ERROR! could not fetch block by hash");

    let time_as_seconds: String = fields.time.to_string();
    let time_as_utc: String = Utc
        .timestamp_opt(time_as_seconds.parse::<i64>().unwrap(), 0)
        .unwrap()
        .to_string();

    BTreeMap::from([
        (String::from(MAP_KEY_TIME_AS_SECONDS), time_as_seconds),
        (String::from(MAP_KEY_TIME_AS_UTC), time_as_utc),
        (String::from(MAP_KEY_HASH), hash.to_string()),
    ])
}

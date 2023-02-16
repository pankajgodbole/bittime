//
// main.rs
//

// Coding conventions
#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![deny(dead_code)]
#![deny(unused_imports)]
//#![deny(missing_docs)]
#![deny(unused_must_use)]


#![allow(unused)]

//TODO Use lifetime annotations

use redis::{Connection};

// To get notified of new blocks
use async_zmq::{
    Result,
    subscribe,
    Multipart,
    Subscribe,
    StreamExt
};

// To interact with the Bitcoin Core node via its RPC interface
use bitcoincore_rpc::{
    Auth,
    Client,
    RpcApi,
};

// For logging
use log::{
    error,
    warn,
    info,
    debug,
    trace,
};

use bittime::{
    ERROR_RPC_BLOCKCOUNT_RETRIEVAL,
    ERROR_RPC_BLOCKCHAIN_INFO_RETRIEVAL,
    get_heights,
    get_info,
    store_height,
    store_info,
    fetch_height
};

use std::{
    collections::BTreeMap,
    thread::sleep,
    time::Duration
};


// Bitcoin Core RPC connection details
const NODE_URL: &str = "http://localhost:8332";
const USERNAME: &str = "t580";
const PASSWORD: &str = "g7-oP?3USrjv-cyEz3^z%wEvTXv23i";

// Redis details
const REDIS_URL: &str = "redis://127.0.0.1/";

// Bitcoin RPC errors
const ERROR_RPC_BITCOIN_NODE_AUTHENTICATION: &str =
    "ERROR! could not authenticate with the Bitcoin node. Exiting ...";

// Database errors
const ERROR_DB_CONNECTION: &str = "ERROR! could not connect with the database";
const ERROR_DB_VALUE_STORAGE: &str = "ERROR! could not store the value";

// Database key names
const DB_KEY_NEXT_TO_PROCESS_HEIGHT: &str = "next_to_process_height";

// ZMQ connection details for blocks and transactions
//const ZMQ_URI_PUBHASHBLOCK :&str = "tcp://127.0.0.1:28332";
const ZMQ_URI_PUBRAWBLOCK: &str = "tcp://127.0.0.1:28332";
//const ZMQ_URI_PUBHASHTX    :&str = "tcp://127.0.0.1:28332";
//const ZMQ_URI_PUBRAWTX     :&str = "tcp://127.0.0.1:28332";

// ZMQ topics for blocks and transactions
//const ZMQ_TOPIC_PUBHASHBLOCK :&str = "hashblock";
const ZMQ_TOPIC_PUBRAWBLOCK: &str = "rawblock";
//const ZMQ_TOPIC_PUBHASHTX    :&str = "hashtx";
//const ZMQ_TOPIC_PUBRAWTX     :&str = "rawtx";

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    trace!("main");

    // Authenticate with a local Bitcoin node
    let mut rpc_client: Client =
        Client::new(
            NODE_URL,
            Auth::UserPass(USERNAME.to_string(), PASSWORD.to_string()),
        )
    //.unwrap()
    .expect(ERROR_RPC_BITCOIN_NODE_AUTHENTICATION);

    // Connect to the database
    let db_client: redis::Client = redis::Client::open(REDIS_URL).expect(ERROR_DB_CONNECTION);
    let mut db_conn: Connection = db_client.get_connection().unwrap();

    loop {
        trace!(":main :loop");

        let mut hc: u64 = 0;
        let mut hv: u64 = 0;

        let heights: (u64, u64) = get_heights(&rpc_client, &mut db_conn)?;
        match heights {
            (_, _) => {
                hc = heights.0;
                hv = heights.1;
            }
            _ => {
                error!(":main :loop :{}", ERROR_RPC_BLOCKCHAIN_INFO_RETRIEVAL);
                continue;
            }
        }

        println!("main: hc: {}, hv: {}", hc, hv);

        if hc == hv {
            //
            // Process blocks beginning with the genesis block
            //

            // Get the height of the next block we have to process
            let mut hn: u64 = 0;
            let hn_: Option<u64> = Some(fetch_height(&mut db_conn, DB_KEY_NEXT_TO_PROCESS_HEIGHT));
            if hn_.is_some() {
                hn = hn_.unwrap();
            }

            println!("main: hn: {}", hn);

            if hn == (hv + 1) {
                // The app has caught up with the node (which in turn has caught up with the network)
                // Now listen for notifications for new blocks, as they are verified by the node

                //let zmq_context :Context = Context::new();
                let mut sub: Subscribe = subscribe(ZMQ_URI_PUBRAWBLOCK)?
                    //.expect("ERROR :could not build a socket")
                    .connect()?;
                //.expect("ERROR :could not connect to the ZMQ socket");

                sub.set_subscribe(ZMQ_TOPIC_PUBRAWBLOCK)?;
                //.expect("ERROR :could not set the topic to which to subscribe");

                //
                // Listen for ZeroMQ notifications of a new block being verified, then
                // fetch its details
                //
                while let Some(msg) = sub.next().await {
                    let _zmq_new_block_event: Multipart = msg?;
                    //println!("main: zmq_new_block_event :{:?}", zmq_new_block_event);

                    // Find the current tip of the chain
                    let hv: u64 = rpc_client
                        .get_block_count()
                        .expect(ERROR_RPC_BLOCKCOUNT_RETRIEVAL);
                    println!("main: hn == (hv+1): hv: {}", hv);

                    // The next block to process is the current chain tip
                    let hn: u64 = hv;
                    let info_map: BTreeMap<String, String> = get_info(&mut rpc_client, hn);
                    //println!("{}: info_map:{:?}", ht, info_map);
                    println!(
                        "{}: time_as_seconds: {}, time_as_utc: {}, hash:{}",
                        hn, info_map["time_as_seconds"], info_map["time_as_utc"], info_map["hash"]
                    );
                    store_info(&mut db_conn, hn.to_string(), info_map)
                        .expect(ERROR_DB_VALUE_STORAGE);
                }
            } else {
                while hn < (hv + 1) {
                    let ht: u64 = hn; // Temporarily save the next height to process
                    let info_map: BTreeMap<String, String> = get_info(&mut rpc_client, ht);
                    //println!("{}: info_map:{:?}", ht, info_map);
                    println!(
                        "{}: time_as_seconds: {}, time_as_utc: {}, hash:{}",
                        ht, info_map["time_as_seconds"], info_map["time_as_utc"], info_map["hash"]
                    );
                    store_info(&mut db_conn, hn.to_string(), info_map)
                        .expect(ERROR_DB_VALUE_STORAGE);

                    hn += 1;
                    store_height(&mut db_conn, DB_KEY_NEXT_TO_PROCESS_HEIGHT, hn).unwrap();
                }
            }
        }
        // Wait for the node to synchronize with the network
        else {
            println!(
                "main: (if hv < hc): Waiting for the node to catch up with the Bitcoin network ..."
            );
            sleep(Duration::from_secs(60));
            continue;
        }
    }

    Ok(())
}

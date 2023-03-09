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

use std::{
    collections::BTreeMap,
    thread::sleep,
    time::Duration
};

use bittime::{
    get_longest_chain_height,
    get_last_verified_block_height,
    get_info,
    store_height,
    store_info,
    fetch_height
};

mod constants;
use constants::constants::{
    NODE_URL,
    USERNAME,
    PASSWORD,
    ERROR_RPC_BITCOIN_NODE_AUTHENTICATION,
    REDIS_URL,
    ERROR_DB_CONNECTION,
    DB_KEY_BLOCKCHAIN_NUM_HEADERS,
    DB_KEY_LAST_NODE_VERIFIED_HEIGHT,
    DB_KEY_NEXT_TO_PROCESS_HEIGHT,
    ERROR_DB_STORAGE_HEIGHT,
    ERROR_DB_STORAGE_VALUE,
    ERROR_DB_STORAGE_LAST_VERIFIED_HEIGHT,
    ERROR_DB_STORAGE_HEIGHT_TO_VERIFY_NEXT,
    ERROR_RPC_BLOCKCOUNT_RETRIEVAL,
    ZMQ_URI_PUBRAWBLOCK,
    ZMQ_TOPIC_PUBRAWBLOCK,
};

// Bitcoin Core RPC connection details



#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!(":main");

    // Authenticate with a local Bitcoin node
    let mut rpc_client: Client =
        Client::new(
            NODE_URL,
            Auth::UserPass(USERNAME.to_string(), PASSWORD.to_string()),
        )
        .expect(ERROR_RPC_BITCOIN_NODE_AUTHENTICATION);

    // Connect to the database
    let db_client: redis::Client = redis::Client::open(REDIS_URL).expect(ERROR_DB_CONNECTION);
    let mut db_conn: Connection = db_client.get_connection().unwrap();

    //
    // Process blocks beginning with the genesis block
    //
    loop {
        trace!(":main :loop");

        let mut hc: u64 = 0;
        let mut hv: u64 = 0;

        hc = get_longest_chain_height(&rpc_client, &mut db_conn)?;
        hv = get_last_verified_block_height(&rpc_client ,&mut db_conn)?;
        println!("main: hc: {}, hv: {}", hc, hv);

        // If there was an error while retrieving the heights, then try again
        if hc == 0 || hv == 0 {
            continue;
        }

        // Save these heights for later use
        store_height(&mut db_conn, DB_KEY_BLOCKCHAIN_NUM_HEADERS, hc, ERROR_DB_STORAGE_HEIGHT);
        store_height(&mut db_conn, DB_KEY_LAST_NODE_VERIFIED_HEIGHT, hv, ERROR_DB_STORAGE_LAST_VERIFIED_HEIGHT);

        if hc == hv {

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
                let mut sub: Subscribe =
                    subscribe(ZMQ_URI_PUBRAWBLOCK)?
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
                    let hv: u64 =
                        rpc_client
                        .get_block_count()
                        .expect(ERROR_RPC_BLOCKCOUNT_RETRIEVAL);
                    println!("main: hn == (hv+1): hv: {}", hv);

                    // The next block to process is the current chain tip
                    let hn: u64 = hv;
                    let info_map: BTreeMap<String, String> = get_info(&mut rpc_client, hn);
                    //println!("{}: info_map:{:?}", ht, info_map);
                    println!(
                        "{}: time_as_seconds: {}, time_as_utc: {}, hash:{}",
                        hn,
                        info_map["time_as_seconds"],
                        info_map["time_as_utc"],
                        info_map["hash"]
                    );
                    store_info(&mut db_conn, hn.to_string(), info_map).expect(ERROR_DB_STORAGE_VALUE);
                }
            } else {
                while hn < (hv + 1) {
                    let ht: u64 = hn; // Temporarily save the next height to process
                    let info_map: BTreeMap<String, String> = get_info(&mut rpc_client, ht);
                    //println!("{}: info_map:{:?}", ht, info_map);
                    println!(
                        "{}: time_as_seconds: {}, time_as_utc: {}, hash:{}",
                        ht,
                        info_map["time_as_seconds"],
                        info_map["time_as_utc"],
                        info_map["hash"]
                    );
                    store_info(&mut db_conn, hn.to_string(), info_map).expect(ERROR_DB_STORAGE_VALUE);

                    hn += 1;
                    store_height(&mut db_conn, DB_KEY_NEXT_TO_PROCESS_HEIGHT, hn, ERROR_DB_STORAGE_HEIGHT_TO_VERIFY_NEXT).unwrap();
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

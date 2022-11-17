//
// main.rs
//


extern crate bitcoincore_rpc;

use std::{
      env::args
    , process::{exit , Command , Output , Child , Stdio}
    , string::String};

use bitcoincore_rpc::{Auth , Client , RpcApi};
use bitcoincore_rpc::bitcoin::BlockHash;

pub const CMD :&str = "bitcoin-cli";
pub const ARG_GETBLOCK :&str = "getblock";
pub const ARG_GETBLOCKHASH :&str = "getblockhash";

fn main() {
    println!(" ");

    //
    // Handle the command-line arguments
    //
    let mut all_args :Vec<String> = args().skip(1).collect();
    if all_args.is_empty() {
        print_usage_and_exit(); }

    //
    // Read the details of all blocks up to the current latest block
    //

    let block_height :String = all_args.remove(0);
    //let block_hash :String = all_args.remove(0);
    //let cmd_getblockhash :String = all_args.remove(0);

    //
    // Given a block height obtain the block's hash
    //
   let mut cmd_getblockhash :Command = Command::new(CMD);
    cmd_getblockhash
        .arg(ARG_GETBLOCKHASH)
        .arg(block_height);
    let op_cmd_getblockhash : Output =
        cmd_getblockhash
            .output()
            .expect("failed to execute");

    let blockhash_trailing_newline :String = String::from_utf8_lossy(&op_cmd_getblockhash.stdout).to_string();

    // We need to remove the trailing newline from the block hash
    let block_hash :String = blockhash_trailing_newline.replace("\n" ,"");
    println!("main: block_hash: {block_hash}");

    //
    // Execute a command and capture its output for further processing
    // (Ref: https://stackoverflow.com/questions/63807700/how-to-execute-a-command-in-a-shell-running-as-a-child-process)
    //
    let mut cmd_getblock :Command = Command::new(CMD);
    let cmd_getblock_output :Output =
        cmd_getblock
            .arg(ARG_GETBLOCK)
            .arg(block_hash)
            .stdout(Stdio::piped())
            .output()
            .expect("failed to execute");
    let str_getblock_output :String = String::from_utf8_lossy(&cmd_getblock_output.stdout).to_string();
    println!("main: str_getblock_output: {}" , str_getblock_output);

    println!(" ");

/*
    let rpc =
        Client::new("http://localhost:8332"
                     , Auth::UserPass(  "t580".to_string()
                                            ,"g7-oP?3USrjv-cyEz3^z%wEvTXv23i".to_string())).unwrap();
    let best_block_hash :BlockHash = rpc.get_best_block_hash().unwrap();
    println!("main: best_block_hash:{}" , best_block_hash);


 */

}

fn print_usage_and_exit()
{
    println!("USAGE:");
    println!("cargo run <BLOCK_HEIGHT>");

    // **OPTION**
    // Print useful information about what subcommands and arguments you can use
    // println!("...");

    exit(-1);
}

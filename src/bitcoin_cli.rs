use bitcoincore_rpc_json::{GetBlockchainInfoResult, GetBlockResult, GetDescriptorInfoResult, GetRawTransactionResult, ScanTxOutResult};
use bitcoincore_rpc_json::bitcoin::{BlockHash, Txid};
use ssh2::Session;
use crate::ssh::execute_command;

pub(crate) fn get_blockchain_info(sess: &Session) -> GetBlockchainInfoResult {
    let get_blockchain_info_str = execute_command(sess,"bitcoin-cli getblockchaininfo");
    serde_json::from_str(&get_blockchain_info_str).unwrap()
}
pub(crate) fn get_block_hash(sess: &Session, block_height: u64) -> String {
    let block_hash_command = format!("bitcoin-cli getblockhash {block_height}");
    execute_command(&sess, &block_hash_command)
}

pub(crate) fn get_block(sess: &Session, block_hash: &str) -> GetBlockResult {
    let get_block_command = format!("bitcoin-cli getblock {block_hash}");
    let block_str = execute_command(&sess, &get_block_command);
    serde_json::from_str(&block_str).unwrap()
}

pub(crate) fn get_raw_transaction(sess: &Session, txid: &Txid, block_hash: &BlockHash) -> GetRawTransactionResult {
    let get_raw_transaction_command = format!("bitcoin-cli getrawtransaction {txid} true {block_hash}");
    let tx_str = execute_command(&sess, &get_raw_transaction_command);
    serde_json::from_str(&tx_str).unwrap()
}

pub(crate) fn _getdescriptorinfo(sess: &Session, descriptor: &str) -> GetDescriptorInfoResult {
    let command = format!("bitcoin-cli getdescriptorinfo \"{descriptor}\"");
    let res = execute_command(&sess, &command);
    serde_json::from_str(&res).unwrap()
}

pub(crate) fn scantxoutset(sess: &Session, descriptor: &str) -> ScanTxOutResult {
    let command = format!("bitcoin-cli scantxoutset start '[\"{descriptor}\"]'");
    println!("command: {command}");
    let res = execute_command(&sess, &command);
    println!("res: {res}");
    serde_json::from_str(&res).unwrap()
}
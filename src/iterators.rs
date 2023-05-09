use bitcoincore_rpc_json::{GetBlockResult, GetRawTransactionResult};
use ssh2::Session;
use crate::bitcoin_cli::{get_block, get_block_hash, get_raw_transaction};

pub(crate) fn iterate_through_blocks<F, T>(start: u64, end: u64, sess: &Session, f: F) -> Vec<T>
    where F: Fn(&GetRawTransactionResult) -> T {
    let mut all_blocks_vector: Vec<T> = Vec::new();
    for b in start..end {
        println!("getting block hash for block {b}");
        let block_hash = get_block_hash(sess, b);

        let block: GetBlockResult = get_block(sess, &block_hash);
        let block_vector = iterate_through_txs(sess, block, &f);
        all_blocks_vector.extend(block_vector);
    }
    return all_blocks_vector
}

pub(crate) fn iterate_through_txs<F, T>(sess: &Session, block: GetBlockResult, f: F) -> Vec<T>
    where F: Fn(&GetRawTransactionResult) -> T {
    let mut i = 0;
    let block_hash = block.hash;
    let mut tx_outputs = Vec::new();
    for txid in block.tx {
        let tx: GetRawTransactionResult = get_raw_transaction(sess, &txid, &block_hash);
        let f_output = f(&tx);
        tx_outputs.push(f_output);
        println!("i: {}, txid: {}", i, &tx.txid);
        i = i + 1;
    }
    tx_outputs
}
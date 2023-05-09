use std::collections::HashMap;
use bitcoincore_rpc_json::{GetBlockchainInfoResult, GetRawTransactionResult, GetRawTransactionResultVout};
use crate::bitcoin_cli::get_blockchain_info;
use crate::iterators::iterate_through_blocks;
use crate::ssh::get_session;

pub(crate) fn get_script_type(tx: &GetRawTransactionResult) -> Vec<GetRawTransactionResultVout> {
    tx.vout.clone()
}

pub(crate) fn locking_script() -> () {
    println!("starting locking script");
    let sess = get_session();

    let get_blockchain_info_result : GetBlockchainInfoResult = get_blockchain_info(&sess);
    let block_height = get_blockchain_info_result.blocks;
    let starting_block = block_height - 5;
    println!("blockheight: {}", block_height);

    // iterate through each transaction
    let output = iterate_through_blocks(starting_block, block_height, &sess, get_script_type);
    let vouts : Vec<GetRawTransactionResultVout>= output.into_iter().flatten().collect();
    let mut map = HashMap::new();
    for vout in vouts {
        let script_type = vout.script_pub_key.type_.unwrap();
        println!("vout type: {script_type:?}");
        let script_str = format!("{script_type:?}");
        let count = map.entry(script_str).or_insert(0);
        *count += 1;
    }

    let mut vec: Vec<_> = map.iter().collect();
    vec.sort_by(|a, b| b.1.cmp(a.1));

    let total: i32 = vec.clone().into_iter().map(|(_, b)| b).sum();
    println!("starting block: {}, ending block: {}", starting_block, block_height);
    println!("total transactions {:?}", total);
    println!("map {:?}", vec);
}
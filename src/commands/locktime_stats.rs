use std::collections::HashMap;
use bitcoincore_rpc_json::{ GetRawTransactionResult};
use crate::bitcoin_cli::get_blockchain_info;
use crate::iterators::{iterate_through_blocks};
use crate::ssh::{get_session};

pub(crate) fn get_locktime(tx: &GetRawTransactionResult) -> u32 {
    tx.locktime
}

pub(crate) fn locktime_stats() {
    let sess = get_session();

    let get_blockchain_info = get_blockchain_info(&sess);

    let block_height = get_blockchain_info.blocks;
    let starting_block = block_height - 1;

    let locktimes = iterate_through_blocks(starting_block, block_height, &sess, get_locktime);
    let mut locktime_map = HashMap::new();
    for l in locktimes {
        let count = locktime_map.entry(l).or_insert(0);
        *count += 1;
    }
    let mut locktime_vec: Vec<_> = locktime_map.iter().collect();
    locktime_vec.sort_by(|a, b| b.1.cmp(a.1));

    let total: i32 = locktime_vec.clone().into_iter().map(|(_, b)| b).sum();
    println!("starting block: {}, ending block: {}", starting_block, block_height);
    println!("total transactions {:?}", total);
    println!("map {:?}", locktime_vec);
}
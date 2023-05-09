use bitcoincore_rpc_json::{GetBlockchainInfoResult, GetRawTransactionResult, GetRawTransactionResultVin};
use crate::bitcoin_cli::get_blockchain_info;
use crate::iterators::iterate_through_blocks;
use crate::ssh::get_session;

pub(crate) fn get_script_type_vin(tx: &GetRawTransactionResult) -> Vec<GetRawTransactionResultVin> {
    // for vout in &tx.vout {
    //     println!("vout: {vout:?}");
    //     vout.n
    // }
    tx.vin.clone()
}

pub(crate) fn unlocking_script() -> () {
    println!("starting unlocking_script script");
    let sess = get_session();

    let get_blockchain_info_result : GetBlockchainInfoResult = get_blockchain_info(&sess);
    let block_height = get_blockchain_info_result.blocks;
    let starting_block = block_height - 1;
    println!("blockheight: {}", block_height);

    // iterate through each transaction
    let output = iterate_through_blocks(starting_block, block_height, &sess, get_script_type_vin);
    let vins : Vec<GetRawTransactionResultVin>= output.into_iter().flatten().collect();
    for vin in vins {
        let script = match vin.script_sig {
            Some(s) => {
                s.asm
            }
            None => {
                "segwit".to_string()
            }
        };

        println!("vin script type: {script:?}");
    }
}
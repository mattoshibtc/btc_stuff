use std::env;
use crate::bitcoin_cli::{scantxoutset};
use crate::ssh::get_session;

fn append_derivation_path(xpub: String, derivation_path: &str) -> String {
    let mut mutable = xpub.to_owned();
    mutable.push_str(derivation_path);
    mutable
}

fn ypub_to_xpub(ypub: &String) -> String {
    // ypub as in Ypub
    // base58check decode
    let mut decoded = bs58::decode(ypub).with_check(None).into_vec().unwrap();

    // xpub version bytes hex: 0488b21e.
    // change the first 4 bytes to their decimal value in the vector
    decoded[0] = 4; // 04 in hex
    decoded[1] = 136; // 88 in hex
    decoded[2] = 178; // b2 in hex
    decoded[3] = 30; // 1e in hex

    // base58check encode
    bs58::encode(decoded).with_check().into_string()
}
bitcoin-cli
OP_DUP OP_MULTISIG N <derivedpubkey1> <derivedpubkey2> <derivedpubkey3> <derivedpubkey4> <derivedpubkey5> M

pub(crate) fn casa() -> () {
    println!("starting casa");

    let ypubs = vec!(
        env::var("YPUB_1").unwrap().to_string(),
        env::var("YPUB_2").unwrap().to_string(),
        env::var("YPUB_3").unwrap().to_string(),
        env::var("YPUB_4").unwrap().to_string(),
        env::var("YPUB_5").unwrap().to_string()
    );

    // output descriptors use xpubs, so convert Ypub to xpub
    let xpubs: Vec<_> = ypubs.iter().map(|ypub| {
        let xpub = ypub_to_xpub(ypub);
        append_derivation_path(xpub, "/0/*")
    }).collect();

    println!("xpubs: {:?}", xpubs);

    let sess = get_session();

    let descriptor = format!("sh(wsh(sortedmulti(3, {},{},{},{},{})))",
                             xpubs[0], xpubs[1], xpubs[2], xpubs[3], xpubs[4]);

    println!("scanning addresses...");
    let res = scantxoutset(&sess, &descriptor);
    println!("res: {:?}", res);

    // search change addresses too
    let xpubs_change: Vec<_> = ypubs.iter().map(|ypub| {
        let xpub = ypub_to_xpub(ypub);
        append_derivation_path(xpub, "/1/*")
    }).collect();

    let descriptor_change = format!("sh(wsh(sortedmulti(3, {},{},{},{},{})))",
                                    xpubs_change[0], xpubs_change[1], xpubs_change[2], xpubs_change[3], xpubs_change[4]);

    println!("scanning change addresses...");
    let res_change = scantxoutset(&sess, &descriptor_change);
    println!("res: {:?}", res_change);

    let total_btc = res.total_amount + res_change.total_amount;
    println!("total_btc: {total_btc}");

}
use chrono::NaiveDateTime;
use colored::Colorize;
use crate::endian::le_to_be_hex;

// parse raw locktime
pub(crate) fn parse_raw_locktime(tx_locktime: &str) -> () {
    // convert to big endian
    let be = le_to_be_hex(tx_locktime);
    println!("big endian hex:      {}", be.yellow());
    // convert to decimal
    let decimal = i64::from_str_radix(be.as_str(), 16).unwrap();
    println!("decimal:             {}", decimal.to_string().yellow());
    if decimal < 5_000_000 {
        println!("Locked until block:  {}", decimal.to_string().yellow());
    } else {
        let date_time = NaiveDateTime::from_timestamp_opt(decimal as i64, 0).unwrap();
        println!("Locked until:        {}", date_time.to_string().yellow());
    }
}
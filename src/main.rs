mod bitcoin_cli;
mod commands;
mod endian;
mod iterators;
mod ssh;
use colored::*;
use std::env;
use crate::commands::casa::casa;
use crate::commands::locking_script::locking_script;
use crate::commands::locktime_stats::locktime_stats;
use crate::commands::parse_raw_locktime::parse_raw_locktime;
use crate::commands::unlocking_script::unlocking_script;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("{}", "You need to pass an argument to the cargo run".red());
        return
    }

    let program = &args[1];
    match program.as_str() {
        "casa" => {
            // needs env variables
            // export YPUB_1=<key1>
            // export YPUB_2=<key2>
            // export YPUB_3=<key3>
            // export YPUB_4=<key4>
            // export YPUB_5=<key5>
            casa();
        }
        // cargo run parse_raw_locktime <4 byte locktime in raw transaction>
        "parse_raw_locktime" => {
            parse_raw_locktime(&args[2]);
        }
        // cargo run locktime_stats
        "locktime_stats" => {
            locktime_stats();
        }
        // cargo run locking_script
        "locking_script" => {
            locking_script();
        }
        "unlocking_script" => {
            unlocking_script();
        }
        "versionNoStats" => {
            println!("{}", "to be implemented".red())
        }
        _ => println!("{}{}", "Unknown program type: ".red(), program.blue())
    }
}

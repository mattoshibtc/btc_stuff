
// convert little endian hex string to big endian hex string
pub(crate) fn le_to_be_hex(le_hex: &str) -> String {
    let mut bytes = hex::decode(le_hex).unwrap();
    bytes.reverse();
    hex::encode(bytes)
}

// // takes a big endian hex and returns decimal
// pub(crate) fn hex_to_decimal(be_hex: &str) -> u32 {
//     let mut bytes = hex::decode(be_hex).unwrap();
//     i32::from_be_bytes(bytes)
// }


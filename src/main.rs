use std::fs;

use ethabi::Contract;
use hex;

fn main() {
    // input data
    let data = "0xa9059cbb000000000000000000000000d18a52ae66778bf9ece5515115875a313d45f0e900000000000000000000000000000000000000000000000000000000007fde60";

    // remove hex prefix: 0x
    let prefix_removed_data = data.trim_start_matches("0x");

    // get method id, 4 bytes from data
    let method_id = &prefix_removed_data[0..8];
    println!("method_id={}", method_id);

    // load abi
    let contract = Contract::load(fs::read("abi/erc20.abi").unwrap().as_slice()).unwrap();
    // get matched function
    let function = contract.functions().into_iter().find(|f| { hex::encode(f.short_signature()) == method_id }).unwrap();

    // method id removed data
    let method_removed_data = &prefix_removed_data[8..];
    // using selected function decodes input data
    let tokens = function.decode_input(hex::decode(method_removed_data).unwrap().as_slice()).unwrap();

    println!("{:?}", tokens);
}

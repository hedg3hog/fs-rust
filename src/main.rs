use std::fs::read_to_string;
use std::time::Instant;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;


#[derive(Serialize, Deserialize,PartialEq, Clone, )]
struct Bid {
    items: HashSet<u16>,
    value: i32
}

fn load_bids(path: &str) -> Vec<Bid>{
    let f = read_to_string(path).unwrap();
    let bids : Vec<Bid> = serde_json::from_str(&f).unwrap();
    return bids
}

fn main() {
    println!("Hello, world!");
}

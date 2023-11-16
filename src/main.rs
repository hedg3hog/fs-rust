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

fn bid_sum(bids_idx: &[usize], bids : &[Bid]) -> i32 {
    bids_idx.iter().map(|&b| bids[b].value).sum()
}

fn prune_bids(path: &[usize], list_to_check: Vec<usize>, bids: &[Bid]) -> Vec<usize> {
    let sold: HashSet<u16> = path.iter().flat_map(|&b| bids[b].items.iter().cloned()).collect();
    list_to_check.into_iter().filter(|&b| sold.is_disjoint(&bids[b].items)).collect()
}

fn main() {
    println!("Hello, world!");
}


use std::fs::read_to_string;
use std::time::Instant;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;


#[derive(Serialize, Deserialize,PartialEq, Clone, )]
struct Bid {
    items: HashSet<u16>,
    value: u32
}

fn load_bids(path: &str) -> Vec<Bid>{
    let f = read_to_string(path).unwrap();
    let bids : Vec<Bid> = serde_json::from_str(&f).unwrap();
    return bids
}

fn bid_sum(bids : Vec<Bid>) -> u32 {
    bids.iter().map(|b| b.value).sum()
}

fn prune_bids(path: Vec<Bid>, list_to_check: Vec<Bid>) -> Vec<Bid> {
    let sold: HashSet<u16> = path.iter().flat_map(|b| b.items.iter()).cloned().collect();
    list_to_check.into_iter().filter(|b| sold.is_disjoint(&b.items)).collect()
}

fn full_search(bids : Vec<Bid>, best_value: u32, best_path: Vec<Bid>, current_path: Vec<Bid>) -> (Vec<Bid>, u32){
    if bids.len() == 0 {
        return (best_path, best_value)
    }
    let mut best_value = best_value.clone();
    let mut best_path = best_path;
    for b in bids.clone(){
        if bid_sum(bids.clone()) + bid_sum(current_path.clone()) < best_value {
            break;
        }
        let mut c_path = current_path.clone();
        c_path.push(b.clone());
        let x =  bid_sum(c_path.clone()) + b.value;
        if  x > best_value {
            best_value = x;
            best_path = c_path.clone();
        }

    (best_path, best_value) = full_search(prune_bids(c_path.clone(), bids.clone()), best_value, best_path, c_path)

        
}
    

    return (best_path, best_value);

    


}
fn main() {
    let bids_ = load_bids("bids03-ID.json");
    let selection = &bids_[..11];
    let (winner, value) = full_search(selection.to_vec(), 0, vec![], vec![]);
    println!("value: {}", value);
    println!("winner len {}", winner.len());
    println!("winner sum {}", bid_sum(winner.clone()));

    for b in winner{
        println!("{:?}:{}", b.items, b.value);
    }


   
}


use std::fs::read_to_string;
use std::process::exit;
use std::time::Instant;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::{env, usize};

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

fn bid_sum(bids_idx: &[usize], bids : &[Bid]) -> u32 {
    bids_idx.iter().map(|&b| bids[b].value).sum()
}

fn prune_bids(path: &[usize], list_to_check: Vec<usize>, bids: &[Bid]) -> Vec<usize> {
    let sold: HashSet<u16> = path.iter().flat_map(|&b| bids[b].items.iter().cloned()).collect();
    list_to_check.into_iter().filter(|&b| sold.is_disjoint(&bids[b].items)).collect()
}


fn full_search(bids_idx : Vec<usize>, best_value: u32, best_path: Vec<usize>, current_path: Vec<usize>, bids: &Vec<Bid>) -> (Vec<usize>, u32){
    if bids_idx.len() == 0 {
        return (best_path, best_value)
    }
    let mut best_value = best_value.clone();
    let mut best_path = best_path;
    for b in bids_idx.clone(){
        // break if best value can not be over bidden
        if bid_sum(&bids_idx, &bids) + bid_sum(&current_path, &bids) < best_value {
            break;
        }
        let mut c_path = current_path.clone();
        c_path.push(b.clone());

        let x =  bid_sum(&c_path, &bids);
        if  x > best_value {
            best_value = x;
            best_path = c_path.clone();
        }

    (best_path, best_value) = full_search(prune_bids(&c_path, bids_idx.clone(), &bids), best_value, best_path, c_path, bids)

        
}
    

    return (best_path, best_value);

    


}
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", args[1].parse::<usize>().unwrap());
    let n: usize = args[1].parse::<usize>().unwrap();
    let bids_ = load_bids("bids03-ID.json");
    if n > bids_.len() {
        println!("argument bigger than size of bids");
        exit(1)
    }
    let selection = bids_[..n].to_vec();
    println!("Bids to check: {}", selection.len());
    let now = Instant::now();
    let (winner, value) = full_search((0..n).collect(), 0, vec![], vec![], &selection);
    println!("Time: {}", now.elapsed().as_secs_f32());
    println!("value: {}", value);
    println!("winner len {}", winner.len());
    println!("winner sum {}", bid_sum(&winner.clone(), &selection));



   
}


use ps2dq5::encount::{Group, Table, TABLES};
use ps2dq5::monster::Kind;
use std::collections::{HashMap, HashSet};
use std::env;

fn simulate_encount(table: &Table, iter: usize) -> Vec<Vec<Group>> {
    return (0..iter).map(|_| table.encount()).collect();
}

fn encount_distribution(table: &ps2dq5::encount::Table, iter: usize) -> HashMap<Kind, f64> {
    let encounts = simulate_encount(table, iter);

    let mut histogram = HashMap::<Kind, usize>::new();
    for encount in &encounts {
        let mut monster_set = HashSet::new();
        for group in encount {
            monster_set.insert(group.monster());
        }

        for monster in &monster_set {
            let count = histogram.entry(*monster).or_default();
            *count += 1;
        }
    }

    let distribution: HashMap<Kind, f64> = histogram
        .iter()
        .map(|(&k, &v)| (k, (v as f64) / (encounts.len() as f64)))
        .collect();
    return distribution;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let table_index: usize = args[1].parse().unwrap();
    let iter: usize = args[2].parse().unwrap();

    println!("{:#?}", encount_distribution(&TABLES[table_index], iter));
}

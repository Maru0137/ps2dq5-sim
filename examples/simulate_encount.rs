use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let table_index: usize = args[1].parse().unwrap();
    let iter: usize = args[2].parse().unwrap();

    let encounts = ps2dq5::encount::simulate(table_index, iter);
    let monster_dists = ps2dq5::encount::monster_distribution(&encounts);
    //let encount_dists = ps2dq5::encount::encount_distribution(&encounts);

    println!("{:#?}", &monster_dists);
    // println!("{:#?}", &encount_dists);
}

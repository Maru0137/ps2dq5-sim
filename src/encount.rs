use crate::monster;
use csv;
use num_traits;
use rand::distributions::uniform::{SampleBorrow, SampleUniform};
use rand::prelude::*;
use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::*;

pub fn rand_range<T: SampleUniform, B>(upper: B) -> T
where
    B: SampleBorrow<T> + num_traits::Zero,
{
    let mut rng = thread_rng();
    return rng.gen_range(B::zero(), upper);
}

// TODO: must investigate and to enum
#[derive(Clone, Copy, Debug)]
pub struct Preemtive(u32);
// enum Preemtive {
//     Unknown0 = 0,
//     Unknown1 = 1,
//     Unknown2 = 2,
//     Unknown3 = 3,
// }

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
enum Pattern {
    Mixed,
    Single,
    Fixed,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Group {
    monster: monster::Kind,
    num: usize,
}

impl Group {
    pub fn monster(&self) -> monster::Kind {
        self.monster
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Encount {
    groups: Vec<Group>,
}

impl Encount {
    pub fn groups(&self) -> &Vec<Group> {
        &self.groups
    }

    pub fn sort(&mut self) {
        self.groups.sort();
    }
}

#[derive(Clone, Default, Debug)]
pub struct Entry {
    prob: u32,
    monster: monster::Kind,
    num: usize,
}

impl Entry {
    pub fn from_record(record: &csv::StringRecord, entry_index: usize) -> Self {
        const OFFSET: usize = 8;
        const MEMBER_NUM: usize = 3;
        let i = OFFSET + entry_index * MEMBER_NUM;

        let prob: u32 = record[i].parse().unwrap();
        let monster = monster::Kind(record[i + 1].parse::<usize>().unwrap());
        let num: usize = record[i + 2].parse().unwrap();

        return Self { prob, monster, num };
    }
}

#[derive(Clone, Debug)]
pub struct Table {
    index: usize,
    lv: u32,
    preemtive: Preemtive,
    frequency: u32,
    can_scout: bool,
    group_num_probs: [u32; Self::GROUP_MAX - 1],
    entries: [Entry; Self::ENTRY_SIZE],
}

impl Table {
    const MIXED_GROUP_UPPER: usize = 5;
    const SINGLE_GROUP_UPPER: usize = 10;
    const FIXED_GROUP_UPPER: usize = 11;
    const ENTRY_SIZE: usize = Self::FIXED_GROUP_UPPER;
    const GROUP_MAX: usize = 4;

    fn from_record(record: &csv::StringRecord) -> Self {
        let index: usize = record[0].parse().unwrap();
        let lv: u32 = record[1].parse().unwrap();
        let preemtive = Preemtive(record[2].parse::<u32>().unwrap());
        let frequency: u32 = record[3].parse().unwrap();
        let can_scout: bool = record[4].parse::<u32>().unwrap() == 1;
        let mut group_num_probs: [u32; 3] = [0; 3];
        for (i, prob) in group_num_probs.iter_mut().enumerate() {
            *prob = record[5 + i].parse().unwrap();
        }
        let mut entries: [Entry; 11] = Default::default();
        for (i, entry) in entries.iter_mut().enumerate() {
            *entry = Entry::from_record(record, i);
        }
        return Self {
            index,
            lv,
            preemtive,
            frequency,
            can_scout,
            group_num_probs,
            entries,
        };
    }

    fn decide_num_in_group(num: usize, pattern: Pattern) -> usize {
        return match pattern {
            Pattern::Mixed => 1 + rand_range(num),
            Pattern::Single => match num {
                0..=2 => num + 1,
                3..=4 => num - 2 + rand_range(2),
                5 => num - 3 + rand_range(3),
                6 => num - 2 + rand_range(4),
                _ => num + 1,
            },
            Pattern::Fixed => {
                // TODO: must investigate
                num
            }
        };
    }

    fn make_groups(entries: &[&Entry], pattern: Pattern) -> Vec<Group> {
        return entries
            .iter()
            .map(|entry| Group {
                monster: entry.monster,
                num: Self::decide_num_in_group(entry.num, pattern),
            })
            .collect();
    }

    fn rand_to_index(r: u32, probs: &[u32]) -> Option<usize> {
        let mut sum = 0;
        for (i, p) in probs.iter().enumerate() {
            sum += p;
            if r < sum {
                return Some(i);
            }
        }
        return None;
    }

    fn choose_from_probs(probs: &[u32]) -> usize {
        let sum = probs.iter().fold(0, |sum, v| sum + v);
        let r = rand_range(sum);
        return Self::rand_to_index(r, probs).unwrap();
    }

    fn choose_entry(&self, only_mixed: bool) -> (usize, &Entry) {
        let candidates = match only_mixed {
            true => &self.entries[..Self::MIXED_GROUP_UPPER],
            false => &self.entries[..],
        };
        let probs: Vec<_> = candidates.iter().map(|entry| entry.prob).collect();
        let i = Self::choose_from_probs(&probs);
        return (i, &self.entries[i]);
    }

    fn encount_mixed(&self, first_group: &Entry) -> Vec<Group> {
        // Addition groups
        let additional = Self::choose_from_probs(&self.group_num_probs) + 1;
        let mut entries = vec![first_group];
        for _ in 0..additional {
            let (_, entry) = self.choose_entry(true);
            entries.push(entry);
        }

        return Self::make_groups(&entries, Pattern::Mixed);
    }

    pub fn encount(&self) -> Encount {
        let (index, entry) = self.choose_entry(false);
        return if index < Self::MIXED_GROUP_UPPER {
            Encount {
                groups: self.encount_mixed(&entry),
            }
        } else if index < Self::SINGLE_GROUP_UPPER {
            Encount {
                groups: Self::make_groups(&[entry], Pattern::Single),
            }
        } else {
            // TODO: implement
            Encount {
                groups: Self::make_groups(&[entry], Pattern::Single),
            }
        };
    }
}

fn encount_tables_from_csv(rdr: &mut csv::Reader<&[u8]>) -> Vec<Table> {
    let mut tables = Vec::new();

    for result in rdr.records() {
        let record = result.unwrap();
        tables.push(Table::from_record(&record));
    }
    return tables;
}

lazy_static! {
    pub static ref TABLES: Vec<Table> = {
        let encount_table_csv = include_str!("../assets/encount_table.csv");
        let mut rdr = csv::Reader::from_reader(encount_table_csv.as_bytes());
        encount_tables_from_csv(&mut rdr)
    };
}

pub fn simulate(table_index: usize, iter: usize) -> Vec<Encount> {
    let table = &TABLES[table_index];
    return (0..iter).map(|_| table.encount()).collect();
}

fn sort(encounts: &Vec<Encount>) -> Vec<Encount> {
    let mut sorted = encounts.clone();
    for encount in sorted.iter_mut() {
        encount.sort();
    }

    return sorted;
}

pub fn encount_distribution(encounts: &Vec<Encount>) -> HashMap<Encount, f64> {
    let sorted = sort(encounts);

    let mut histogram = HashMap::<Encount, usize>::new();
    for encount in sorted.iter() {
        let count = histogram.entry(encount.clone()).or_default();
        *count += 1;
    }

    let distribution: HashMap<Encount, f64> = histogram
        .iter()
        .map(|(k, &v)| (k.clone(), (v as f64) / (encounts.len() as f64)))
        .collect();
    return distribution;
}

pub fn monster_distribution(encounts: &Vec<Encount>) -> HashMap<monster::Kind, f64> {
    let mut histogram = HashMap::<monster::Kind, usize>::new();
    for encount in encounts.iter() {
        let mut monster_set = HashSet::new();
        for group in encount.groups() {
            monster_set.insert(group.monster());
        }

        for monster in &monster_set {
            let count = histogram.entry(*monster).or_default();
            *count += 1;
        }
    }

    let distribution: HashMap<monster::Kind, f64> = histogram
        .iter()
        .map(|(&k, &v)| (k, (v as f64) / (encounts.len() as f64)))
        .collect();
    return distribution;
}

#[derive(Clone, Debug, Serialize)]
pub struct EncountSimulation {
    monster_dists: HashMap<monster::Kind, f64>,
    encount_dists: HashMap<Encount, f64>,
}

#[wasm_bindgen]
pub fn encount_simulation_for_js(table_index: usize, iter: usize) -> JsValue {
    let encounts = simulate(table_index, iter);
    let monster_dists = monster_distribution(&encounts);
    let encount_dists = encount_distribution(&encounts);
    let ret = EncountSimulation {
        monster_dists,
        encount_dists,
    };
    return JsValue::from_serde(&ret).unwrap();
}

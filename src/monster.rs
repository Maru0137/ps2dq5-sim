use std::fmt;

fn monster_names_from_csv(rdr: &mut csv::Reader<&[u8]>) -> Vec<String> {
    let mut names = Vec::new();

    for result in rdr.records() {
        let record = result.unwrap();
        let name: String = record[0].parse().unwrap();
        names.push(name);
    }
    return names;
}

lazy_static! {
    static ref NAMES: Vec<String> = {
        let monsters_csv = include_str!("../assets/monsters.csv");
        let mut rdr = csv::Reader::from_reader(monsters_csv.as_bytes());
        monster_names_from_csv(&mut rdr)
    };
}

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Kind(pub usize);

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &NAMES[self.0])
    }
}

impl fmt::Debug for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Kind")
            .field("id", &self.0)
            .field("name", &self.to_string())
            .finish()
    }
}

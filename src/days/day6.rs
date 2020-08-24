use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Clone, Debug)]
struct Planet {
    name: String,
    dist: i64,
}

fn get_planet_hashmap(data_list: Vec<&str>) -> HashMap<String, Planet> {
    let mut data: HashMap<String, Planet> = HashMap::new();
    for elt in data_list {
        let parts: Vec<&str> = elt.split(")").collect();
        data.insert(
            parts[1].to_string(),
            Planet {
                name: parts[0].to_string(),
                dist: 0,
            },
        );
    }
    data
}

fn get_planet_set(data_list: Vec<&str>) -> HashSet<String> {
    let mut data: HashSet<String> = HashSet::new();
    for elt in data_list {
        let parts: Vec<&str> = elt.split(")").collect();
        data.insert(parts[1].to_string());
        data.insert(parts[0].to_string());
    }
    data
}

pub fn main() {
    let data = fs::read_to_string("./src/data/day6.txt").expect("Error");
    let data_list: Vec<_> = data.trim().split("\n").collect();

    let planets = get_planet_hashmap(data_list.clone());
    let planets_set = get_planet_set(data_list.clone());

    let mut uni = Universe { map: planets, set: planets_set };
    let dist = uni.get_total_distance();
    let path_you = uni.get_path_to_com("YOU".to_string());
    let path_san = uni.get_path_to_com("SAN".to_string());
    let stuff: HashSet<_> = path_you.symmetric_difference(&path_san).collect();

    println!("Part 1: {:?}", dist);
    println!("Part 2: {:?}", &stuff.len());

}

struct Universe {
    map: HashMap<String, Planet>,
    set: HashSet<String>,
}

impl Universe {
    fn get_distance(&mut self, name: &str) -> i64 {
        if name == "COM" {
            0
        }
        else if self.map.get(&name.to_string()).unwrap().dist > 0 {
            self.map.get(&name.to_string()).unwrap().dist
        } else {
            let next_name = self.map.get(&name.to_string()).unwrap().name.clone();
            let new_dist = self.get_distance(&next_name[..]) + 1;
            let planet = self.map.get_mut(&name.to_string()).unwrap();
            planet.dist = new_dist;
            planet.dist
        }
    }
    fn get_total_distance(&mut self) -> i64 {
        let mut total = 0;
        for name in self.set.clone() {
            total += self.get_distance(&name[..]);
        }
        total
    }
    fn get_path_to_com(&self, mut name: String) -> HashSet<String> {
        let mut path = HashSet::<String>::new();
        while name != "COM" {
            name = self.map.get(&name[..]).unwrap().name.to_string();
            path.insert(name.clone());
        }
        path.insert("COM".to_string());
        path
    }
}

use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::fs;

#[derive(Clone, Debug)]
struct Planet {
    name: String,
    dist: i64,
}


pub fn main() {
    println!("hello");
    let data = fs::read_to_string("./src/data/day6.txt").expect("Error");
    let mut data_list: Vec<_> = data.trim().split("\n").collect();
    data_list.sort();
    let mut planets = get_planet_hashmap(data_list.clone());
    let mut uni = Universe { map: planets };
    let x = uni.get_distance("WLQ");
    println!("{:?}", x);
}

struct Universe {
    map: HashMap<String, Planet>,
}

impl Universe {
    fn get_distance(mut self, name: &str) -> i64 {
        if name == "COM" {
            0 as i64
        }
        else {
            let planet = self.map.get_mut(name).unwrap();
            println!("{:?}", planet);
            let name = planet.name.clone();
            planet.dist = (self).get_distance(&name[..]) + 1;
            planet.dist
        }
    }
}

fn get_planet_hashmap(data_list: Vec<&str>) -> HashMap<String, Planet> {
    let mut data: HashMap<String, Planet> = HashMap::new();
    for elt in data_list {
        let parts: Vec<&str> = elt.split(")").collect();
        data.insert(parts[1].to_string(), Planet { name: parts[0].to_string(), dist: 0 });
    }
    data
}

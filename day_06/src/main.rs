use std::collections::HashMap;
use std::fs;
use std::time::Instant;

#[derive(Clone)]
struct Orb {
    parent: u32,
    distance: u32,
}

struct Map {
    orbs: Vec<Orb>,
    orb_names_to_id: HashMap<String, u32>,
}

fn main() {
    let now = Instant::now();
    let puzzle = fs::read_to_string("./puzzle.txt").unwrap();

    let mut map = Map {
        orbs: Vec::with_capacity(1800),
        orb_names_to_id: HashMap::with_capacity(1800),
    };
    let mut unique_orbs = Vec::new();

    let mut you_id = 0;
    let mut san_id = 0;

    for line in puzzle.lines() {
        if line.is_empty() {
            continue;
        }

        let (orb_left_name, orb_right_name) = line.split_once(')').unwrap();
        let is_com = orb_left_name == "COM";
        let is_san = orb_right_name == "SAN";
        let is_you = orb_right_name == "YOU";
        let (orb_left_id, orb_right_id) = (
            map.orb_name_to_id(orb_left_name),
            map.orb_name_to_id(orb_right_name),
        );

        map.orbs[orb_right_id as usize] = Orb {
            parent: orb_left_id,
            distance: if is_com { 1 } else { 0 },
        };
        unique_orbs.push(orb_right_id);

        if is_san {
            san_id = orb_right_id;
        } else if is_you {
            you_id = orb_right_id;
        }
    }

    let mut total_distance = 0;
    for orb_id in unique_orbs {
        total_distance += map.calculate_distance(orb_id);
    }
    println!("{}", total_distance);

    let mut you_chain = map.chain(you_id);
    let mut san_chain = map.chain(san_id);

    loop {
        // Pop elements from the chians untils they diverge
        if san_chain.pop().unwrap() != you_chain.pop().unwrap() {
            break;
        }
    }
    println!("{}", san_chain.len() + you_chain.len());

    println!("Elapsed: {:.2?}", now.elapsed());
}

impl Map {
    fn orb_name_to_id(&mut self, name: &str) -> u32 {
        if let Some(orb_id) = self.orb_names_to_id.get(name) {
            return *orb_id;
        }

        let id = self.orbs.len() as u32;
        self.orb_names_to_id.insert(name.to_string(), id);

        self.orbs.push(Orb {
            parent: 0,
            distance: 0,
        });

        id
    }
    fn calculate_distance(&mut self, orb_id: u32) -> u32 {
        let orb = &self.orbs[orb_id as usize];
        if orb.distance > 0 {
            return orb.distance;
        }

        let new_distance = self.calculate_distance(orb.parent) + 1;
        self.orbs[orb_id as usize].distance = new_distance;

        new_distance
    }
    fn chain(&mut self, mut orb_id: u32) -> Vec<u32> {
        let mut resp = Vec::new();

        loop {
            let orb = &self.orbs[orb_id as usize];
            if orb.distance == 1 {
                break;
            }

            resp.push(orb_id);
            orb_id = orb.parent;
        }

        resp
    }
}

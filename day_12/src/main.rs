use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

struct Moon {
    pos: [i16; 3],
    vel: [i16; 3],
}

fn main() {
    let now = Instant::now();
    let puzzle = fs::read_to_string("./puzzle.txt").unwrap();
    let mut moons: Vec<Moon> = Vec::new();
    let mut moon_prev_states: Vec<HashMap<[i16; 3], Vec<usize>>> = Vec::new();
    let mut moon_diffs: Vec<HashSet<usize>> = Vec::new();

    for line in puzzle.lines() {
        if line.is_empty() {
            continue;
        }

        let mut pos = [0i16; 3];
        for (idx, part) in line.split('=').enumerate() {
            if idx == 0 {
                continue;
            }

            let nr: i16 = match idx {
                1 => part.split(',').nth(0).unwrap(),
                2 => part.split(',').nth(0).unwrap(),
                3 => part.split('>').nth(0).unwrap(),
                _ => continue,
            }
            .parse()
            .unwrap();

            match idx {
                1 => pos[0] = nr,
                2 => pos[1] = nr,
                3 => pos[2] = nr,
                _ => unreachable!(),
            }
        }

        moons.push(Moon { pos, vel: [0; 3] });
        moon_prev_states.push(HashMap::new());
        moon_diffs.push(HashSet::new());
    }

    let mut visited_axis: [HashMap<[i16; 4], usize>; 3] =
        [HashMap::new(), HashMap::new(), HashMap::new()];

    let total_moons = moons.len();
    let mut idx = 0;
    loop {
        let mut axis = [[0i16; 4]; 3];

        for i in 0..total_moons {
            let moon = &moons[i];

            for (j, pos_axis) in moon.pos.iter().enumerate() {
                axis[j][i] = *pos_axis;
            }

            // if let Some(v) = moon_prev_states[i].get_mut(&moon.pos) {
            //     v.push(idx);
            // } else {
            //     moon_prev_states[i].insert(moon.pos, vec![idx]);
            // }
            // if let Some(prev_idx) = moon_prev_states[i].insert(moon.pos, idx) {
            //     let diff = idx - prev_idx;
            //     if !moon_diffs[i].insert(diff) {
            //         println!("moon {} at {} -> {} : {}", i, prev_idx, idx, diff);
            //     }
            // }

            let mut vel = moon.vel;

            for j in 0..total_moons {
                if i == j {
                    continue;
                }
                let cmp_moon = &moons[j].pos;

                for k in 0..3 {
                    if moon.pos[k] == cmp_moon[k] {
                        continue;
                    }
                    if moon.pos[k] < cmp_moon[k] {
                        vel[k] += 1;
                    } else if moon.pos[k] > cmp_moon[k] {
                        vel[k] -= 1;
                    }
                }
            }

            moons[i].vel = vel;
        }

        for i in 0..axis.len() {
            if i == 2 {
                if let Some(prev_idx) = visited_axis[i].get(&axis[i]) {
                    let mut prediction = idx % 102355;
                    if prediction > 0 {
                        prediction -= 1;
                    }
                    println!("{} -> {} pred: {}", prev_idx, idx, prediction);
                } else {
                    visited_axis[i].insert(axis[i], idx);
                }
            }
        }

        for i in 0..total_moons {
            moons[i].pos[0] += moons[i].vel[0];
            moons[i].pos[1] += moons[i].vel[1];
            moons[i].pos[2] += moons[i].vel[2];
        }

        if idx == 999 {
            let mut result_p1 = 0;
            for moon in moons.iter() {
                let potential_energy = moon.pos[0].abs() + moon.pos[1].abs() + moon.pos[2].abs();
                let kinetic_energy = moon.vel[0].abs() + moon.vel[1].abs() + moon.vel[2].abs();
                result_p1 += potential_energy * kinetic_energy;
            }
            println!("{}", result_p1);
        }

        if idx == 1_000_000 {
            break;
        }

        idx += 1;
    }

    println!("Elapsed: {:.2?}", now.elapsed());
}

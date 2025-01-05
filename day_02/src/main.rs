use std::fs;
use std::time::Instant;

const PART_2_EXPECTED: usize = 19690720;

fn main() {
    let now = Instant::now();
    let puzzle = fs::read_to_string("./puzzle.txt").unwrap();

    let mut memory = Vec::new();
    for entry in puzzle.trim_end().split(',') {
        let nr: usize = entry.parse().unwrap();
        memory.push(nr);
    }

    let p1 = run_program(memory.clone(), 12, 2);
    println!("{}", p1);

    let mut nouns = Vec::with_capacity(100);
    for v in 0..100 {
        nouns.push((memory[v], v));
    }
    nouns.sort_by(|a, b| a.0.cmp(&b.0));
    let verbs = nouns.clone();

    for (idx, (_, noun)) in nouns.iter().enumerate() {
        let result = run_program(memory.clone(), *noun, verbs[0].1);
        if result > PART_2_EXPECTED {
            nouns = nouns[0..idx].to_vec();
            break;
        }
    }

    for (idx, (_, verb)) in nouns.iter().enumerate() {
        let result = run_program(memory.clone(), nouns[0].1, *verb);
        if result > PART_2_EXPECTED {
            nouns = nouns[0..idx].to_vec();
            break;
        }
    }

    'outer: for (_, noun) in nouns.iter().rev() {
        for (_, verb) in verbs.iter() {
            let result = run_program(memory.clone(), *noun, *verb);
            if result == PART_2_EXPECTED {
                println!("{}", 100 * noun + verb);
                break 'outer;
            }
            if result > PART_2_EXPECTED {
                break;
            }
        }
    }

    println!("Elapsed: {:.2?}", now.elapsed());
}

fn run_program(mut memory: Vec<usize>, noun: usize, verb: usize) -> usize {
    memory[1] = noun;
    memory[2] = verb;

    let mut instruction_pointer = 0;
    loop {
        let a = memory[memory[instruction_pointer + 1]];
        let b = memory[memory[instruction_pointer + 2]];
        let c_pointer = memory[instruction_pointer + 3];

        match memory[instruction_pointer] {
            1 => {
                memory[c_pointer] = a + b;
            }
            2 => {
                memory[c_pointer] = a * b;
            }
            99 => {
                break;
            }
            op => panic!("unknown opcode {}", op),
        }

        instruction_pointer += 4;
    }

    memory[0]
}

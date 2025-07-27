use std::fs;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let puzzle = fs::read_to_string("./puzzle.txt").unwrap();
    let mut input = Vec::new();
    for c in puzzle.chars() {
        match c {
            '0'..='9' => {
                let digit = c.to_digit(10).unwrap();
                input.push(digit as isize);
            }
            _ => {}
        }
    }

    let patterns = generate_pattern(input.len());

    let offset = input.iter().take(7).fold(0, |acc, &x| acc * 10 + x) as usize;
    println!("offset: {}", offset);

    let mut skipped = 0;
    let mut part_two_input = input.clone();
    for nr in 0..10_000 {
        input = calculate_stage(&patterns, &input);

        if nr == 99 {
            print!("p1: ");
            for idx in 0..8 {
                print!("{}", input[idx]);
            }
            println!();
        }

        for nr in &input {
            if skipped >= offset {
                part_two_input.push(*nr);
                continue;
            }
            skipped += 1;
        }
    }

    println!("part 2 list len: {}", part_two_input.len());
    input = part_two_input;

    for _ in 0..100 {
        let mut new_input = Vec::with_capacity(input.len());
        let mut total = 0;
        for nr in input.iter().rev() {
            total += nr;
            new_input.push((total % 10).abs());
        }

        new_input.reverse();
        input = new_input;
    }

    print!("p2: ");
    for idx in 0..8 {
        print!("{}", input[idx]);
    }
    println!();

    println!("Elapsed: {:.2?}", now.elapsed());
}

fn calculate_stage(patterns: &Vec<Vec<isize>>, input: &Vec<isize>) -> Vec<isize> {
    let mut new_input = input.clone();
    for (idx, pattern) in patterns.iter().enumerate() {
        let mut line_result = 0;
        for sub_idx in idx..pattern.len() {
            line_result += input[sub_idx] * pattern[sub_idx];
        }
        if line_result < 0 {
            line_result = -line_result;
        }
        line_result %= 10;
        new_input[idx] = line_result;
    }
    new_input
}

fn generate_pattern(input_len: usize) -> Vec<Vec<isize>> {
    let mut patterns = Vec::new();
    let pattern_len = input_len + 1;
    for repeat in 1..=input_len {
        let mut pattern = Vec::new();
        'outer: loop {
            for entry in [0, 1, 0, -1] {
                for _ in 0..repeat {
                    pattern.push(entry);
                    if pattern.len() == pattern_len {
                        break 'outer;
                    }
                }
            }
        }
        pattern.remove(0);
        patterns.push(pattern);
    }
    patterns
}

use std::fs;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let puzzle = fs::read_to_string("./puzzle.txt").unwrap();

    let mut part_1 = 0;
    let mut part_2 = 0;
    for line in puzzle.lines() {
        if line.is_empty() {
            continue;
        }

        let mut nr: usize = line.parse().unwrap();
        let mut i = 0;
        loop {
            if nr < 9 {
                break;
            }
            nr = nr / 3 - 2;
            if i == 0 {
                part_1 += nr;
            }
            part_2 += nr;
            i += 1;
        }
    }

    println!("{}", part_1);
    println!("{}", part_2);

    println!("Elapsed: {:.2?}", now.elapsed());
}

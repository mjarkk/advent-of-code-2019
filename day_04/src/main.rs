use std::fs;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let puzzle = fs::read_to_string("./puzzle.txt").unwrap();

    let (min_str, max_str) = puzzle.lines().nth(0).unwrap().split_once('-').unwrap();
    let min = min_str.parse::<u32>().unwrap();
    let max = max_str.parse::<u32>().unwrap();

    let (p1, p2) = solve(5, min, max, 0, 1);
    println!("{}", p1 + p2);
    println!("{}", p2);

    println!("Elapsed: {:.2?}", now.elapsed());
}

fn solve(offset: u32, min: u32, max: u32, base: u32, offset_by: u32) -> (u32, u32) {
    let mut result_p1 = 0;
    let mut result_p2 = 0;
    let exp = 10u32.pow(offset);

    'outer: for i in offset_by..=9 {
        let sub_base = i * exp + base;
        if sub_base > max {
            break;
        }

        if offset != 0 {
            let results = solve(offset - 1, min, max, sub_base, i);
            result_p1 += results.0;
            result_p2 += results.1;
            continue;
        }

        if sub_base < min {
            continue;
        }

        let mut p1_matches = false;
        let mut last_digit = sub_base % 10;
        let mut eq_digits = 0;
        for j in 1..6 {
            let digit = sub_base / 10u32.pow(j) % 10;
            if digit == last_digit {
                eq_digits += 1;
                continue;
            }

            if eq_digits == 1 {
                result_p2 += 1;
                continue 'outer;
            }
            if eq_digits > 1 {
                p1_matches = true;
            }
            eq_digits = 0;
            last_digit = digit;
        }

        if eq_digits == 1 {
            result_p2 += 1;
            continue;
        }
        if eq_digits > 1 || p1_matches {
            result_p1 += 1;
        }
    }

    (result_p1, result_p2)
}

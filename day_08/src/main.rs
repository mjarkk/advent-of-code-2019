use std::fs;
use std::time::Instant;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

struct Layer {
    data: Vec<char>,
    zeros: u8,
    ones: u8,
    twos: u8,
}

fn main() {
    let now = Instant::now();
    let puzzle = fs::read_to_string("./puzzle.txt").unwrap();

    let mut layers: Vec<Layer> = Vec::new();
    for (idx, c) in puzzle.chars().enumerate() {
        if c == '\n' {
            break;
        }

        if idx % (WIDTH * HEIGHT) == 0 {
            layers.push(Layer {
                data: Vec::new(),
                zeros: 0,
                ones: 0,
                twos: 0,
            });
        }

        let layer = layers.last_mut().unwrap();
        layer.data.push(c);
        match c {
            '0' => layer.zeros += 1,
            '1' => layer.ones += 1,
            '2' => layer.twos += 1,
            _ => (),
        }
    }

    let mut fewest_zeros = (WIDTH * HEIGHT) as u8;
    let mut result = 0;
    for layer in layers.iter() {
        if layer.zeros < fewest_zeros {
            fewest_zeros = layer.zeros;
            result = layer.ones as u32 * layer.twos as u32;
        }
    }

    println!("{}", result);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let mut color = '2';
            for layer in layers.iter() {
                let c = layer.data[y * WIDTH + x];
                if c != '2' {
                    color = c;
                    break;
                }
            }
            print!("{}", if color == '0' { ' ' } else { '█' });
        }
        println!();
    }

    println!("Elapsed: {:.2?}", now.elapsed());
}

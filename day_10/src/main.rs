mod search_base;

use core::panic;
use crossterm::{
    cursor::{self, MoveTo},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::stdout;
use std::time::Instant;
use std::{fs, thread::sleep, time::Duration};

fn main() {
    let now = Instant::now();
    let puzzle = fs::read_to_string("./puzzle.txt").unwrap();

    let mut state = State {
        map: Vec::new(),
        map_size: (0, 0),
        search_cords: Vec::new(),
        circle_cords: Vec::new(),
    };

    for line in puzzle.lines() {
        if line.is_empty() {
            continue;
        }

        let mut row: Vec<bool> = Vec::new();
        for c in line.chars() {
            row.push(c == '#');
        }
        state.map.push(row);
    }
    state.map_size = (state.map[0].len() as i16, state.map.len() as i16);

    (state.search_cords, state.circle_cords) =
        search_base::search_cords(state.map_size.1.max(state.map_size.0) as usize);

    let mut best_score = 0;
    let mut best_score_cords = (0, 0);

    let y_offset = state.map_size.1 / 10;
    let x_offset = state.map_size.0 / 10;
    for y in y_offset..state.map_size.1 - y_offset {
        for x in x_offset..state.map_size.0 - x_offset {
            if !state.map[y as usize][x as usize] {
                continue;
            }

            let score = state.calculate_score(x as i16, y as i16);
            if score > best_score {
                best_score = score;
                best_score_cords = (x, y);
            }
        }
    }

    println!("{}", best_score);

    let result_part_2 = state.vaporize_asteroid(best_score_cords, false);
    println!("{}", result_part_2.0 * 100 + result_part_2.1);

    println!("Elapsed: {:.2?}", now.elapsed());
}

struct State {
    map: Vec<Vec<bool>>,
    map_size: (i16, i16),
    search_cords: Vec<Vec<Vec<(i16, i16, f32)>>>,
    circle_cords: Vec<(i16, i16, f32)>,
}

impl State {
    fn calculate_score(&self, x: i16, y: i16) -> usize {
        let mut score = 0;
        for side_layers in self.search_cords.iter() {
            for layer in side_layers.iter() {
                let mut hits = false;
                for (dx, dy, _) in layer.iter() {
                    let mut x_offset = x + dx;
                    let mut y_offset = y + dy;

                    while x_offset >= 0
                        && x_offset < self.map_size.0
                        && y_offset >= 0
                        && y_offset < self.map_size.1
                    {
                        hits = true;
                        if self.map[y_offset as usize][x_offset as usize] {
                            score += 1;
                            break;
                        }

                        x_offset += dx;
                        y_offset += dy;
                    }
                }
                if !hits {
                    break;
                }
            }
        }

        score
    }

    fn vaporize_asteroid(&mut self, (x, y): (i16, i16), debug: bool) -> (i16, i16) {
        let mut removed = 0;
        let mut cords_from_circle_to_remove: Vec<usize> = Vec::new();

        if debug {
            _ = execute!(stdout(), EnterAlternateScreen, cursor::Hide);

            for y in 0..self.map_size.1 {
                let mut chars = Vec::new();
                for x in 0..self.map_size.0 {
                    if self.map[y as usize][x as usize] {
                        chars.push('#');
                    } else {
                        chars.push('.');
                    }
                }
                _ = execute!(
                    stdout(),
                    MoveTo(0, y as u16),
                    Print(chars.iter().cloned().collect::<String>()),
                );
            }

            _ = execute!(
                stdout(),
                MoveTo(x as u16, y as u16),
                SetForegroundColor(Color::Blue),
                Print("#"),
                ResetColor,
            );
        }

        loop {
            for (idx, (dx, dy, _)) in self.circle_cords.iter().enumerate() {
                let mut x_offset = x + dx;
                let mut y_offset = y + dy;

                let mut vaporized_some = false;
                while x_offset >= 0
                    && x_offset < self.map_size.0
                    && y_offset >= 0
                    && y_offset < self.map_size.1
                {
                    if self.map[y_offset as usize][x_offset as usize] {
                        // Vaporize this asteroid
                        self.map[y_offset as usize][x_offset as usize] = false;
                        vaporized_some = true;
                        removed += 1;
                        if removed == 200 {
                            if debug {
                                _ = execute!(stdout(), LeaveAlternateScreen, cursor::Show);
                            }
                            return (x_offset, y_offset);
                        }

                        if debug {
                            _ = execute!(
                                stdout(),
                                MoveTo(x_offset as u16, y_offset as u16),
                                SetForegroundColor(Color::Red),
                                Print("#"),
                                ResetColor,
                            );
                            sleep(Duration::from_millis(100));
                        }

                        break;
                    }

                    x_offset += dx;
                    y_offset += dy;
                }

                if !vaporized_some {
                    // Remove this layer from the circle cords
                    cords_from_circle_to_remove.push(idx);
                }
            }

            for idx in cords_from_circle_to_remove.iter().rev() {
                self.circle_cords.remove(*idx);
            }
            if self.circle_cords.is_empty() {
                panic!("No more asteroids to vaporize");
            }
            cords_from_circle_to_remove.clear();
        }
    }
}

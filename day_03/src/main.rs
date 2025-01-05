use std::fs;
use std::time::Instant;

enum Axis {
    X,
    Y,
}

struct Line {
    position: isize,
    start: isize,
    end: isize,
    cost: isize,
    source_start: isize,
}

impl Line {
    fn overlaps(&self, line: (isize, isize), pos: isize) -> Option<(isize, isize)> {
        if pos < self.start || pos > self.end {
            None
        } else if pos == 0 && self.position == 0 {
            None
        } else if self.position < line.0.min(line.1) || self.position > line.0.max(line.1) {
            None
        } else {
            let cost = self.cost + (line.0 - self.position).abs() + (pos - self.source_start).abs();
            let distance = self.position.abs() + pos.abs();
            Some((distance, cost))
        }
    }
}

fn main() {
    let now = Instant::now();
    let puzzle = fs::read_to_string("./puzzle.txt").unwrap();

    let mut vertical_lines: Vec<Line> = Vec::new();
    let mut horizontal_lines: Vec<Line> = Vec::new();
    let mut answer_p1 = 0;
    let mut answer_p2 = 0;

    for (idx, line) in puzzle.lines().enumerate() {
        let mut pos = (0isize, 0isize);
        let mut total_cost = 0;
        if line.is_empty() {
            continue;
        }

        assert!(idx < 2);

        for entry in line.split(',') {
            let direction = entry.chars().nth(0).unwrap();
            let distance: isize = entry[1..].parse().unwrap();

            let (end, axis) = match direction {
                'U' => (pos.1 - distance, Axis::Y),
                'R' => (pos.0 + distance, Axis::X),
                'D' => (pos.1 + distance, Axis::Y),
                'L' => (pos.0 - distance, Axis::X),
                _ => panic!("Invalid direction"),
            };

            if idx == 0 {
                match axis {
                    Axis::X => {
                        horizontal_lines.push(Line {
                            position: pos.1,
                            start: end.min(pos.0),
                            end: end.max(pos.0),
                            cost: total_cost,
                            source_start: pos.0,
                        });
                        pos.0 = end;
                    }
                    Axis::Y => {
                        vertical_lines.push(Line {
                            position: pos.0,
                            start: end.min(pos.1),
                            end: end.max(pos.1),
                            cost: total_cost,
                            source_start: pos.1,
                        });
                        pos.1 = end;
                    }
                }

                total_cost = total_cost + distance;
                continue;
            }

            match axis {
                Axis::X => {
                    // Check if the line intersects with any vertical lines
                    for line in vertical_lines.iter() {
                        if let Some((distance, partial_cost)) = line.overlaps((pos.0, end), pos.1) {
                            if answer_p1 == 0 || distance < answer_p1 {
                                answer_p1 = distance;
                            }

                            let cost = total_cost + partial_cost;
                            if answer_p2 == 0 || cost < answer_p2 {
                                answer_p2 = cost;
                            }

                            break;
                        }
                    }

                    pos.0 = end;
                }
                Axis::Y => {
                    // Check if the line intersects with any horizontal lines
                    for line in horizontal_lines.iter() {
                        if let Some((distance, partial_cost)) = line.overlaps((pos.1, end), pos.0) {
                            if answer_p1 == 0 || distance < answer_p1 {
                                answer_p1 = distance;
                            }

                            let cost = total_cost + partial_cost;
                            if answer_p2 == 0 || cost < answer_p2 {
                                answer_p2 = cost;
                            }

                            break;
                        }
                    }

                    pos.1 = end;
                }
            }

            total_cost = total_cost + distance;
        }
    }

    println!("{}", answer_p1);
    println!("{}", answer_p2);
    println!("Elapsed: {:.2?}", now.elapsed());
}

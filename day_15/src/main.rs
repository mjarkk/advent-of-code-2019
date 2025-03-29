mod vm;

use crossterm::{cursor, event, execute, style::Print, terminal};
use std::fs;
use std::io::stdout;
use std::time::Instant;
use vm::{Interupt, Program};

const DEBUG: bool = false;
const MAP_SIZE: usize = 50;

fn main() {
    let now = Instant::now();
    let puzzle = fs::read_to_string("./puzzle.txt").unwrap();

    let t = Instant::now();
    let mut runtime = Runtime::default();
    for code in puzzle.split(',') {
        let code: i64 = if let Ok(v) = code.parse() {
            v
        } else {
            code.split('\n').nth(0).unwrap().parse().unwrap()
        };
        runtime.source_memory.push(code);
    }
    println!("read puzzle duration: {:.2?}", t.elapsed());

    let end = runtime.discover_map();

    let p1 = ShortestPathFinder::find(&runtime.map, runtime.start, end).unwrap();
    println!("p1: {}", p1);

    let p2 = find_longest_path(runtime.map, end);
    println!("p2: {}", p2);

    println!("Elapsed: {:.2?}", now.elapsed());
}

#[derive(Copy, Clone)]
enum Location {
    Undiscovered,
    Empty,
    Wall,
}

#[derive(Debug)]
enum DroidDirection {
    Up,
    Down,
    Left,
    Right,
}

impl DroidDirection {
    fn to_num(&self) -> i64 {
        match self {
            DroidDirection::Up => 1,
            DroidDirection::Down => 2,
            DroidDirection::Right => 4,
            DroidDirection::Left => 3,
        }
    }
    fn move_location(&self, location: (usize, usize)) -> (usize, usize) {
        match self {
            DroidDirection::Up => (location.0, location.1 - 1),
            DroidDirection::Down => (location.0, location.1 + 1),
            DroidDirection::Right => (location.0 + 1, location.1),
            DroidDirection::Left => (location.0 - 1, location.1),
        }
    }
}

#[derive(Default)]
struct Runtime {
    source_memory: Vec<i64>,
    program: Program,
    map: Vec<Vec<Location>>,
    start: (usize, usize),
    location: (usize, usize),
    end: Option<(usize, usize)>,
}

impl Runtime {
    fn discover_map(&mut self) -> (usize, usize) {
        if DEBUG {
            terminal::enable_raw_mode().unwrap();
            _ = execute!(
                stdout(),
                terminal::EnterAlternateScreen,
                cursor::Hide,
                cursor::DisableBlinking,
                event::EnableFocusChange,
                event::EnableMouseCapture,
            );
        }

        self.reset();

        for _ in 0..MAP_SIZE {
            self.map.push(vec![Location::Undiscovered; MAP_SIZE]);
        }
        self.start = (MAP_SIZE / 2, MAP_SIZE / 2);
        self.location = self.start;

        loop {
            if DEBUG {
                self.print_map();
            }

            let mut moved = true;
            while moved {
                moved = false;
                for direction in [
                    DroidDirection::Up,
                    DroidDirection::Down,
                    DroidDirection::Right,
                    DroidDirection::Left,
                ] {
                    loop {
                        let new_location = direction.move_location(self.location);
                        match self.map[new_location.1][new_location.0] {
                            Location::Wall | Location::Empty => break,
                            Location::Undiscovered => {
                                self.walk(&direction);
                                moved = true;
                            }
                        }
                    }
                }
            }

            if let Some(path) = self.empty_location_to_explore() {
                for direction in &path {
                    self.walk(direction);
                }
            } else {
                break;
            }
        }

        if DEBUG {
            terminal::disable_raw_mode().unwrap();
            _ = execute!(
                stdout(),
                terminal::LeaveAlternateScreen,
                cursor::Show,
                cursor::EnableBlinking,
                event::DisableFocusChange,
                event::DisableMouseCapture,
            );
        }

        self.end.expect("End location must be found")
    }
    fn print_map(&mut self) {
        for (y, row) in self.map.iter().enumerate() {
            for (x, data) in row.iter().enumerate() {
                let location = (x, y);
                let c = match data {
                    _ if location == self.start => 'S',
                    Location::Undiscovered => ' ',
                    Location::Empty => '.',
                    Location::Wall => '#',
                };
                _ = execute!(
                    stdout(),
                    cursor::MoveTo(x as u16, y as u16),
                    Print(c.to_string())
                );
            }
        }
    }
    fn walk(&mut self, direction: &DroidDirection) {
        let mut input = vec![direction.to_num()];
        match self.program.run(&mut input) {
            Interupt::Halt => panic!("Halted"),
            Interupt::Input => panic!("Expected output, got input"),
            Interupt::Output(v) => {
                let new_location = direction.move_location(self.location);
                match v {
                    0 => {
                        // Droid hit a wall, change direction
                        self.map[new_location.1][new_location.0] = Location::Wall;
                    }
                    1 => {
                        // Walked one step
                        self.location = direction.move_location(self.location);
                        self.map[new_location.1][new_location.0] = Location::Empty;
                    }
                    2 => {
                        self.location = direction.move_location(self.location);
                        self.map[new_location.1][new_location.0] = Location::Empty;
                        self.end = Some(self.location);
                    }
                    v => panic!("Unknown output: {}", v),
                };
            }
        }
    }
    fn path_to_location(
        &self,
        from: (usize, usize),
        to: (usize, usize),
    ) -> Option<Vec<DroidDirection>> {
        PathFinder::find(&self.map, from, to)
    }
    fn empty_location_to_explore(&mut self) -> Option<Vec<DroidDirection>> {
        let mut offset = 0;
        let mut to_check = Vec::new();
        let base_location = (self.location.0 as isize, self.location.1 as isize);
        loop {
            to_check.clear();

            offset += 1;
            let side_len = offset * 2 + 1;

            // Left side
            let x = base_location.0 - offset;
            let y = base_location.1 - offset;
            for i in 0..side_len {
                if let Some(v) = valid_cord(x, y + i) {
                    to_check.push(v);
                }
            }

            // Top side
            let x = base_location.0 + offset;
            let y = base_location.1 - offset;
            for i in 0..side_len {
                if let Some(v) = valid_cord(x - i, y) {
                    to_check.push(v);
                }
            }

            // Right side
            let x = base_location.0 + offset;
            let y = base_location.1 + offset;
            for i in 0..side_len {
                if let Some(v) = valid_cord(x, y - i) {
                    to_check.push(v);
                }
            }

            // Bottom side
            let x = base_location.0 - offset;
            let y = base_location.1 + offset;
            for i in 0..side_len {
                if let Some(v) = valid_cord(x + i, y) {
                    to_check.push(v);
                }
            }

            if to_check.is_empty() {
                return None;
            }

            let mut seen_empty_tiles = false;
            let mut has_tiles = false;
            for location in &to_check {
                if let Location::Undiscovered = self.map[location.1][location.0] {
                    seen_empty_tiles = true;
                    if let Some(path) = self.path_to_location(self.location, *location) {
                        return Some(path);
                    }
                } else {
                    has_tiles = true;
                }
            }
            if !has_tiles && seen_empty_tiles {
                return None;
            }
        }
    }
    fn reset(&mut self) {
        self.program.reset(self.source_memory.clone());
    }
}

fn valid_cord(x: isize, y: isize) -> Option<(usize, usize)> {
    if x < 0 || y < 0 || x >= MAP_SIZE as isize || y >= MAP_SIZE as isize {
        return None;
    }
    Some((x as usize, y as usize))
}

struct PathFinder<'a> {
    map: &'a Vec<Vec<Location>>,
    visited: Vec<bool>,
    dest: (usize, usize),
}

impl<'a> PathFinder<'a> {
    fn find(
        map: &'a Vec<Vec<Location>>,
        from: (usize, usize),
        to: (usize, usize),
    ) -> Option<Vec<DroidDirection>> {
        let mut finder = Self {
            map,
            visited: vec![false; MAP_SIZE * MAP_SIZE],
            dest: to,
        };

        let mut path = finder.resolve(from)?;

        path.reverse();
        Some(path)
    }

    fn resolve(&mut self, location: (usize, usize)) -> Option<Vec<DroidDirection>> {
        if location == self.dest {
            return Some(Vec::new());
        }

        self.visited[location.1 * MAP_SIZE + location.0] = true;

        for direction in [
            DroidDirection::Up,
            DroidDirection::Left,
            DroidDirection::Down,
            DroidDirection::Right,
        ] {
            let new_location = direction.move_location(location);
            if self.visited[new_location.1 * MAP_SIZE + new_location.0] {
                continue;
            }

            match &self.map[new_location.1][new_location.0] {
                Location::Undiscovered => {
                    if new_location == self.dest {
                        return Some(vec![direction]);
                    }
                    continue;
                }
                Location::Wall => continue,
                Location::Empty => { /* NOP */ }
            }

            if let Some(path) = self.resolve(new_location) {
                let mut path = path;
                path.push(direction);
                return Some(path);
            }
        }

        None
    }
}

struct ShortestPathFinder<'a> {
    map: &'a Vec<Vec<Location>>,
    visited: Vec<Option<usize>>,
    dest: (usize, usize),
}

impl<'a> ShortestPathFinder<'a> {
    fn find(
        map: &'a Vec<Vec<Location>>,
        from: (usize, usize),
        to: (usize, usize),
    ) -> Option<usize> {
        let mut finder = Self {
            map,
            visited: vec![None; MAP_SIZE * MAP_SIZE],
            dest: to,
        };

        finder.resolve(from, 0)
    }

    fn resolve(&mut self, location: (usize, usize), cost: usize) -> Option<usize> {
        if location == self.dest {
            return Some(cost);
        }

        self.visited[location.1 * MAP_SIZE + location.0] = Some(cost);
        let new_cost = cost + 1;
        let mut lowest_cost = None;

        for direction in [
            DroidDirection::Up,
            DroidDirection::Left,
            DroidDirection::Down,
            DroidDirection::Right,
        ] {
            let new_location = direction.move_location(location);

            if let Some(cost) = self.visited[new_location.1 * MAP_SIZE + new_location.0] {
                // This route is more expensive than a previous one
                // Skip!
                if cost <= new_cost {
                    continue;
                }
            }

            match self.map[new_location.1][new_location.0] {
                Location::Undiscovered | Location::Wall => continue,
                Location::Empty => { /* NOP */ }
            }

            if let Some(cost) = self.resolve(new_location, new_cost) {
                if let Some(current_cost) = lowest_cost {
                    if cost < current_cost {
                        lowest_cost = Some(cost);
                    }
                } else {
                    lowest_cost = Some(cost);
                }
            }
        }

        lowest_cost
    }
}

fn find_longest_path(map: Vec<Vec<Location>>, from: (usize, usize)) -> usize {
    let mut visited = vec![false; MAP_SIZE * MAP_SIZE];
    visited[from.1 * MAP_SIZE + from.0] = true;

    let mut queue = vec![(from.0, from.1, 0)];

    loop {
        let mut new_queue = Vec::new();
        let mut most_expensive = 0;
        for location_and_cost in queue {
            if location_and_cost.2 > most_expensive {
                most_expensive = location_and_cost.2;
            }
            visited[location_and_cost.1 * MAP_SIZE + location_and_cost.0] = true;

            for direction in [
                DroidDirection::Up,
                DroidDirection::Left,
                DroidDirection::Down,
                DroidDirection::Right,
            ] {
                let new_location =
                    direction.move_location((location_and_cost.0, location_and_cost.1));

                if visited[new_location.1 * MAP_SIZE + new_location.0] {
                    continue;
                }

                match map[new_location.1][new_location.0] {
                    Location::Undiscovered | Location::Wall => continue,
                    Location::Empty => { /* NOP */ }
                }

                new_queue.push((new_location.0, new_location.1, location_and_cost.2 + 1));
            }
        }
        if new_queue.is_empty() {
            return most_expensive;
        }
        queue = new_queue;
    }
}

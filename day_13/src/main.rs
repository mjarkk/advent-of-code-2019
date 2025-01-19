mod vm;

use crossterm::{cursor, event, execute, style, style::Print, terminal};
use std::fs;
use std::io::stdout;
use std::thread::sleep;
use std::time::{Duration, Instant};
use vm::{Interupt, Program};

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

    let total_blocks = runtime.count_blocks();
    println!("{}", total_blocks);

    let t = Instant::now();
    let score = runtime.play(false);
    println!("{}", score);
    println!("part 2 duration: {:.2?}", t.elapsed());

    println!("Elapsed: {:.2?}", now.elapsed());
}

#[derive(Copy, Clone)]
enum Tail {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

#[derive(Default)]
struct Runtime {
    source_memory: Vec<i64>,
    program: Program,
}

impl Runtime {
    fn count_blocks(&mut self) -> i64 {
        self.reset();

        let mut blocks = 0;
        let mut input = Vec::new();
        let mut out_nr = 0;
        loop {
            match self.program.start(&mut input) {
                Interupt::Halt => break,
                Interupt::Input => panic!("Input required"),
                Interupt::Output(v) => {
                    if out_nr == 2 && v == 2 {
                        blocks += 1;
                    }
                    out_nr = (out_nr + 1) % 3;
                }
            }
        }

        blocks
    }
    fn play(&mut self, debug: bool) -> i64 {
        self.reset();
        self.program.memory[0] = 2;

        let mut map: Vec<Vec<Tail>> = Vec::new();
        if debug {
            let mut map_row: Vec<Tail> = Vec::new();
            map_row.resize(46, Tail::Empty);
            map.resize(26, map_row);
        }

        let mut input = Vec::new();

        let mut x = 0;
        let mut y = 0;

        let mut out_nr = 0;
        let mut score = 0;

        if debug {
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

        let mut paddle_pos = 0;
        let mut ball_pos = 0;

        'outer: loop {
            let interupt = self.program.start(&mut input);

            match interupt {
                Interupt::Halt => break,
                Interupt::Input => {
                    if debug {
                        // Print the map to the screen
                        for (y, row) in map.iter().enumerate() {
                            let mut row_chars = Vec::new();
                            for (x, cell) in row.iter().enumerate() {
                                match cell {
                                    Tail::Empty => row_chars.push(' '),
                                    Tail::Wall => row_chars.push('#'),
                                    Tail::Block => row_chars.push('x'),
                                    Tail::Paddle => {
                                        paddle_pos = x;
                                        row_chars.push('-')
                                    }
                                    Tail::Ball => {
                                        ball_pos = x;
                                        row_chars.push('o')
                                    }
                                };
                            }

                            _ = execute!(
                                stdout(),
                                cursor::MoveTo(0, y as u16),
                                Print(row_chars.iter().collect::<String>())
                            );
                        }
                        _ = execute!(
                            stdout(),
                            cursor::MoveTo(2, map.len() as u16 + 1),
                            Print(format!("Score: {}", score)),
                        );

                        let now = Instant::now();
                        let duration = 50;
                        if event::poll(Duration::from_millis(duration)).unwrap() {
                            if let event::Event::Key(event) = event::read().unwrap() {
                                match event.code {
                                    event::KeyCode::Esc => break 'outer,
                                    event::KeyCode::Char('q') => break 'outer,
                                    event::KeyCode::Char('c')
                                        if event.modifiers & event::KeyModifiers::CONTROL
                                            == event::KeyModifiers::CONTROL =>
                                    {
                                        break 'outer
                                    }
                                    _ => {} // ignore
                                }
                            }
                        }

                        if now.elapsed() < Duration::from_millis(duration) {
                            sleep(Duration::from_millis(duration) - now.elapsed());
                        }
                    }

                    if paddle_pos < ball_pos {
                        input.push(1);
                    } else if paddle_pos > ball_pos {
                        input.push(-1);
                    } else {
                        input.push(0);
                    }
                }
                Interupt::Output(v) => {
                    match out_nr {
                        0 => {
                            x = v;
                        }
                        1 => {
                            y = v;
                        }
                        2 => {
                            if debug {
                                match v {
                                    v if x == -1 && y == 0 => {
                                        score = v;
                                    }
                                    0 => map[y as usize][x as usize] = Tail::Empty,
                                    1 => map[y as usize][x as usize] = Tail::Wall,
                                    2 => map[y as usize][x as usize] = Tail::Block,
                                    3 => {
                                        paddle_pos = x as usize;
                                        map[y as usize][x as usize] = Tail::Paddle;
                                    }
                                    4 => {
                                        ball_pos = x as usize;
                                        map[y as usize][x as usize] = Tail::Ball;
                                    }
                                    _ => panic!("Unknown tile, v:{}, x:{}, y:{}", v, x, y),
                                };
                            } else {
                                match v {
                                    v if x == -1 && y == 0 => {
                                        score = v;
                                    }
                                    3 => {
                                        paddle_pos = x as usize;
                                    }
                                    4 => {
                                        ball_pos = x as usize;
                                    }
                                    _ => {} // ignore
                                };
                            }
                        }
                        _ => unreachable!(),
                    }

                    out_nr = (out_nr + 1) % 3;
                }
            }
        }

        if debug {
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

        for idx in 0..self
            .program
            .memory_flags
            .len()
            .min(self.program.memory.len())
        {
            let flag = &self.program.memory_flags[idx];
            let val = &self.program.memory[idx];

            let color = match flag {
                vm::Flag::Unflagged => style::Color::DarkGrey,
                vm::Flag::Inst => style::Color::Green,
                vm::Flag::Param => style::Color::Blue,
                vm::Flag::ReadWrite => style::Color::Red,
                vm::Flag::Read => style::Color::Cyan,
                vm::Flag::Write => style::Color::White,
            };

            _ = execute!(
                stdout(),
                style::SetForegroundColor(color),
                Print(format!("{}, ", val)),
                style::ResetColor,
            );
        }
        _ = execute!(stdout(), cursor::MoveLeft(2), Print("  \n"));

        score
    }
    fn reset(&mut self) {
        self.program.reset(self.source_memory.clone());
    }
}

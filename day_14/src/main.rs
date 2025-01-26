use std::collections::HashMap;
use std::fs;
use std::time::Instant;

#[derive(Debug, Clone, Default)]
struct ResourceRequirement {
    id: usize,
    nr: usize,
}

#[derive(Debug, Clone, Default)]
struct Resource {
    nr: usize,
    requirements: Vec<ResourceRequirement>,
}

#[derive(Debug, Clone, Default)]
struct Requirement {
    need: usize,
    current_value: usize,
}

struct Solver {
    intredients: Vec<Resource>,
    ingredient_name_to_id: HashMap<String, usize>,
    feul_id: usize,
    ore_id: usize,
}

const ONE_TRILLION: usize = 1_000_000_000_000;

fn main() {
    let now = Instant::now();
    let puzzle = fs::read_to_string("./puzzle.txt").unwrap();

    let mut all_requirements: Vec<Requirement> = Vec::new();
    let mut solver = Solver {
        intredients: Vec::new(),
        ingredient_name_to_id: HashMap::new(),
        feul_id: 0,
        ore_id: 0,
    };

    for _ in 0..100 {
        all_requirements.push(Requirement::default());
        solver.intredients.push(Resource::default());
    }

    for line in puzzle.lines() {
        if line.is_empty() {
            continue;
        }

        let (requirements_str, out) = line.split_once(" => ").unwrap();

        let mut requirements = Vec::new();
        for requirement in requirements_str.split(", ") {
            let (nr, name) = requirement.split_once(' ').unwrap();
            let id = solver.ingredient_id(name);

            requirements.push(ResourceRequirement {
                id,
                nr: nr.parse().unwrap(),
            });

            all_requirements[id].need += 1;
        }

        let (nr, name) = out.split_once(' ').unwrap();
        let id = solver.ingredient_id(name);

        solver.intredients[id] = Resource {
            nr: nr.parse().unwrap(),
            requirements,
        };
    }

    let p1 = solver.solve(&mut all_requirements.clone(), solver.feul_id, 1);
    println!("p1: {}", p1);

    let mut step_size = 1 << 20;
    let mut request_ore = 0;
    loop {
        let p2 = solver.solve(&mut all_requirements.clone(), solver.feul_id, request_ore);
        if p2 > ONE_TRILLION {
            request_ore -= step_size;
            step_size /= 2;

            if step_size == 0 {
                println!("p2: {}", request_ore);
                break;
            }
        }

        request_ore += step_size;
    }

    println!("Elapsed: {:.2?}", now.elapsed());
}

impl Solver {
    fn solve(
        &self,
        requirements: &mut Vec<Requirement>,
        resource_id: usize,
        amount: usize,
    ) -> usize {
        let resource = &self.intredients[resource_id];

        let mut multiplier = amount / resource.nr;
        if amount % resource.nr != 0 {
            multiplier += 1;
        }

        let mut total = 0;
        for requirement in resource.requirements.iter() {
            if requirement.id == self.ore_id {
                total += requirement.nr * multiplier;
                continue;
            }

            let mut r = requirements[requirement.id].clone();
            r.current_value += requirement.nr * multiplier;
            r.need -= 1;
            let should_solve = r.need == 0;
            let value = r.current_value;
            requirements[requirement.id] = r;

            if should_solve {
                total += self.solve(requirements, requirement.id, value);
            }
        }

        total
    }

    fn ingredient_id(&mut self, ingredient_name: &str) -> usize {
        match self.ingredient_name_to_id.get(ingredient_name) {
            Some(id) => *id,
            None => {
                let id = self.ingredient_name_to_id.len();
                self.ingredient_name_to_id
                    .insert(ingredient_name.to_string(), id);

                if ingredient_name == "ORE" {
                    self.ore_id = id;
                } else if ingredient_name == "FUEL" {
                    self.feul_id = id;
                }

                id
            }
        }
    }
}

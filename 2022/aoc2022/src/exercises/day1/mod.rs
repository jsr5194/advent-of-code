use log::info;
use std::fs;

fn get_input(filename: &str) -> Vec<Elf> {
    let contents_str = fs::read_to_string(&filename).expect("could not read file");
    let mut elves = vec![];
    let mut cur_elf = Elf::default();
    for line in contents_str.lines() {
        if line == String::default() {
            elves.push(cur_elf);
            cur_elf = Elf::default();
        } else {
            cur_elf.food.push(Food::from(
                line.parse::<usize>().expect("could not convert to usize"),
            ));
        }
    }
    elves.push(cur_elf);
    elves
}

pub fn run_part1(filename: &str) -> usize {
    let elves = get_input(&filename);
    let mut largest_cal = 0;
    for elf in elves {
        let cur_cal = elf.get_total_calories();
        if cur_cal > largest_cal {
            largest_cal = cur_cal;
        }
    }
    info!("Part 1 Answer: {:?}", largest_cal);
    largest_cal
}

pub fn run_part1_iter(filename: &str) -> usize {
    let largest_cal = get_input(&filename)
        .iter()
        .map(|elf| elf.get_total_calories_iter())
        .max()
        .unwrap();

    info!("Part 1 iter Answer: {:?}", largest_cal);
    largest_cal
}

pub fn run_part2(filename: &str) -> usize {
    let elves = get_input(&filename);
    let mut cals = vec![];
    for elf in elves {
        cals.push(elf.get_total_calories());
    }
    cals.sort();
    let largest_cal = cals[cals.len() - 1] + cals[cals.len() - 2] + cals[cals.len() - 3];
    info!("Part 2 Answer: {:?}", largest_cal);
    largest_cal
}

pub fn run_part2_iter(filename: &str) -> usize {
    let mut cals = get_input(&filename)
        .iter()
        .map(|elf| elf.get_total_calories_iter())
        .collect::<Vec<usize>>();

    cals.sort();
    let largest_cal = cals[cals.len() - 3..cals.len()].iter().sum();
    info!("Part 2 iter Answer: {:?}", largest_cal);
    largest_cal
}

#[cfg(test)]
mod tests {
    use crate::exercises::day1::run_part1;
    use crate::exercises::day1::run_part1_iter;
    use crate::exercises::day1::run_part2;
    use crate::exercises::day1::run_part2_iter;
    #[test]
    fn test() {
        assert_eq!(run_part1("./src/exercises/day1/input_test.txt"), 24000);
        assert_eq!(run_part1("./src/exercises/day1/input.txt"), 71300);
        assert_eq!(run_part1_iter("./src/exercises/day1/input.txt"), 71300);
        assert_eq!(run_part2("./src/exercises/day1/input_test.txt"), 45000);
        assert_eq!(run_part2("./src/exercises/day1/input.txt"), 209691);
        assert_eq!(run_part2_iter("./src/exercises/day1/input.txt"), 209691)
    }
}

#[derive(Debug, Default, Clone)]
struct Food {
    calories: usize,
}

impl From<usize> for Food {
    fn from(raw_calories: usize) -> Self {
        Food {
            calories: raw_calories,
        }
    }
}

#[derive(Debug, Default)]
struct Elf {
    food: Vec<Food>,
}

impl Elf {
    fn get_total_calories(&self) -> usize {
        let mut total_cal = 0;
        for item in &self.food {
            total_cal += item.calories;
        }
        total_cal
    }

    fn get_total_calories_iter(&self) -> usize {
        self.food.iter().map(|food| food.calories).sum::<usize>()
    }
}

use log::info;

pub fn process_input(filedata: &String) -> Vec<Elf> {
    let mut elves = vec![];
    let mut cur_elf = Elf::default();
    for line in filedata.lines() {
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

pub fn run_part1(filedata: &String) -> usize {
    let elves = process_input(filedata);
    let mut largest_cal = part1(&elves);
    info!("Part 1 Answer: {:?}", largest_cal);
    largest_cal
}

pub fn part1(elves: &Vec<Elf>) -> usize {
    let mut largest_cal = 0;
    for elf in elves {
        let cur_cal = elf.get_total_calories();
        if cur_cal > largest_cal {
            largest_cal = cur_cal;
        }
    }
    largest_cal
}

pub fn run_part1_iter(filedata: &String) -> usize {
    let elves = process_input(&filedata);
    let largest_cal = part1_iter(&elves);
    info!("Part 1 iter Answer: {:?}", largest_cal);
    largest_cal
}

pub fn part1_iter(elves: &Vec<Elf>) -> usize {
    elves
        .iter()
        .map(|elf| elf.get_total_calories_iter())
        .max()
        .unwrap()
}

pub fn run_part2(filedata: &String) -> usize {
    let elves = process_input(&filedata);
    let largest_cal = part2(&elves);
    info!("Part 2 Answer: {:?}", largest_cal);
    largest_cal
}

pub fn part2(elves: &Vec<Elf>) -> usize {
    let mut cals = vec![];
    for elf in elves {
        cals.push(elf.get_total_calories());
    }
    cals.sort();
    let largest_cal = cals[cals.len() - 1] + cals[cals.len() - 2] + cals[cals.len() - 3];
    largest_cal
}

pub fn run_part2_iter(filedata: &String) -> usize {
    let mut elves = process_input(&filedata);
    let largest_cal = part2_iter(&elves);
    info!("Part 2 iter Answer: {:?}", largest_cal);
    largest_cal
}

pub fn part2_iter(elves: &Vec<Elf>) -> usize {
    let mut cals = elves
        .iter()
        .map(|elf| elf.get_total_calories_iter())
        .collect::<Vec<usize>>();

    cals.sort();
    let largest_cal = cals[cals.len() - 3..cals.len()].iter().sum();
    largest_cal
}

#[cfg(test)]
mod tests {
    use crate::common::read_file;
    use crate::exercises::day1::run_part1;
    use crate::exercises::day1::run_part1_iter;
    use crate::exercises::day1::run_part2;
    use crate::exercises::day1::run_part2_iter;
    #[test]
    fn test() {
        assert_eq!(
            run_part1(&read_file("./src/exercises/day1/input_test.txt")),
            24000
        );
        assert_eq!(
            run_part1(&read_file("./src/exercises/day1/input.txt")),
            71300
        );
        assert_eq!(
            run_part1_iter(&read_file("./src/exercises/day1/input.txt")),
            71300
        );
        assert_eq!(
            run_part2(&read_file("./src/exercises/day1/input_test.txt")),
            45000
        );
        assert_eq!(
            run_part2(&read_file("./src/exercises/day1/input.txt")),
            209691
        );
        assert_eq!(
            run_part2_iter(&read_file("./src/exercises/day1/input.txt")),
            209691
        )
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
pub struct Elf {
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

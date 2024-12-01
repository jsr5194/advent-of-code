use log::info;

pub fn process_input(filedata: &String) -> Vec<Rucksack> {
    let mut rucksack_group = vec![];
    for line in filedata.lines() {
        let (left, right) = line.split_at(line.len() / 2);
        rucksack_group.push(Rucksack {
            c1: left,
            c2: right,
        });
    }
    rucksack_group
}

pub fn run_part1(filedata: &String) -> usize {
    let input = process_input(filedata);
    let result = part1(&input);
    info!("Day 3 Part 1 Answer: {:?}", result);
    result
}

pub fn part1(rucksack_list: &Vec<Rucksack>) -> usize {
    let mut priority = 0;
    for rucksack in rucksack_list {
        let mut shared_items: Vec<char> = vec![];
        for item in rucksack.c1.chars() {
            if rucksack.c2.contains(item) && !shared_items.contains(&item) {
                shared_items.push(item.clone());
                if item.is_lowercase() {
                    priority += ((item as u8) as usize) - 0x60;
                } else if item.is_uppercase() {
                    priority += ((item as u8) as usize) - 0x40 + 0x1A;
                } else {
                    panic!("item is not upper or lower case")
                }
            }
        }
    }
    priority
}

pub fn run_part2(filedata: &String) -> usize {
    let input = process_input(filedata);
    let result = part2(&input);
    info!("Day 3 Part 2 Answer: {:?}", result);
    result
}

pub fn part2(rucksack_list: &Vec<Rucksack>) -> usize {
    let mut priority = 0;
    let mut rucksack_idx = 0;
    while rucksack_idx < rucksack_list.len() - 2 {
        let mut shared_items: Vec<char> = vec![];
        for item in rucksack_list[rucksack_idx].get_contents().chars() {
            if rucksack_list[rucksack_idx + 1]
                .get_contents()
                .contains(item)
                && rucksack_list[rucksack_idx + 2]
                    .get_contents()
                    .contains(item)
                && !shared_items.contains(&item)
            {
                shared_items.push(item);
                if item.is_lowercase() {
                    priority += ((item as u8) as usize) - 0x60;
                } else if item.is_uppercase() {
                    priority += ((item as u8) as usize) - 0x40 + 0x1A;
                } else {
                    panic!("item is not upper or lower case")
                }
            }
        }
        rucksack_idx += 3;
    }

    priority
}

#[cfg(test)]
mod tests {
    use crate::common::read_file;
    use crate::exercises::day3::run_part1;
    use crate::exercises::day3::run_part2;
    #[test]
    fn test() {
        assert_eq!(
            run_part1(&read_file("./src/exercises/day3/input_test.txt")),
            157
        );
        assert_eq!(
            run_part1(&read_file("./src/exercises/day3/input.txt")),
            7872
        );
        assert_eq!(
            run_part2(&read_file("./src/exercises/day3/input_test.txt")),
            70
        );
        assert_eq!(
            run_part2(&read_file("./src/exercises/day3/input.txt")),
            2497
        );
    }
}

#[derive(Debug, Default)]
pub struct Rucksack<'a> {
    c1: &'a str,
    c2: &'a str,
}

impl<'a> Rucksack<'a> {
    fn get_contents(&self) -> String {
        format!("{}{}", self.c1, self.c2)
    }
}

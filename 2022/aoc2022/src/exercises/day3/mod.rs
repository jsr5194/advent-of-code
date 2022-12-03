use log::info;
use std::collections::HashMap;

pub fn process_input_p1(filedata: &String) -> RucksackList {
    RucksackList::from(filedata)
}

pub fn process_input_p2(filedata: &String) -> ElfGroups {
    ElfGroups::from(filedata)
}

pub fn run_part1(filedata: &String) -> u32 {
    let rucksack_list = process_input_p1(filedata);
    let priority_sum = part1(&rucksack_list);
    info!("Day 3 Part 1 Answer: {:?}", priority_sum);
    priority_sum
}

pub fn part1(rucksack_list: &RucksackList) -> u32 {
    let shared_items: Vec<SharedItems> = rucksack_list.get_individual_shared_items();
    shared_items.iter().map(|item| item.get_priority()).sum()
}

pub fn run_part2(filedata: &String) -> u32 {
    let elf_rucksack_groups = process_input_p2(filedata);
    let priority_sum = part2(&elf_rucksack_groups);
    info!("Day 3 Part 2 Answer: {:?}", priority_sum);
    priority_sum
}

pub fn part2(elf_groups: &ElfGroups) -> u32 {
    let shared_items: SharedItems = elf_groups.get_badges();
    shared_items.get_priority()
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

#[derive(Default, Debug)]
struct SharedItems {
    items: Vec<char>,
}

impl SharedItems {
    fn get_priority(&self) -> u32 {
        let mut priority = HashMap::new();
        priority.insert('a', 1);
        priority.insert('b', 2);
        priority.insert('c', 3);
        priority.insert('d', 4);
        priority.insert('e', 5);
        priority.insert('f', 6);
        priority.insert('g', 7);
        priority.insert('h', 8);
        priority.insert('i', 9);
        priority.insert('j', 10);
        priority.insert('k', 11);
        priority.insert('l', 12);
        priority.insert('m', 13);
        priority.insert('n', 14);
        priority.insert('o', 15);
        priority.insert('p', 16);
        priority.insert('q', 17);
        priority.insert('r', 18);
        priority.insert('s', 19);
        priority.insert('t', 20);
        priority.insert('u', 21);
        priority.insert('v', 22);
        priority.insert('w', 23);
        priority.insert('x', 24);
        priority.insert('y', 25);
        priority.insert('z', 26);
        priority.insert('A', 27);
        priority.insert('B', 28);
        priority.insert('C', 29);
        priority.insert('D', 30);
        priority.insert('E', 31);
        priority.insert('F', 32);
        priority.insert('G', 33);
        priority.insert('H', 34);
        priority.insert('I', 35);
        priority.insert('J', 36);
        priority.insert('K', 37);
        priority.insert('L', 38);
        priority.insert('M', 39);
        priority.insert('N', 40);
        priority.insert('O', 41);
        priority.insert('P', 42);
        priority.insert('Q', 43);
        priority.insert('R', 44);
        priority.insert('S', 45);
        priority.insert('T', 46);
        priority.insert('U', 47);
        priority.insert('V', 48);
        priority.insert('W', 49);
        priority.insert('X', 50);
        priority.insert('Y', 51);
        priority.insert('Z', 52);

        self.items
            .iter()
            .map(|item| *priority.get(item).unwrap() as u32)
            .sum()
    }
}

#[derive(Default, Debug, Clone)]
struct Compartment {
    items: Vec<char>,
}

impl From<Vec<char>> for Compartment {
    fn from(raw_compartment: Vec<char>) -> Self {
        Compartment {
            items: raw_compartment,
        }
    }
}

#[derive(Default, Debug, Clone)]
struct Rucksack {
    compartment1: Compartment,
    compartment2: Compartment,
}

impl From<&str> for Rucksack {
    fn from(cur_rucksack: &str) -> Self {
        let mut raw_items = cur_rucksack.chars().collect::<Vec<char>>();

        let raw_c1: Vec<char> = raw_items[0..raw_items.len() / 2].to_vec();
        let raw_c2: Vec<char> = raw_items[raw_items.len() / 2..raw_items.len()].to_vec();
        Rucksack {
            compartment1: Compartment::from(raw_c1),
            compartment2: Compartment::from(raw_c2),
        }
    }
}

impl Rucksack {
    fn get_shared_items(&self) -> SharedItems {
        let mut shared_items = SharedItems::default();
        for item in &self.compartment1.items {
            if self.compartment2.items.contains(&item) {
                if !shared_items.items.contains(&item) {
                    shared_items.items.push(item.clone());
                }
            }
        }
        shared_items
    }

    fn combine(&self) -> Vec<char> {
        let mut rucksack_vec: Vec<char> = vec![];
        rucksack_vec.append(
            &mut self
                .compartment1
                .items
                .iter()
                .map(|item| *item)
                .collect::<Vec<char>>(),
        );
        rucksack_vec.append(
            &mut self
                .compartment2
                .items
                .iter()
                .map(|item| *item)
                .collect::<Vec<char>>(),
        );
        rucksack_vec
    }
}

#[derive(Default, Debug, Clone)]
pub struct RucksackList {
    rucksacks: Vec<Rucksack>,
}

impl From<&String> for RucksackList {
    fn from(input: &String) -> Self {
        RucksackList {
            rucksacks: input
                .lines()
                .map(|cur_rucksack| Rucksack::from(cur_rucksack))
                .collect::<Vec<Rucksack>>(),
        }
    }
}

impl RucksackList {
    fn get_individual_shared_items(&self) -> Vec<SharedItems> {
        let mut shared_items: Vec<SharedItems> = vec![];
        for rucksack in &self.rucksacks {
            shared_items.push(rucksack.get_shared_items());
        }
        shared_items
    }

    fn get_cross_shared_items(&self) -> char {
        let mut shared_items: char = '*';
        for item in self.rucksacks[0].combine() {
            let mut shared_items_found = true;
            for rucksack in self.rucksacks[1..self.rucksacks.len()].iter() {
                if !rucksack.combine().contains(&item) {
                    shared_items_found = false;
                    break;
                }
            }
            if shared_items_found {
                shared_items = item;
            }
        }

        if shared_items == '*' {
            panic!("a valid shared_items was not found");
        }

        shared_items
    }
}

#[derive(Default, Debug)]
pub struct ElfGroups {
    groups: Vec<RucksackList>,
}

impl From<&String> for ElfGroups {
    fn from(input: &String) -> Self {
        let mut groups = ElfGroups::default();
        let group_mod = 3;
        let mut idx = 0;
        let mut cur_rucksack_list = RucksackList::default();
        for line in input.lines() {
            cur_rucksack_list.rucksacks.push(Rucksack::from(line));
            if idx % group_mod == group_mod - 1 {
                groups.groups.push(cur_rucksack_list.clone());
                cur_rucksack_list.rucksacks.clear();
            }
            idx += 1;
        }
        groups
    }
}

impl ElfGroups {
    fn get_badges(&self) -> SharedItems {
        SharedItems {
            items: self
                .groups
                .iter()
                .map(|rucksack_list| rucksack_list.get_cross_shared_items())
                .collect::<Vec<char>>(),
        }
    }
}

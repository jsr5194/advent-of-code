use log::info;
use std::collections::{HashMap, VecDeque};

use crate::handheld::Handheld;

pub fn process_input(filedata: &String) -> Handheld {
    let mut device = Handheld::default();
    device.build_fs_from_stdout(filedata);
    device
}

pub fn run_part1(filedata: &String) -> usize {
    let mut device = process_input(filedata);
    let result = part1(&device);
    info!("Day 7 Part 1 Answer: {:?}", result);
    result
}

pub fn part1(device: &Handheld) -> usize {
    let dir_sizes: HashMap<String, usize> = device.calculate_fs_size();

    let mut result = 0;
    for (dir, size) in &dir_sizes {
        if size <= &100000 {
            result += size;
        }
    }
    result
}

pub fn run_part2(filedata: &String) -> usize {
    let device = process_input(filedata);
    let result = part2(&device);
    info!("Day 7 Part 2 Answer: {:?}", result);
    result
}

pub fn part2(device: &Handheld) -> usize {
    let mut dir_sizes: HashMap<String, usize> = device.calculate_fs_size();

    let needed_space = 30000000;
    let disk_size = 70000000;
    let free_space = disk_size - dir_sizes.get(&"/".to_string()).unwrap();
    let mut smallest_single_option: usize = disk_size;
    for (dir, size) in &dir_sizes {
        if free_space + size >= needed_space {
            if size < &smallest_single_option {
                smallest_single_option = *size;
            }
        }
    }
    smallest_single_option
}

#[cfg(test)]
mod tests {
    use crate::common::read_file;
    use crate::exercises::day7::run_part1;
    use crate::exercises::day7::run_part2;
    #[test]
    fn test() {
        assert_eq!(
            run_part1(&read_file("./src/exercises/day7/input_test.txt")),
            95437
        );
        assert_eq!(
            run_part1(&read_file("./src/exercises/day7/input.txt")),
            2061777
        );
        assert_eq!(
            run_part2(&read_file("./src/exercises/day7/input_test.txt")),
            24933642
        );
    }
}

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
    let result = part1(&mut device);
    info!("Day 7 Part 1 Answer: {:?}", result);
    info!("invalid guess (too low): {:?}", 1595977);
    result
}

pub fn part1(device: &mut Handheld) -> usize {
    let mut dir_sizes: HashMap<String, usize> = HashMap::new();
    for file in device.get_fs() {
        let dir = file.get_dir();
        if dir == &"/".to_string() {
            println!("in root");
            let mut cur_dir = dir.to_string();
            if dir_sizes.contains_key(&cur_dir) {
                let mut cur_dirsize = dir_sizes.get_mut(&cur_dir).unwrap();
                let mut new_dirsize = *cur_dirsize + file.get_filesize();
                dir_sizes.insert(cur_dir.to_string(), new_dirsize);
            } else {
                dir_sizes.insert(cur_dir.to_string(), *file.get_filesize());
            }
        } else {
            for raw_dir in file.get_dir().split("/") {
                let mut cur_dir = raw_dir.to_string();
                if cur_dir == "".to_string() {
                    cur_dir = "/".to_string()
                }
                if dir_sizes.contains_key(&cur_dir) {
                    let mut cur_dirsize = dir_sizes.get_mut(&cur_dir).unwrap();
                    let mut new_dirsize = *cur_dirsize + file.get_filesize();
                    dir_sizes.insert(cur_dir.to_string(), new_dirsize);
                } else {
                    dir_sizes.insert(cur_dir.to_string(), *file.get_filesize());
                }
            }
            println!("");
        }
    }

    for file in device.get_fs() {
        println!("{:?}", file);
    }

    let mut result = 0;
    for (dir, size) in &dir_sizes {
        println!("{:?} {:?}", dir, size);
        if size <= &100000 {
            result += size;
        }
    }

    result
}

pub fn run_part2(filedata: &String) -> usize {
    //let device = process_input(filedata);
    //let result = part2(&device);
    //info!("Day 7 Part 2 Answer: {:?}", result);
    //result
    0
}

pub fn part2(device: &Handheld) -> usize {
    0
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
    }
}

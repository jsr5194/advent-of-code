use log::info;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn process_input(filedata: &String) -> Vec<SectionAssignmentPair> {
    filedata
        .lines()
        .map(|line| SectionAssignmentPair::from(line))
        .collect::<Vec<SectionAssignmentPair>>()
}

pub fn run_part1(filedata: &String) -> usize {
    let section_assignment_pairs = process_input(filedata);
    let result = part1(&section_assignment_pairs);
    info!("Day 4 Part 1 Answer: {:?}", result);
    result
}

pub fn part1(section_assignment_pairs: &Vec<SectionAssignmentPair>) -> usize {
    let mut result = 0;
    for pair in section_assignment_pairs {
        if pair.assignment1.is_subset(&pair.assignment2)
            || pair.assignment2.is_subset(&pair.assignment1)
        {
            result += 1;
        }
    }

    result
}

pub fn run_part2(filedata: &String) -> usize {
    let section_assignment_pairs = process_input(filedata);
    let result = part2(&section_assignment_pairs);
    info!("Day 4 Part 2 Answer: {:?}", result);
    result
}

pub fn part2(section_assignment_pairs: &Vec<SectionAssignmentPair>) -> usize {
    let mut result = 0;
    for pair in section_assignment_pairs {
        if pair.assignment1.intersection(&pair.assignment2).count() > 0 {
            result += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::common::read_file;
    use crate::exercises::day4::run_part1;
    use crate::exercises::day4::run_part2;
    #[test]
    fn test() {
        assert_eq!(
            run_part1(&read_file("./src/exercises/day4/input_test.txt")),
            2
        );
        assert_eq!(run_part1(&read_file("./src/exercises/day4/input.txt")), 456);
        assert_eq!(
            run_part2(&read_file("./src/exercises/day4/input_test.txt")),
            4
        );
        assert_eq!(run_part2(&read_file("./src/exercises/day4/input.txt")), 808);
    }
}

#[derive(Debug, Default)]
pub struct SectionAssignmentPair {
    assignment1: HashSet<usize>,
    assignment2: HashSet<usize>,
}

impl From<&str> for SectionAssignmentPair {
    fn from(raw_pair: &str) -> Self {
        let mut raw_pair_split = raw_pair.split(',');
        let assignment1_split = raw_pair_split
            .next()
            .unwrap()
            .split('-')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let assignment2_split = raw_pair_split
            .next()
            .unwrap()
            .split('-')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        SectionAssignmentPair {
            assignment1: HashSet::from_iter(assignment1_split[0]..=assignment1_split[1]),
            assignment2: HashSet::from_iter(assignment2_split[0]..=assignment2_split[1]),
        }
    }
}

//impl From<&str> for SectionAssignmentPair {
//    fn from(raw_pair: &str) -> Self {
//        let mut raw_pair_split = raw_pair.split(',');
//        let assignment1_split = raw_pair_split
//            .next()
//            .unwrap()
//            .split('-')
//            .map(|x| x.parse::<usize>().unwrap())
//            .collect::<Vec<usize>>();
//        let assignment2_split = raw_pair_split
//            .next()
//            .unwrap()
//            .split('-')
//            .map(|x| x.parse::<usize>().unwrap())
//            .collect::<Vec<usize>>();
//
//        let mut assignment1: HashSet<usize> = HashSet::new();
//        for i in assignment1_split[0]..=assignment1_split[1] {
//            assignment1.insert(i);
//        }
//
//        let mut assignment2: HashSet<usize> = HashSet::new();
//        for i in assignment2_split[0]..=assignment2_split[1] {
//            assignment2.insert(i);
//        }
//
//        SectionAssignmentPair {
//            assignment1: assignment1,
//            assignment2: assignment2,
//        }
//    }
//}

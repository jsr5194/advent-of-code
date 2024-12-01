use log::info;

use crate::handheld::Handheld;

pub fn process_input(filedata: &String) -> Handheld {
    let mut device = Handheld::default();
    device.ingest_packet(filedata);
    device
}

pub fn run_part1(filedata: &String) -> usize {
    let device = process_input(filedata);
    let result = part1(&device);
    info!("Day 6 Part 1 Answer: {:?}", result);
    result
}

pub fn part1(device: &Handheld) -> usize {
    device.find_start_of_pkt()
}

pub fn run_part2(filedata: &String) -> usize {
    let device = process_input(filedata);
    let result = part2(&device);
    info!("Day 6 Part 2 Answer: {:?}", result);
    result
}

pub fn part2(device: &Handheld) -> usize {
    device.find_start_of_msg()
}

#[cfg(test)]
mod tests {
    use crate::common::read_file;
    use crate::exercises::day6::run_part1;
    use crate::exercises::day6::run_part2;
    #[test]
    fn test() {
        assert_eq!(
            run_part1(&read_file("./src/exercises/day6/input_test_part1_1.txt")),
            7
        );
        assert_eq!(
            run_part1(&read_file("./src/exercises/day6/input_test_part1_2.txt")),
            5
        );
        assert_eq!(
            run_part1(&read_file("./src/exercises/day6/input_test_part1_3.txt")),
            6
        );
        assert_eq!(
            run_part1(&read_file("./src/exercises/day6/input_test_part1_4.txt")),
            10
        );
        assert_eq!(
            run_part1(&read_file("./src/exercises/day6/input_test_part1_5.txt")),
            11
        );
        assert_eq!(
            run_part1(&read_file("./src/exercises/day6/input.txt")),
            1794
        );
        assert_eq!(
            run_part2(&read_file("./src/exercises/day6/input_test_part2_1.txt")),
            19
        );
        assert_eq!(
            run_part2(&read_file("./src/exercises/day6/input_test_part2_2.txt")),
            23
        );
        assert_eq!(
            run_part2(&read_file("./src/exercises/day6/input_test_part2_3.txt")),
            23
        );
        assert_eq!(
            run_part2(&read_file("./src/exercises/day6/input_test_part2_4.txt")),
            29
        );
        assert_eq!(
            run_part2(&read_file("./src/exercises/day6/input_test_part2_5.txt")),
            26
        );
        assert_eq!(
            run_part2(&read_file("./src/exercises/day6/input.txt")),
            2851
        );
    }
}

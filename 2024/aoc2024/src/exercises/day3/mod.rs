use crate::toboggan::Instruction;
use crate::toboggan::Toboggan;
use log::info;

pub fn process_input(filedata: &String) -> Toboggan {
    Toboggan::from(filedata)
}

pub fn run_part1(filedata: &String) -> u32 {
    let mut toboggan = process_input(filedata);
    toboggan.ignore_execute_flag = true;
    toboggan.run();
    let result = toboggan.results.iter().sum::<u32>();
    println!("Day 3 Part 1 Result: {}", result);
    result
}

pub fn run_part2(filedata: &String) -> u32 {
    let mut toboggan = process_input(filedata);
    toboggan.run();
    let result = toboggan.results.iter().sum::<u32>();
    println!("Day 3 Part 2 Result: {}", result);
    result
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
            161
        );
        assert_eq!(
            run_part1(&read_file("./src/exercises/day3/input.txt")),
            187825547
        );
        assert_eq!(
            run_part2(&read_file("./src/exercises/day3/input_test_2.txt")),
            48
        );
        assert_eq!(
            run_part2(&read_file("./src/exercises/day3/input.txt")),
            85508223
        )
    }
}

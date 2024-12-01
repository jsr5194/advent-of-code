use log::info;

use crate::handheld::Handheld;

pub fn process_input(filedata: &String) -> Handheld {
    let mut handheld = Handheld::default();
    handheld.load_program(filedata);
    handheld
}

pub fn run_part1(filedata: &String) -> isize {
    let mut handheld = process_input(filedata);
    let result = part1(&mut handheld);
    info!("Day 9 Part 1 Answer: {}", result);
    result
}

pub fn part1(handheld: &mut Handheld) -> isize {
    let mut signal_strength_sum: isize = 0;

    //during cycle 20
    handheld.debug_program(20);
    signal_strength_sum += handheld.get_signal_strength();

    //during cycle 60
    handheld.debug_program(60);
    signal_strength_sum += handheld.get_signal_strength();

    //during cycle 100
    handheld.debug_program(100);
    signal_strength_sum += handheld.get_signal_strength();

    //during cycle 140
    handheld.debug_program(140);
    signal_strength_sum += handheld.get_signal_strength();

    //during cycle 180
    handheld.debug_program(180);
    signal_strength_sum += handheld.get_signal_strength();

    //during cycle 220
    handheld.debug_program(220);
    signal_strength_sum += handheld.get_signal_strength();

    signal_strength_sum
}

pub fn run_part2(filedata: &String) -> usize {
    let mut handheld = process_input(filedata);
    let result = part2(&mut handheld);
    info!("Day 9 Part 2 Answer: {}", result);
    result
}

pub fn part2(handheld: &mut Handheld) -> usize {
    handheld.print_display();
    handheld.run_program();
    handheld.print_display();
    0
}

#[cfg(test)]
mod tests {
    use crate::common::read_file;
    use crate::exercises::day10::run_part1;
    use crate::exercises::day10::run_part2;
    #[test]
    fn test() {
        assert_eq!(
            run_part1(&read_file("./src/exercises/day10/input_test_2.txt")),
            13140
        );
        assert_eq!(
            run_part1(&read_file("./src/exercises/day10/input.txt")),
            14820
        );
    }
}

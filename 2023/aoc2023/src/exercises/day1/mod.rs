use log::info;

pub fn process_input(filedata: &String) -> Vec<String> {
    let mut cd: Vec<String> = vec![];
    for line in filedata.lines() {
        cd.push(line.to_string());
    }
    cd
}

pub fn run_part1(filedata: &String) -> u32 {
    let calibration_doc = process_input(filedata);
    let mut calibration_values: Vec<u32> = vec![];
    for line in calibration_doc {
        let mut digits: Vec<char> = vec![];
        for c in line.chars() {
            if c.is_digit(10) {
                digits.push(c);
            }
        }
        calibration_values.push(
            format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
                .parse()
                .unwrap(),
        );
    }

    let mut result = 0;
    for value in calibration_values {
        result += value;
    }

    println!("Part 1 Result: {:?}", result);

    result
}

pub fn run_part2(filedata: &String) -> u32 {
    let calibration_doc = process_input(filedata);
    let mut calibration_values: Vec<u32> = vec![];
    let mut round = 0;
    for line in calibration_doc {
        let mut string_digit = String::default();
        let mut digits: Vec<char> = vec![];
        for c in line.chars() {
            if c.is_digit(10) {
                string_digit = String::default();
                digits.push(c);
            } else {
                string_digit += c.to_string().as_str();

                if string_digit.contains("one") {
                    digits.push('1');
                    string_digit.remove(0);
                } else if string_digit.contains("two") {
                    digits.push('2');
                    string_digit.remove(0);
                } else if string_digit.contains("three") {
                    digits.push('3');
                    string_digit.remove(0);
                } else if string_digit.contains("four") {
                    digits.push('4');
                    string_digit.remove(0);
                } else if string_digit.contains("five") {
                    digits.push('5');
                    string_digit.remove(0);
                } else if string_digit.contains("six") {
                    digits.push('6');
                    string_digit.remove(0);
                } else if string_digit.contains("seven") {
                    digits.push('7');
                    string_digit.remove(0);
                } else if string_digit.contains("eight") {
                    digits.push('8');
                    string_digit.remove(0);
                } else if string_digit.contains("nine") {
                    digits.push('9');
                    string_digit.remove(0);
                }
            }
            println!("{:?}", string_digit);
        }
        println!("{:?}", digits);

        calibration_values.push(
            format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
                .parse()
                .unwrap(),
        );

        round += 1;
    }

    let mut result = 0;
    for value in &calibration_values {
        result += value;
    }

    println!("{:?}", &calibration_values);

    println!("Part 2 Result: {:?}", result);

    // not: 53998 - too low
    // not: 51314 - too low
    // not: 53945 - too low

    result
}

#[cfg(test)]
mod tests {
    use crate::common::read_file;
    use crate::exercises::day1::run_part1;
    use crate::exercises::day1::run_part2;
    #[test]
    fn test() {
        assert_eq!(
            run_part1(&read_file("./src/exercises/day1/input_test.txt")),
            142
        );
        assert_eq!(
            run_part1(&read_file("./src/exercises/day1/input.txt")),
            54632
        );
        assert_eq!(
            run_part2(&read_file("./src/exercises/day1/input_test_2.txt")),
            281
        );
        //        assert_eq!(
        //            run_part2(&read_file("./src/exercises/day1/input.txt")),
        //            209691
        //        );
        //        assert_eq!(
        //            run_part2_iter(&read_file("./src/exercises/day1/input.txt")),
        //            209691
        //        )
    }
}

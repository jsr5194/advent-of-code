use log::info;
use std::fs;

fn get_input(filename: &str) {
    let contents_str = fs::read_to_string(&filename).expect("could not read file");
}

pub fn run_part1(filename: &str) {
    info!("running part 1");
}

pub fn run_part2(filename: &str) {
    info!("running part 2");
}

#[cfg(test)]
mod tests {
    use crate::exercises::day2::run_part1;
    use crate::exercises::day2::run_part2;
    #[test]
    fn test() {}
}

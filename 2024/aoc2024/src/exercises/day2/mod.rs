use log::info;
use std::str::FromStr;

#[derive(Debug, Default)]
struct Level {
    value: usize,
    is_bad: bool,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseLevelError;

impl FromStr for Level {
    type Err = ParseLevelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Level {
            value: s.parse::<usize>().unwrap(),
            is_bad: false,
        })
    }
}

#[derive(Debug, Default)]
struct Report {
    levels: Vec<Level>,
    is_decreasing: bool,
    is_increasing: bool,
}

impl Report {
    fn get_bad_level_count(&self) -> u32 {
        let mut bad_level_count = 0;
        for level in &self.levels {
            if level.is_bad {
                bad_level_count += 1;
            }
        }
        bad_level_count
    }

    fn find_bad_levels(&mut self) {
        let mut max = &self.levels.len() - 1;
        let mut first_run = true;
        let mut is_gradual = false;
        let mut contains_lateral = false;
        for idx in 0..max {
            if idx + 1 > max {
                break;
            }

            // check if the levels are increasing
            if &self.levels[idx].value < &self.levels[idx + 1].value {
                if self.is_decreasing {
                    self.levels[idx].is_bad = true;
                } else {
                    self.is_increasing = true;
                    // check delta
                    let delta = &self.levels[idx + 1].value - &self.levels[idx].value;
                    if delta >= 1 && delta <= 3 {
                        if is_gradual || first_run {
                            is_gradual = true;
                        }
                    } else {
                        self.levels[idx].is_bad = true;
                    }
                }
            }
            // check if the levels are decreasing
            else if &self.levels[idx].value > &self.levels[idx + 1].value {
                if self.is_increasing {
                    self.levels[idx].is_bad = true;
                } else {
                    self.is_decreasing = true;
                    // check delta
                    let delta = &self.levels[idx].value - &self.levels[idx + 1].value;
                    if delta >= 1 && delta <= 3 {
                        if is_gradual || first_run {
                            is_gradual = true;
                        }
                    } else {
                        self.levels[idx].is_bad = true;
                    }
                }
            }
            // check for lateral movement
            else if &self.levels[idx].value == &self.levels[idx + 1].value {
                self.levels[idx].is_bad = true;
            }

            if first_run {
                first_run = false;
            }
        }
    }

    fn is_repairable(&mut self) -> bool {
        // when there is more than one bad level there is no reason to attempt a repair
        self.find_bad_levels();
        //if self.get_bad_level_count() > 1 {
        if false {
            return false;
        } else {
            let mut max = &self.levels.len() - 1;
            let mut first_run = true;
            let mut is_gradual = false;
            let mut contains_lateral = false;

            // check middle levels
            for idx in 1..max {
                if idx + 1 > max {
                    break;
                }

                // only proceed when the current level is bad
                if !&self.levels[idx].is_bad {
                    continue;
                }

                // check if the levels are increasing
                if &self.levels[idx - 1].value < &self.levels[idx + 1].value {
                    if self.is_decreasing && !first_run {
                        return false;
                    } else {
                        self.is_increasing = true;
                        // check delta
                        if is_gradual || first_run {
                            let delta = &self.levels[idx + 1].value - &self.levels[idx - 1].value;
                            if delta >= 1 && delta <= 3 {
                                is_gradual = true;
                            } else {
                                return false;
                            }
                        }
                    }
                }
                // check if the levels are decreasing
                else if &self.levels[idx - 1].value > &self.levels[idx + 1].value {
                    if self.is_increasing && !first_run {
                        return false;
                    } else {
                        self.is_decreasing = true;
                        // check delta
                        if is_gradual || first_run {
                            let delta = &self.levels[idx - 1].value - &self.levels[idx + 1].value;
                            if delta >= 1 && delta <= 3 {
                                is_gradual = true;
                            } else {
                                return false;
                            }
                        }
                    }
                }
                // check for lateral movement
                else if &self.levels[idx - 1].value == &self.levels[idx + 1].value {
                    return false;
                }

                if first_run {
                    first_run = false;
                }
            }
        }
        return true;
    }
}

#[derive(Debug, Default)]
struct ReportList {
    reports: Vec<Report>,
}

impl ReportList {
    fn get_total_report_count(&self) -> u32 {
        self.reports.len() as u32
    }

    fn get_repairable_report_count(&mut self) -> u32 {
        let mut repairable_report_count = 0;
        for report in &mut self.reports {
            if report.is_repairable() {
                repairable_report_count += 1;
            } else {
                println!("{:?}", report);
            }
        }
        repairable_report_count
    }

    fn get_bad_report_count(&self) -> u32 {
        let mut bad_report_count = 0;
        for report in &self.reports {
            if report.get_bad_level_count() != 0 {
                bad_report_count += 1;
            }
        }
        bad_report_count
    }
}

pub fn process_input(filedata: &String) -> ReportList {
    let mut report_list = ReportList::default();
    for line in filedata.lines() {
        report_list.reports.push(Report {
            levels: line
                .split(" ")
                .map(|x| x.parse::<Level>().unwrap())
                .collect(),
            is_decreasing: false,
            is_increasing: false,
        });
    }

    report_list
}

pub fn run_part1(filedata: &String) -> u32 {
    let mut report_list = process_input(filedata);
    for report in &mut report_list.reports {
        report.find_bad_levels();
    }

    let good_reports = report_list.get_total_report_count() - report_list.get_bad_report_count();
    let result = good_reports;
    println!("Part 1 Result: {:?}", result);
    result
}

pub fn run_part2(filedata: &String) -> u32 {
    let mut report_list = process_input(filedata);
    let repairable_reports = report_list.get_repairable_report_count();
    let result = repairable_reports;
    println!("Part 2 Result: {:?}", result);

    // 748 == too high
    // 742 == too high
    // 678 == too low
    // 711 == not the right answer, but direction not given
    result
}

#[cfg(test)]
mod tests {
    use crate::common::read_file;
    use crate::exercises::day2::run_part1;
    use crate::exercises::day2::run_part2;
    #[test]
    fn test() {
        assert_eq!(
            run_part1(&read_file("./src/exercises/day2/input_test.txt")),
            2
        );
        assert_eq!(run_part1(&read_file("./src/exercises/day2/input.txt")), 663);
        assert_eq!(
            run_part2(&read_file("./src/exercises/day2/input_test.txt")),
            4
        );
        //        assert_eq!(
        //            run_part2(&read_file("./src/exercises/day2/input.txt")),
        //            24643097
        //        )
    }
}

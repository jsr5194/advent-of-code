use log::info;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
enum Validity {
    Untested,
    Valid,
    Invalid,
    Repaired,
}

impl Default for Validity {
    fn default() -> Self {
        Validity::Untested
    }
}

#[derive(Debug, Default)]
struct OrderingRule {
    page1: Page,
    page2: Page,
}

impl From<&str> for OrderingRule {
    fn from(raw_data: &str) -> Self {
        let split_data: Vec<Page> = raw_data
            .split("|")
            .map(|x| x.parse::<Page>().unwrap())
            .collect::<Vec<Page>>();
        if split_data.len() != 2 {
            panic!("passed ordering rule does not conform to #|#");
        }
        OrderingRule {
            page1: split_data[0].clone(),
            page2: split_data[1].clone(),
        }
    }
}

impl OrderingRule {
    fn contains_page1(&self, cur_page: &Page) -> bool {
        if self.page1.number == cur_page.number {
            return true;
        }
        false
    }

    fn contains_page2(&self, cur_page: &Page) -> bool {
        if self.page2.number == cur_page.number {
            return true;
        }
        false
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
struct Page {
    number: u32,
    is_damaged: bool,
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePageError;

impl FromStr for Page {
    type Err = ParsePageError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Page {
            number: s.parse::<u32>().unwrap(),
            is_damaged: false,
        })
    }
}

#[derive(Debug, Default)]
struct Update {
    pages: Vec<Page>,
    original_pages: Vec<Page>,
    is_valid: Validity,
}

impl From<&str> for Update {
    fn from(raw_data: &str) -> Self {
        Update {
            pages: raw_data
                .split(",")
                .map(|x| x.parse::<Page>().unwrap())
                .collect::<Vec<Page>>(),
            original_pages: raw_data
                .split(",")
                .map(|x| x.parse::<Page>().unwrap())
                .collect::<Vec<Page>>(),
            is_valid: Validity::Untested,
        }
    }
}

impl fmt::Display for Update {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut msg = String::default();
        for p in &self.pages {
            msg = format!("{}{} ", msg, p.number);
        }
        write!(f, "{}", msg)
    }
}

impl Update {
    fn get_middle_value(&self) -> u32 {
        //Need to filter based on validation type
        match self.is_valid {
            Validity::Valid => {
                if self.pages.len() % 2 == 0 {
                    panic!("Even number of page numbers detected");
                }
                let mid_idx = self.pages.len() / 2;
                self.pages[mid_idx].number
            }
            _ => {
                // all other cases come back 0
                0
            }
        }
    }

    fn get_repaired_middle_value(&self) -> u32 {
        //Need to filter based on validation type
        match self.is_valid {
            Validity::Repaired => {
                if self.pages.len() % 2 == 0 {
                    panic!("Even number of page numbers detected");
                }
                let mid_idx = self.pages.len() / 2;
                self.pages[mid_idx].number
            }
            _ => {
                // all other cases come back 0
                0
            }
        }
    }
}

#[derive(Debug, Default)]
struct Printer {
    ordering_rules: Vec<OrderingRule>,
    updates: Vec<Update>,
}

impl Printer {
    fn contains_damaged_pages(&self) -> bool {
        for update in &self.updates {
            for page in &update.pages {
                if page.is_damaged {
                    return true;
                }
            }
        }
        false
    }

    fn analyze_updates(&mut self) {
        for update in &mut self.updates {
            // default to invalid
            let mut passed_rules = 0;

            for rule in &self.ordering_rules {
                // only check for validity when both pages exist in the current update
                if update.pages.contains(&rule.page1) && update.pages.contains(&rule.page2) {
                    let mut rule_page2_found = false;
                    for i in 0..update.pages.len() {
                        if update.pages[i] == rule.page1 {
                            if rule_page2_found {
                                update.pages[i].is_damaged = true;
                            } else {
                                for j in i..update.pages.len() {
                                    if update.pages[j] == rule.page2 {
                                        passed_rules += 1;
                                    }
                                }
                            }
                        } else if update.pages[i] == rule.page2 {
                            rule_page2_found = true;
                        }
                    }
                } else {
                    // rule does not apply so we are counting it as passed
                    passed_rules += 1;
                }
            }
            if passed_rules == self.ordering_rules.len() {
                match update.is_valid {
                    Validity::Repaired => {
                        update.is_valid = Validity::Repaired;
                    }
                    _ => {
                        update.is_valid = Validity::Valid;
                    }
                }
            } else {
                update.is_valid = Validity::Invalid;
            }
        }
    }

    fn get_valid_middle_values_sum(&self) -> u32 {
        let mut middle_sum: u32 = 0;
        for update in &self.updates {
            match update.is_valid {
                Validity::Valid => {
                    middle_sum += update.get_middle_value();
                }
                Validity::Repaired => {
                    // don't do anything
                }
                Validity::Invalid => {
                    // don't do anything
                }
                Validity::Untested => {
                    panic!("why is there still an untested middle value");
                }
            }
        }
        middle_sum
    }

    fn get_repaired_middle_values_sum(&self) -> u32 {
        let mut middle_sum: u32 = 0;
        for update in &self.updates {
            match update.is_valid {
                Validity::Valid => {
                    // don't do anything
                }
                Validity::Repaired => {
                    middle_sum += update.get_repaired_middle_value();
                }
                Validity::Invalid => {
                    // don't do anything
                }
                Validity::Untested => {
                    panic!("why is there still an untested middle value");
                }
            }
        }
        middle_sum
    }

    fn repair_updates(&mut self) {
        for update in &mut self.updates {
            match update.is_valid {
                Validity::Invalid => {
                    for page_idx in 0..update.pages.len() {
                        //for page in &mut update.pages {
                        if update.pages[page_idx].is_damaged {
                            for rule in &self.ordering_rules {
                                // if the current page isn't pertinet don't bother
                                if rule.contains_page1(&update.pages[page_idx]) {
                                    if update.pages.contains(&rule.page2) {
                                        let delete_idx = update
                                            .pages
                                            .iter()
                                            .position(|n| n == &update.pages[page_idx])
                                            .unwrap();
                                        let insert_idx = update
                                            .pages
                                            .iter()
                                            .position(|n| n == &rule.page2)
                                            .unwrap();

                                        update.pages.insert(insert_idx, rule.page1.clone());
                                        update.pages[insert_idx].is_damaged = false;
                                        update.pages[insert_idx + 1].is_damaged = false;
                                    }
                                }
                            }
                        }
                    }

                    update.is_valid = Validity::Repaired;
                }
                _ => {
                    // do nothing as any other state doesn't need repaired
                }
            }
        }

        self.analyze_updates();
        if self.contains_damaged_pages() {
            self.repair_updates();
        }
        println!("made it past");

        //        for update in &mut self.updates {
        //            let mut tmp_repaired_pages: Vec<Page> = update.pages.clone();
        //            match update.is_valid {
        //                Validity::Invalid => {
        //                    for rule in &self.ordering_rules {
        //                        // when one or both of the affected pages are not in the current update the rule doesn't apply
        //                        if tmp_repaired_pages.contains(&rule.page1)
        //                            && tmp_repaired_pages.contains(&rule.page2)
        //                        {
        //                            // go page by page determing placement by the current rule
        //                            let mut first_run = true;
        //                            for page in &update.pages {
        //                                if first_run {
        //                                    tmp_repaired_pages.push(page.clone());
        //                                    first_run = false;
        //                                } else {
        //                                    if page == &rule.page2 {
        //                                        if tmp_repaired_pages.contains(&rule.page1) {
        //                                            let insert_idx = tmp_repaired_pages
        //                                                .iter()
        //                                                .position(|n| n == &rule.page1)
        //                                                .unwrap();
        //                                            tmp_repaired_pages
        //                                                .insert(insert_idx, rule.page1.clone());
        //                                        } else {
        //                                            tmp_repaired_pages.push(page.clone());
        //                                        }
        //                                    } else {
        //                                        tmp_repaired_pages.push(page.clone());
        //                                    }
        //                                }
        //                            }
        //                        }
        //                    }
        //
        //                    // prune out all prior duplicates
        //                    // absolutely not the most optimal way but im getting sick of this
        //                    for i in 0..tmp_repaired_pages.len() - 1 {
        //                        // if the current page is affected by a rule, look ahead and grab the last one
        //                        let mut dup_exists = false;
        //
        //                        let mut rules_apply = false;
        //                        for rule in &self.ordering_rules {
        //                            if rule.contains_page1(&tmp_repaired_pages[i]) {
        //                                if tmp_repaired_pages.contains(&rule.page2) {
        //                                    rules_apply = true;
        //                                }
        //                            } else if rule.contains_page2(&tmp_repaired_pages[i]) {
        //                                if tmp_repaired_pages.contains(&rule.page1) {
        //                                    rules_apply = true;
        //                                }
        //                            }
        //                        }
        //                        if rules_apply {
        //                            for j in i..tmp_repaired_pages.len() {
        //                                if tmp_repaired_pages[i] == tmp_repaired_pages[j] {
        //                                    dup_exists = true;
        //                                    break;
        //                                }
        //                            }
        //                        }
        //                        if !dup_exists {
        //                            if !update.repaired_pages.contains(&tmp_repaired_pages[i]) {
        //                                update.repaired_pages.push(tmp_repaired_pages[i].clone());
        //                            }
        //                        }
        //                    }
        //
        //                    update.is_valid = Validity::Repaired;
        //                }
        //                _ => {
        //                    // do nothing as any other state doesn't need repaired
        //                }
        //            }
        //        }
    }
}

pub fn process_input(filedata: &String) -> Printer {
    let mut printer = Printer::default();

    let mut rule_processing_finished = false;
    for line in filedata.lines() {
        if line == String::default() {
            rule_processing_finished = true;
            continue;
        }

        if rule_processing_finished {
            printer.updates.push(Update::from(line));
        } else {
            printer.ordering_rules.push(OrderingRule::from(line));
        }
    }

    printer
}

pub fn run_part1(filedata: &String) -> u32 {
    let mut printer = process_input(filedata);
    printer.analyze_updates();
    let result = printer.get_valid_middle_values_sum();
    println!("Day 5 Part 1 Result: {}", result);
    result
}

pub fn run_part2(filedata: &String) -> u32 {
    let mut printer = process_input(filedata);
    printer.repair_updates();
    println!("{:?}", printer.updates);
    let result = printer.get_repaired_middle_values_sum();
    println!("Day 5 Part 2 Result: {}", result);
    result
}

#[cfg(test)]
mod tests {
    use crate::common::read_file;
    use crate::exercises::day5::run_part1;
    use crate::exercises::day5::run_part2;
    #[test]
    fn test() {
        assert_eq!(
            run_part1(&read_file("./src/exercises/day5/input_test.txt")),
            143
        );
        assert_eq!(
            run_part1(&read_file("./src/exercises/day5/input.txt")),
            4996
        );
        //        assert_eq!(
        //            run_part2(&read_file("./src/exercises/day5/input_test.txt")),
        //            9
        //        );
        //        assert_eq!(
        //            run_part2(&read_file("./src/exercises/day5/input.txt")),
        //            1939
        //        )
    }
}

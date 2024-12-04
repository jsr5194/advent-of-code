use log::info;
use std::fmt;

#[derive(Debug)]
enum Pattern {
    Line,
    X,
}

#[derive(Debug, Default)]
struct WordSearch {
    contents: Vec<Vec<char>>,
}

impl WordSearch {
    fn find_pattern_count(&self, raw_word: &str, pattern: Pattern) -> u32 {
        let word = String::from(raw_word);
        match pattern {
            Pattern::X => self.check_in_x(&word),
            Pattern::Line => self.check_in_line(&word),
        }
    }

    fn check_in_line(&self, raw_word: &str) -> u32 {
        let mut count = 0;
        let word = String::from(raw_word);

        // iterate over every index checking every possible direction
        let height = self.contents.len();
        let width = self.contents[0].len();

        // process by row
        for r_idx in 0..height {
            // flags to determine whether or not a given direction can be
            // checked based on the current index (i.e. cant check backwards at 0,0)
            let mut can_check_north = false;
            let mut can_check_south = false;
            let mut can_check_east = false;
            let mut can_check_west = false;
            let mut can_check_northeast = false;
            let mut can_check_northwest = false;
            let mut can_check_southeast = false;
            let mut can_check_southwest = false;

            // determine north<->south
            if r_idx > height - word.len() {
                // ex (end,anything)
                can_check_north = true;
                can_check_south = false;
            } else if r_idx < word.len() - 1 {
                // ex (0,anything)
                can_check_north = false;
                can_check_south = true;
            } else {
                // ex (wordlen,wordlen)
                can_check_north = true;
                can_check_south = true;
            }

            // process by column
            for c_idx in 0..width {
                // determine east<->west
                if c_idx > width - word.len() {
                    // ex (anything,end)
                    can_check_east = false;
                    can_check_west = true;
                } else if c_idx < word.len() - 1 {
                    // ex (anything,0)
                    can_check_east = true;
                    can_check_west = false;
                } else {
                    // ex (wordlen,wordlen)
                    can_check_east = true;
                    can_check_west = true;
                }

                // determine diagonals
                if can_check_north && can_check_east {
                    can_check_northeast = true;
                } else {
                    can_check_northeast = false;
                }
                if can_check_north && can_check_west {
                    can_check_northwest = true;
                } else {
                    can_check_northwest = false;
                }
                if can_check_south && can_check_east {
                    can_check_southeast = true;
                } else {
                    can_check_southeast = false;
                }
                if can_check_south && can_check_west {
                    can_check_southwest = true;
                } else {
                    can_check_southwest = false;
                }

                // check for the word
                if can_check_north && self.check_north(&word, r_idx, c_idx) {
                    count += 1;
                }
                if can_check_south && self.check_south(&word, r_idx, c_idx) {
                    count += 1;
                }
                if can_check_east && self.check_east(&word, r_idx, c_idx) {
                    count += 1;
                }
                if can_check_west && self.check_west(&word, r_idx, c_idx) {
                    count += 1;
                }
                if can_check_northeast && self.check_northeast(&word, r_idx, c_idx) {
                    count += 1;
                }
                if can_check_northwest && self.check_northwest(&word, r_idx, c_idx) {
                    count += 1;
                }
                if can_check_southeast && self.check_southeast(&word, r_idx, c_idx) {
                    count += 1;
                }
                if can_check_southwest && self.check_southwest(&word, r_idx, c_idx) {
                    count += 1;
                }
            }
        }

        count as u32
    }

    fn check_north(&self, word: &String, r_idx: usize, c_idx: usize) -> bool {
        let mut word_found = true;
        let letters = word.chars().collect::<Vec<char>>();
        for l_idx in 0..letters.len() {
            if letters[l_idx] != self.contents[r_idx - l_idx][c_idx] {
                word_found = false;
                break;
            }
        }
        word_found
    }

    fn check_south(&self, word: &String, r_idx: usize, c_idx: usize) -> bool {
        let mut word_found = true;
        let letters = word.chars().collect::<Vec<char>>();
        for l_idx in 0..letters.len() {
            if letters[l_idx] != self.contents[r_idx + l_idx][c_idx] {
                word_found = false;
                break;
            }
        }
        word_found
    }

    fn check_east(&self, word: &String, r_idx: usize, c_idx: usize) -> bool {
        let mut word_found = true;
        let letters = word.chars().collect::<Vec<char>>();
        for l_idx in 0..letters.len() {
            if letters[l_idx] != self.contents[r_idx][c_idx + l_idx] {
                word_found = false;
                break;
            }
        }
        word_found
    }

    fn check_west(&self, word: &String, r_idx: usize, c_idx: usize) -> bool {
        let mut word_found = true;
        let letters = word.chars().collect::<Vec<char>>();
        for l_idx in 0..letters.len() {
            if letters[l_idx] != self.contents[r_idx][c_idx - l_idx] {
                word_found = false;
                break;
            }
        }
        word_found
    }

    fn check_northeast(&self, word: &String, r_idx: usize, c_idx: usize) -> bool {
        let mut word_found = true;
        let letters = word.chars().collect::<Vec<char>>();
        for l_idx in 0..letters.len() {
            if letters[l_idx] != self.contents[r_idx - l_idx][c_idx + l_idx] {
                word_found = false;
                break;
            }
        }
        word_found
    }

    fn check_northwest(&self, word: &String, r_idx: usize, c_idx: usize) -> bool {
        let mut word_found = true;
        let letters = word.chars().collect::<Vec<char>>();
        for l_idx in 0..letters.len() {
            if letters[l_idx] != self.contents[r_idx - l_idx][c_idx - l_idx] {
                word_found = false;
                break;
            }
        }
        word_found
    }

    fn check_southeast(&self, word: &String, r_idx: usize, c_idx: usize) -> bool {
        let mut word_found = true;
        let letters = word.chars().collect::<Vec<char>>();
        for l_idx in 0..letters.len() {
            if letters[l_idx] != self.contents[r_idx + l_idx][c_idx + l_idx] {
                word_found = false;
                break;
            }
        }
        word_found
    }

    fn check_southwest(&self, word: &String, r_idx: usize, c_idx: usize) -> bool {
        let mut word_found = true;
        let letters = word.chars().collect::<Vec<char>>();
        for l_idx in 0..letters.len() {
            if letters[l_idx] != self.contents[r_idx + l_idx][c_idx - l_idx] {
                word_found = false;
                break;
            }
        }
        word_found
    }

    fn check_in_x(&self, word: &String) -> u32 {
        let mut count = 0;
        let height = self.contents.len();
        let width = self.contents[0].len();
        let window_width = word.len();
        let window_height = window_width;
        let letters = word.chars().collect::<Vec<char>>();

        // sanity checks
        if window_width > width || window_height > height {
            panic!("Somehow the dimensions of the window are larger than the provided wordsearch");
        } else if word.len() % 2 == 0 {
            panic!("Search term is of even length which means we can't create an X pattern");
        }

        // get the midpoint index
        let mid_idx = word.len() / 2;

        // iterate over the rows starting at the midpoint index
        for r_idx in mid_idx..height - mid_idx {
            // iterate over the cols starting at the midpoint index
            for c_idx in mid_idx..width - mid_idx {
                // if the current char isn't the mid char of the target word no point in continuing
                if self.contents[r_idx][c_idx] == letters[mid_idx] {
                    // make a new wordsearch of just the window
                    let mut raw_window = String::default();

                    let tl_r_idx = r_idx - mid_idx;
                    let tl_c_idx = c_idx - mid_idx;
                    for y in tl_r_idx..tl_r_idx + window_height {
                        for x in tl_c_idx..tl_c_idx + window_width {
                            raw_window = format!("{}{}", raw_window, self.contents[y][x])
                        }
                        raw_window = format!("{}\n", raw_window);
                    }

                    let window_ws = WordSearch::from(&raw_window);
                    if (window_ws.check_northeast(word, window_height - 1, 0)
                        || window_ws.check_southwest(word, 0, window_width - 1))
                        && (window_ws.check_northwest(word, window_height - 1, window_width - 1)
                            || window_ws.check_southeast(word, 0, 0))
                    {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}

impl From<&String> for WordSearch {
    fn from(raw_data: &String) -> Self {
        let mut ws = WordSearch::default();
        for line in raw_data.lines() {
            ws.contents.push(line.chars().collect::<Vec<char>>())
        }
        ws
    }
}

impl fmt::Display for WordSearch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut msg = String::default();
        for row in &self.contents {
            for col in row {
                msg = format!("{}{}", msg, col);
            }
            msg = format!("{}\n", msg);
        }
        write!(f, "{}", msg)
    }
}

pub fn process_input(filedata: &String) -> WordSearch {
    WordSearch::from(filedata)
}

pub fn run_part1(filedata: &String) -> u32 {
    let mut ws = process_input(filedata);

    let result = ws.find_pattern_count("XMAS", Pattern::Line);
    println!("Day 4 Part 1 Result: {}", result);
    result
}

pub fn run_part2(filedata: &String) -> u32 {
    let mut ws = process_input(filedata);

    let result = ws.find_pattern_count("MAS", Pattern::X);
    println!("Day 4 Part 2 Result: {}", result);
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
            18
        );
        assert_eq!(
            run_part1(&read_file("./src/exercises/day4/input.txt")),
            2547
        );
        assert_eq!(
            run_part2(&read_file("./src/exercises/day4/input_test.txt")),
            9
        );
        assert_eq!(
            run_part2(&read_file("./src/exercises/day4/input.txt")),
            1939
        )
    }
}

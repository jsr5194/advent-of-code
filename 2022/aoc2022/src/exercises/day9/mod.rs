use log::info;
use std::collections::{HashMap, VecDeque};
use std::fmt;

pub fn process_input(filedata: &String) -> MotionSeries {
    MotionSeries::from(filedata)
}

pub fn run_part1(filedata: &String) -> usize {
    let series = process_input(filedata);
    let result = part1(&series);
    info!("Day 9 Part 1 Answer: {}", result);
    result
}

pub fn part1(series: &MotionSeries) -> usize {
    let mut bridge = Bridge::default();
    for motion in &series.motions {
        bridge.process_motion(&motion);
    }
    let mut result = 0;
    for (coord, plank) in &bridge.planks {
        if plank.tail_visit_count > 0 {
            result += 1;
        }
    }
    //println!("{}", bridge);
    result
}

pub fn run_part2(filedata: &String) -> usize {
    let series = process_input(filedata);
    let result = part2(&series);
    info!("Day 9 Part 2 Answer: {}", result);
    result
}

pub fn part2(series: &MotionSeries) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::common::read_file;
    use crate::exercises::day9::run_part1;
    use crate::exercises::day9::run_part2;
    #[test]
    fn test() {
        assert_eq!(
            run_part1(&read_file("./src/exercises/day9/input_test.txt")),
            13
        );
        assert_eq!(
            run_part1(&read_file("./src/exercises/day9/input.txt")),
            6339
        );
    }
}

#[derive(Debug, Default, Eq, PartialEq, Hash, Clone)]
pub struct Coordinates {
    x: usize,
    y: usize,
}

impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = format!("({},{})", self.x, self.y);
        write!(f, "{}", msg)
    }
}

impl From<&String> for Coordinates {
    fn from(coord_str: &String) -> Self {
        if coord_str.len() < 5 {
            info!("Invalid coordinate passed: {:?}", coord_str);
            panic!("Invalid coordinate passed");
        }

        let x = coord_str
            .split(",")
            .nth(0)
            .unwrap()
            .split("(")
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let y = coord_str
            .split(",")
            .nth(1)
            .unwrap()
            .split(")")
            .nth(0)
            .unwrap()
            .parse::<usize>()
            .unwrap();

        Coordinates { x: x, y: y }
    }
}

#[derive(Debug, Default)]
struct Plank {
    position: Coordinates,
    tail_visit_count: usize,
}

impl fmt::Display for Plank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut pos = String::default();
        if self.tail_visit_count > 0 {
            pos = "#".to_string()
        } else {
            pos = ".".to_string()
        }
        write!(f, "{}", pos)
    }
}

#[derive(Debug)]
struct Bridge {
    planks: HashMap<Coordinates, Plank>,
    head: Coordinates,
    tail: Coordinates,
}

impl Bridge {
    fn get_max_y(&self) -> usize {
        let mut max_y = 0;
        for key in self.planks.keys() {
            if key.y > max_y {
                max_y = key.y;
            }
        }
        max_y
    }
    fn get_max_x(&self) -> usize {
        let mut max_x = 0;
        for key in self.planks.keys() {
            if key.x > max_x {
                max_x = key.x;
            }
        }
        max_x
    }

    fn update_tail_position(&mut self, x: usize, y: usize) {
        self.tail.x = x;
        self.tail.y = y;
        if let Some(cur_plank) = self.planks.get_mut(&self.tail) {
            cur_plank.tail_visit_count += 1;
        } else {
            self.planks.insert(
                self.tail.clone(),
                Plank {
                    position: self.tail.clone(),
                    tail_visit_count: 1,
                },
            );
        }
    }

    fn move_tail(&mut self) {
        // move the tail
        //   don't need to bother with the following cases:
        //     1) when the head and tail are in the sample place we don't need to do anything
        //     2) tail is already next to the head

        if self.head.x == self.tail.x && self.head.y == self.tail.y - 2 {
            // head is up with one space between
            // H
            // .
            // T
            self.update_tail_position(self.tail.x, self.tail.y - 1);
        } else if self.head.x == self.tail.x + 2 && self.head.y == self.tail.y - 1 {
            // head up and left with a space in between
            // ..H
            // T..
            self.update_tail_position(self.tail.x + 1, self.tail.y - 1);
        } else if self.head.x == self.tail.x + 1 && self.head.y == self.tail.y - 2 {
            // head up and right with a space in between
            // .H
            // ..
            // T.
            self.update_tail_position(self.tail.x + 1, self.tail.y - 1);
        } else if self.head.x == self.tail.x - 2 && self.head.y == self.tail.y - 1 {
            // head up and left with a space in between
            // H..
            // ..T
            self.update_tail_position(self.tail.x - 1, self.tail.y - 1);
        } else if self.head.x == self.tail.x - 1 && self.head.y == self.tail.y - 2 {
            // head up and left with a space in between
            // H.
            // ..
            // .T
            self.update_tail_position(self.tail.x - 1, self.tail.y - 1);
        } else if self.head.x == self.tail.x && self.head.y == self.tail.y + 2 {
            // head is down with one space between
            // T
            // .
            // H
            self.update_tail_position(self.tail.x, self.tail.y + 1);
        } else if self.head.x == self.tail.x - 2 && self.head.y == self.tail.y + 1 {
            // ..T
            // H..
            self.update_tail_position(self.tail.x - 1, self.tail.y + 1);
        } else if self.head.x == self.tail.x - 1 && self.head.y == self.tail.y + 2 {
            // head down and left with a space in between
            // .T
            // ..
            // H.
            self.update_tail_position(self.tail.x - 1, self.tail.y + 1);
        } else if self.head.x == self.tail.x + 2 && self.head.y == self.tail.y + 1 {
            // head down and right with a space in between
            // T..
            // ..H
            self.update_tail_position(self.tail.x + 1, self.tail.y + 1);
        } else if self.head.x == self.tail.x + 1 && self.head.y == self.tail.y + 2 {
            // head down and right with a space in between
            // T.
            // ..
            // .H
            self.update_tail_position(self.tail.x + 1, self.tail.y + 1);
        } else if self.head.x == self.tail.x - 2 && self.head.y == self.tail.y {
            // head is left with one space between
            // H.T
            self.update_tail_position(self.tail.x - 1, self.tail.y);
        } else if self.head.x == self.tail.x + 2 && self.head.y == self.tail.y {
            // head is right with one space between
            // T.H
            self.update_tail_position(self.tail.x + 1, self.tail.y);
        }
    }

    fn update_rope(&mut self) {
        if !self.planks.contains_key(&self.head) {
            self.planks.insert(
                self.head.clone(),
                Plank {
                    position: self.head.clone(),
                    tail_visit_count: 0,
                },
            );
        }
        self.move_tail();
        //println!("{}", self);
    }

    fn process_motion(&mut self, motion: &Motion) {
        // move the head
        //println!("\n==========\n{:?}", motion);
        match motion {
            Motion::Up { count } => {
                for _ in 0..*count {
                    self.head.y -= 1;
                    self.update_rope();
                }
            }
            Motion::Down { count } => {
                for _ in 0..*count {
                    self.head.y += 1;
                    self.update_rope();
                }
            }
            Motion::Left { count } => {
                for _ in 0..*count {
                    self.head.x -= 1;
                    self.update_rope();
                }
            }
            Motion::Right { count } => {
                for _ in 0..*count {
                    self.head.x += 1;
                    self.update_rope();
                }
            }
            Motion::None => panic!("you still have a 'None' motion hanging around"),
        }
    }
}

impl Default for Bridge {
    fn default() -> Self {
        let start_pos = Coordinates::from(&"(6,6)".to_string());
        let mut bridge = Bridge {
            planks: HashMap::new(),
            head: start_pos.clone(),
            tail: start_pos.clone(),
        };
        bridge.planks.insert(
            start_pos.clone(),
            Plank {
                position: start_pos.clone(),
                tail_visit_count: 1,
            },
        );
        bridge
    }
}

impl fmt::Display for Bridge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut msg = String::default();
        for y in 0..=self.get_max_y() {
            for x in 0..=self.get_max_x() {
                let pos = Coordinates::from(&format!("({},{})", x, y));
                if self.planks.contains_key(&pos) {
                    if &pos == &self.head {
                        msg = format!("{} H", msg);
                    } else if &pos == &self.tail {
                        msg = format!("{} T", msg);
                    } else {
                        msg = format!("{} {}", msg, self.planks.get(&pos).unwrap());
                    }
                } else {
                    msg = format!("{} .", msg);
                }
            }
            msg = format!("{}\n", msg);
        }
        write!(f, "{}", msg)
    }
}

#[derive(Debug)]
enum Motion {
    Up { count: usize },
    Down { count: usize },
    Left { count: usize },
    Right { count: usize },
    None,
}

impl Default for Motion {
    fn default() -> Self {
        Motion::None
    }
}

impl From<&str> for Motion {
    fn from(raw_motion: &str) -> Self {
        let mut split_motion = raw_motion.split(" ");
        match split_motion.next().unwrap() {
            "U" => Motion::Up {
                count: split_motion
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap()
                    .clone(),
            },
            "D" => Motion::Down {
                count: split_motion
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap()
                    .clone(),
            },
            "L" => Motion::Left {
                count: split_motion
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap()
                    .clone(),
            },
            "R" => Motion::Right {
                count: split_motion
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap()
                    .clone(),
            },
            _ => unreachable!("Invalid motion code detected"),
        }
    }
}

#[derive(Debug, Default)]
pub struct MotionSeries {
    motions: VecDeque<Motion>,
}

impl From<&String> for MotionSeries {
    fn from(raw_series: &String) -> Self {
        let mut series = MotionSeries::default();
        for raw_motion in raw_series.lines() {
            series.motions.push_back(Motion::from(raw_motion));
        }
        series
    }
}

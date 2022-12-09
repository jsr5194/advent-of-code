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
    //println!("# Initial\n{}\n\n\n", bridge);
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
    let mut bridge = Bridge::default();
    bridge.set_knots(10);
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
        assert_eq!(
            run_part2(&read_file("./src/exercises/day9/input_test_2.txt")),
            36
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
    knots: Vec<Coordinates>,
}

impl Bridge {
    fn set_knots(&mut self, count: usize) {
        if count < 2 {
            panic!("must have at least two knots");
        }

        // -2 to account for the two created by default
        let new_knot = self.knots[self.knots.len() - 1].clone();
        for _ in 0..count - 2 {
            self.knots.push(new_knot.clone());
        }
    }
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

    fn update_knot_position(&mut self, knot_idx: usize, x: usize, y: usize) {
        let tail = Coordinates::from(&format!("({},{})", x, y));

        self.knots[knot_idx] = tail.clone();

        if !self.planks.contains_key(&tail) {
            self.planks.insert(
                tail.clone(),
                Plank {
                    position: tail.clone(),
                    tail_visit_count: 0,
                },
            );
        }
    }

    fn move_trailing_knot(
        &mut self,
        trailing_knot_idx: usize,
        head: &Coordinates,
        tail: &Coordinates,
    ) {
        // move the tail
        //   don't need to bother with the following cases:
        //     1) when the head and tail are in the sample place we don't need to do anything
        //     2) tail is already next to the head

        if head.x == tail.x && head.y == tail.y - 2 {
            // head is up with one space between
            // H
            // .
            // T
            self.update_knot_position(trailing_knot_idx, tail.x, tail.y - 1);
        } else if head.x == tail.x + 2 && head.y == tail.y - 1 {
            // head up and left with a space in between
            // ..H
            // T..
            self.update_knot_position(trailing_knot_idx, tail.x + 1, tail.y - 1);
        } else if head.x == tail.x + 1 && head.y == tail.y - 2 {
            // head up and right with a space in between
            // .H
            // ..
            // T.
            self.update_knot_position(trailing_knot_idx, tail.x + 1, tail.y - 1);
        } else if head.x == tail.x + 2 && head.y == tail.y - 2 {
            // ...1
            // ...2
            // ....
            // 3...
            self.update_knot_position(trailing_knot_idx, tail.x + 1, tail.y - 1);
        } else if head.x == tail.x - 2 && head.y == tail.y - 1 {
            // head up and left with a space in between
            // H..
            // ..T
            self.update_knot_position(trailing_knot_idx, tail.x - 1, tail.y - 1);
        } else if head.x == tail.x - 1 && head.y == tail.y - 2 {
            // head up and left with a space in between
            // H.
            // ..
            // .T
            self.update_knot_position(trailing_knot_idx, tail.x - 1, tail.y - 1);
        } else if head.x == tail.x - 2 && head.y == tail.y - 2 {
            // 1...
            // 2...
            // ....
            // ...3
            self.update_knot_position(trailing_knot_idx, tail.x - 1, tail.y - 1);
        } else if head.x == tail.x && head.y == tail.y + 2 {
            // head is down with one space between
            // T
            // .
            // H
            self.update_knot_position(trailing_knot_idx, tail.x, tail.y + 1);
        } else if head.x == tail.x - 2 && head.y == tail.y + 1 {
            // ..T
            // H..
            self.update_knot_position(trailing_knot_idx, tail.x - 1, tail.y + 1);
        } else if head.x == tail.x - 1 && head.y == tail.y + 2 {
            // head down and left with a space in between
            // .T
            // ..
            // H.
            self.update_knot_position(trailing_knot_idx, tail.x - 1, tail.y + 1);
        } else if head.x == tail.x - 2 && head.y == tail.y + 2 {
            // ...3
            // ....
            // .2..
            // 1...
            self.update_knot_position(trailing_knot_idx, tail.x - 1, tail.y + 1);
        } else if head.x == tail.x + 2 && head.y == tail.y + 1 {
            // head down and right with a space in between
            // T..
            // ..H
            self.update_knot_position(trailing_knot_idx, tail.x + 1, tail.y + 1);
        } else if head.x == tail.x + 1 && head.y == tail.y + 2 {
            // head down and right with a space in between
            // T.
            // ..
            // .H
            self.update_knot_position(trailing_knot_idx, tail.x + 1, tail.y + 1);
        } else if head.x == tail.x + 2 && head.y == tail.y + 2 {
            // 3...
            // ....
            // ..2.
            // ...1
            self.update_knot_position(trailing_knot_idx, tail.x + 1, tail.y + 1);
        } else if head.x == tail.x - 2 && head.y == tail.y {
            // head is left with one space between
            // H.T
            self.update_knot_position(trailing_knot_idx, tail.x - 1, tail.y);
        } else if head.x == tail.x + 2 && head.y == tail.y {
            // head is right with one space between
            // T.H
            self.update_knot_position(trailing_knot_idx, tail.x + 1, tail.y);
        }
    }

    fn update_rope(&mut self) {
        for idx in 0..self.knots.len() - 1 {
            let mut head = self.knots[idx].clone();
            let mut tail = self.knots[idx + 1].clone();
            if !self.planks.contains_key(&head) {
                self.planks.insert(
                    head.clone(),
                    Plank {
                        position: head.clone(),
                        tail_visit_count: 0,
                    },
                );
            }
            self.move_trailing_knot(idx + 1, &head, &tail);
        }

        // update the tail tracker after finished
        if let Some(tail) = self.planks.get_mut(&self.knots[self.knots.len() - 1]) {
            //println!("before: {:?}", tail.tail_visit_count);
            tail.tail_visit_count += 1;
            //println!("after: {:?}", tail.tail_visit_count);
        }

        //println!("{}", self);
    }

    fn process_motion(&mut self, motion: &Motion) {
        // move the head
        //println!("\n==========\n{:?}", motion);
        match motion {
            Motion::Up { count } => {
                for _ in 0..*count {
                    self.knots[0].y -= 1;
                    self.update_rope();
                }
            }
            Motion::Down { count } => {
                for _ in 0..*count {
                    self.knots[0].y += 1;
                    self.update_rope();
                }
            }
            Motion::Left { count } => {
                for _ in 0..*count {
                    self.knots[0].x -= 1;
                    self.update_rope();
                }
            }
            Motion::Right { count } => {
                for _ in 0..*count {
                    self.knots[0].x += 1;
                    self.update_rope();
                }
            }
            Motion::None => panic!("you still have a 'None' motion hanging around"),
        }
    }
}

impl Default for Bridge {
    fn default() -> Self {
        let start_pos = Coordinates::from(&"(20,20)".to_string());
        let mut knots: Vec<Coordinates> = vec![];
        knots.push(start_pos.clone()); // initial head
        knots.push(start_pos.clone()); // initial tail
        let mut bridge = Bridge {
            planks: HashMap::new(),
            knots: knots.clone(),
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

        let display = "tail_touch";

        if display == "tail_touch" {
            for y in 0..=self.get_max_y() {
                for x in 0..=self.get_max_x() {
                    let pos = Coordinates::from(&format!("({},{})", x, y));
                    if self.planks.contains_key(&pos) {
                        if &pos == &self.knots[0] {
                            msg = format!("{} H", msg);
                        } else if &pos == &self.knots[self.knots.len() - 1] {
                            msg = format!("{} T", msg);
                        } else {
                            msg = format!("{} #", msg);
                        }
                    } else {
                        msg = format!("{} .", msg);
                    }
                }
                msg = format!("{}\n", msg);
            }
        } else {
            for y in 0..=self.get_max_y() {
                for x in 0..=self.get_max_x() {
                    let mut found = false;
                    for knot_idx in 0..self.knots.len() {
                        let pos = Coordinates::from(&format!("({},{})", x, y));
                        if self.knots[knot_idx] == pos {
                            msg = format!("{} {}", msg, knot_idx);
                            found = true;

                            break;
                        }
                    }
                    if !found {
                        msg = format!("{} .", msg);
                    }
                }
                msg = format!("{}\n", msg);
            }
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

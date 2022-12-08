use log::info;
use std::collections::HashMap;
use std::fmt;

pub fn process_input(filedata: &String) -> Forest {
    Forest::from(filedata)
}

pub fn run_part1(filedata: &String) -> usize {
    let forest = process_input(filedata);
    let result = part1(&forest);
    info!("Day 8 Part 1 Answer: {:?}", result);
    result
}

pub fn part1(forest: &Forest) -> usize {
    let mut visible_trees = 0;
    for (coord, tree) in &forest.trees {
        if tree.visible_north {
            visible_trees += 1;
        } else if tree.visible_south {
            visible_trees += 1;
        } else if tree.visible_west {
            visible_trees += 1;
        } else if tree.visible_east {
            visible_trees += 1;
        }
    }
    visible_trees
}

pub fn run_part2(filedata: &String) -> usize {
    let forest = process_input(filedata);
    let result = part2(&forest);
    info!("Day 8 Part 2 Answer: {:?}", result);
    info!("196 is too low");
    result
}

pub fn part2(forest: &Forest) -> usize {
    let mut max_scenic_score = 0;
    for y in 1..=forest.get_max_y() {
        for x in 1..=forest.get_max_x() {
            let mut cur_scenic_score = 1;
            let cur_coord = Coordinates::from(&format!("({},{})", x, y));
            let cur_tree_height = forest.trees.get(&cur_coord).unwrap().height;
            // get north count
            let mut north_count = 0;
            for y2 in (0..y).rev() {
                let north_tree = forest
                    .trees
                    .get(&Coordinates::from(&format!("({},{})", x, y2)))
                    .unwrap();

                north_count += 1;
                if north_tree.height >= cur_tree_height {
                    break;
                }
            }
            cur_scenic_score *= north_count;

            // get south count
            let mut south_count = 0;
            for y2 in y + 1..=forest.get_max_y() {
                let south_tree = forest
                    .trees
                    .get(&Coordinates::from(&format!("({},{})", x, y2)))
                    .unwrap();

                south_count += 1;
                if south_tree.height >= cur_tree_height {
                    break;
                }
            }
            cur_scenic_score *= south_count;

            // get west count
            let mut west_count = 0;
            for x2 in (0..x).rev() {
                let west_tree = forest
                    .trees
                    .get(&Coordinates::from(&format!("({},{})", x2, y)))
                    .unwrap();

                west_count += 1;
                if west_tree.height >= cur_tree_height {
                    break;
                }
            }
            cur_scenic_score *= west_count;

            // get east count
            let mut east_count = 0;
            for x2 in x + 1..=forest.get_max_x() {
                let east_tree = forest
                    .trees
                    .get(&Coordinates::from(&format!("({},{})", x2, y)))
                    .unwrap();

                east_count += 1;
                if east_tree.height >= cur_tree_height {
                    break;
                }
            }
            cur_scenic_score *= east_count;

            if cur_scenic_score > max_scenic_score {
                max_scenic_score = cur_scenic_score;
            }
        }
    }

    max_scenic_score
}

#[cfg(test)]
mod tests {
    use crate::common::read_file;
    use crate::exercises::day8::run_part1;
    use crate::exercises::day8::run_part2;
    #[test]
    fn test() {
        assert_eq!(
            run_part1(&read_file("./src/exercises/day8/input_test.txt")),
            21
        );
        assert_eq!(
            run_part1(&read_file("./src/exercises/day8/input.txt")),
            1825
        );
        assert_eq!(
            run_part2(&read_file("./src/exercises/day8/input_test.txt")),
            8
        );
        assert_eq!(
            run_part2(&read_file("./src/exercises/day8/input.txt")),
            235200
        );
    }
}

#[derive(Debug, Default, Eq, PartialEq, Hash)]
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
            println!("Invalid coordinate passed: {:?}", coord_str);
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

#[derive(Debug, Default, PartialEq, PartialOrd)]
pub struct Tree {
    height: usize,
    visible_north: bool,
    visible_south: bool,
    visible_west: bool,
    visible_east: bool,
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut msg = String::default();

        msg = format!(" {} ", self.height);

        write!(f, "{}", msg)
    }
}

#[derive(Debug, Default)]
pub struct Forest {
    trees: HashMap<Coordinates, Tree>,
}

impl Forest {
    fn get_max_y(&self) -> usize {
        let mut max_y = 0;
        for key in self.trees.keys() {
            if key.y > max_y {
                max_y = key.y;
            }
        }
        max_y
    }
    fn get_max_x(&self) -> usize {
        let mut max_x = 0;
        for key in self.trees.keys() {
            if key.x > max_x {
                max_x = key.x;
            }
        }
        max_x
    }
}

impl fmt::Display for Forest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut msg = String::default();
        for y in 0..=self.get_max_y() {
            for x in 0..=self.get_max_x() {
                let coord = format!("({},{})", x, y);
                msg = format!(
                    "{}{}",
                    msg,
                    self.trees.get(&Coordinates::from(&coord)).unwrap()
                );
            }
            msg = format!("{}\n", msg);
        }
        write!(f, "{}", msg)
    }
}

impl From<&String> for Forest {
    fn from(raw_forest: &String) -> Self {
        let mut forest = Forest::default();
        let mut y = 0;
        for row in raw_forest.lines() {
            let mut x = 0;
            for raw_tree in row
                .chars()
                .map(|t| t.to_string().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
            {
                forest.trees.insert(
                    Coordinates::from(&format!("({},{})", x, y)),
                    Tree {
                        height: raw_tree,
                        visible_north: true,
                        visible_south: true,
                        visible_west: true,
                        visible_east: true,
                    },
                );
                x += 1;
            }
            y += 1;
        }

        // fix visibility
        // no need to process the outer border since its always true
        for y in 1..forest.get_max_y() {
            for x in 1..forest.get_max_x() {
                // current
                let cur_coord = Coordinates::from(&format!("({},{})", x, y));
                let cur_tree_height = forest.trees.get(&cur_coord).unwrap().height;
                let mut visible_north = true;
                let mut visible_south = true;
                let mut visible_west = true;
                let mut visible_east = true;

                // check trees from the north
                for y2 in 0..y {
                    let north_tree = forest
                        .trees
                        .get(&Coordinates::from(&format!("({},{})", x, y2)))
                        .unwrap();

                    if north_tree.height >= cur_tree_height {
                        visible_north = false;
                        break;
                    }
                }

                // check trees from the south
                for y2 in y + 1..=forest.get_max_y() {
                    let south_tree = forest
                        .trees
                        .get(&Coordinates::from(&format!("({},{})", x, y2)))
                        .unwrap();

                    if south_tree.height >= cur_tree_height {
                        visible_south = false;
                        break;
                    }
                }

                // check trees from the west
                for x2 in 0..x {
                    let west_tree = forest
                        .trees
                        .get(&Coordinates::from(&format!("({},{})", x2, y)))
                        .unwrap();

                    if west_tree.height >= cur_tree_height {
                        visible_west = false;
                        break;
                    }
                }

                // check trees from the south
                for x2 in x + 1..=forest.get_max_x() {
                    let east_tree = forest
                        .trees
                        .get(&Coordinates::from(&format!("({},{})", x2, y)))
                        .unwrap();

                    if east_tree.height >= cur_tree_height {
                        visible_east = false;
                        break;
                    }
                }

                if let Some(cur_tree) = forest.trees.get_mut(&cur_coord) {
                    cur_tree.visible_north = visible_north;
                    cur_tree.visible_south = visible_south;
                    cur_tree.visible_west = visible_west;
                    cur_tree.visible_east = visible_east;
                }
            }
        }

        forest
    }
}

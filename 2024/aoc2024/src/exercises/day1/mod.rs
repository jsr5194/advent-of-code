use log::info;

#[derive(Debug, Default)]
struct LocationList {
    location_ids: Vec<usize>,
}

pub fn process_input(filedata: &String) -> Vec<LocationList> {
    let mut location_lists = vec![];
    location_lists.push(LocationList::default());
    location_lists.push(LocationList::default());

    for line in filedata.lines() {
        let mut data = line.split("   ");
        location_lists[0]
            .location_ids
            .push(data.next().unwrap().parse::<usize>().unwrap().clone());
        location_lists[1]
            .location_ids
            .push(data.next().unwrap().parse::<usize>().unwrap().clone());
    }

    location_lists
}

pub fn run_part1(filedata: &String) -> u32 {
    let mut location_lists = process_input(filedata);

    for list in &mut location_lists {
        list.location_ids.sort();
    }

    let mut total_distance = 0;
    for idx in 0..location_lists[0].location_ids.len() {
        let l1 = location_lists[0].location_ids[idx];
        let l2 = location_lists[1].location_ids[idx];

        let diff: usize;
        if l2 > l1 {
            diff = l2 - l1;
        } else {
            diff = l1 - l2;
        }
        total_distance += diff
    }

    let result = total_distance as u32;
    println!("Part 1 Result: {:?}", result);
    result
}

pub fn run_part2(filedata: &String) -> u32 {
    let mut location_lists = process_input(filedata);

    let mut similarity_score = 0;
    for loc_i in &location_lists[0].location_ids {
        let mut count = 0;
        for loc_j in &location_lists[1].location_ids {
            if loc_i == loc_j {
                count += 1;
            }
        }
        similarity_score += loc_i * count;
    }

    let result = similarity_score as u32;
    println!("Part 2 Result: {:?}", result);
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
            11
        );
        assert_eq!(
            run_part1(&read_file("./src/exercises/day1/input.txt")),
            2769675
        );
        assert_eq!(
            run_part2(&read_file("./src/exercises/day1/input_test.txt")),
            31
        );
        assert_eq!(
            run_part2(&read_file("./src/exercises/day1/input.txt")),
            24643097
        )
    }
}

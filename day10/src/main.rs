use std::fs;
use std::collections::HashMap;

// const FILE_PATH: &str = "data/temp1.txt";
// const FILE_PATH: &str = "data/temp2.txt";
const FILE_PATH: &str = "data/org.txt";

fn main() {
    part1();
    part2();
}

fn part1() {
    match fs::read_to_string(FILE_PATH) {
        Ok(text) => {
            let mut lines = text.split('\n').collect::<Vec<&str>>();
            lines.pop();

            let mut numbers: Vec<u64> = Vec::with_capacity(lines.len() + 2);
            for line in &lines {
                let n = line.parse().expect("should be a number");
                numbers.push(n);
            }

            numbers.push(0);
            numbers.sort();
            numbers.push(numbers[numbers.len()-1] + 3);

            let mut diff_1_count: u64 = 0;
            let mut diff_3_count: u64 = 0;

            for i in 0..numbers.len()-1 {
                let diff = numbers[i+1] - numbers[i];
                match diff {
                    1 => {
                        diff_1_count += 1;
                    },
                    3 => {
                        diff_3_count += 1;
                    },
                    _ => {},
                }
            }

            println!("[PART 1]: {}", diff_1_count * diff_3_count);
        },
        Err(err) => panic!("{}", err),
    }
}

fn part2() {
    match fs::read_to_string(FILE_PATH) {
        Ok(text) => {
            let mut lines = text.split('\n').collect::<Vec<&str>>();
            lines.pop();

            let mut numbers: Vec<u64> = Vec::with_capacity(lines.len() + 2);
            for line in &lines {
                let n = line.parse().expect("should be a number");
                numbers.push(n);
            }

            numbers.push(0);
            numbers.sort();
            numbers.push(numbers[numbers.len()-1] + 3);

            let mut map: HashMap<(usize, usize), u64> = HashMap::new();
            println!(
                "[PART 2]: {}",
                count_ways(
                    &numbers,
                    0,
                    0,
                    numbers.len()-1,
                    &mut map,
                ),
            );
        },
        Err(err) => panic!("{}", err),
    }
}

fn count_ways(
    adapters: &Vec<u64>,
    prev_i: usize,
    start_i: usize,
    end_i: usize,
    map: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    match map.get(&(prev_i, start_i)) {
        Some(count) => {
            return *count;
        },
        None => {},
    }

    let mut counts: u64 = 0;
    for i in start_i..=end_i {
        if i == 0 {
            continue;
        }

        let diff = adapters[i] - adapters[prev_i];

        match diff {
            1 | 2 | 3 => {
                if start_i == end_i {
                    return 1_u64;
                }
                
                counts += count_ways(
                    adapters,
                    i,
                    i + 1,
                    end_i,
                    map,
                );
            },
            _ => {},
        }
    }

    map.insert(
        (prev_i, start_i),
        counts,
    );

    return counts;
}

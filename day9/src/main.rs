use std::fs;
use std::collections::HashSet;

// const FILE_PATH: &str = "data/temp.txt";
// const PREAMBLE: usize = 5;

const FILE_PATH: &str = "data/org.txt";
const PREAMBLE: usize = 25;

fn main() {
    part1();
    part2();
}

fn part1() {
    match fs::read_to_string(FILE_PATH) {
        Ok(text) => {
            let mut lines = text.split('\n').collect::<Vec<&str>>();
            lines.pop();

            let mut numbers: Vec<u64> = Vec::new();
            for line in lines {
                let n: u64 = line.parse().expect("should be a number");
                numbers.push(n);
            }

            let mut preamble_numbers: HashSet<u64> = HashSet::new();
            for i in 0..PREAMBLE {
                preamble_numbers.insert(numbers[i]);
            }

            for i in PREAMBLE..numbers.len() {
                let mut is_found = false;

                for preamble in &preamble_numbers {
                    if numbers[i] > *preamble {
                        if preamble_numbers.contains(&(numbers[i] - *preamble)) {
                            is_found = true;
                            break;
                        }
                    }
                }

                if !is_found {
                    println!("[PART 1] {}", numbers[i]);
                    break;
                }

                preamble_numbers.remove(&numbers[i - PREAMBLE]);
                preamble_numbers.insert(numbers[i]);
            }
        },
        Err(err) => panic!("{}", err),
    }
}

fn part2() {
    match fs::read_to_string(FILE_PATH) {
        Ok(text) => {
            let mut lines = text.split('\n').collect::<Vec<&str>>();
            lines.pop();

            let mut numbers: Vec<u64> = Vec::new();
            for line in lines {
                let n: u64 = line.parse().expect("should be a number");
                numbers.push(n);
            }

            let mut preamble_numbers: HashSet<u64> = HashSet::new();
            for i in 0..PREAMBLE {
                preamble_numbers.insert(numbers[i]);
            }

            for i in PREAMBLE..numbers.len() {
                let mut is_found = false;

                for preamble in &preamble_numbers {
                    if numbers[i] > *preamble {
                        if preamble_numbers.contains(&(numbers[i] - *preamble)) {
                            is_found = true;
                            break;
                        }
                    }
                }

                if !is_found {
                    let mut left_i: usize = 0;
                    let mut right_i: usize = 0;

                    let mut total: u64 = 0;
                    loop {
                        if total == numbers[i] {
                            numbers[left_i..right_i].sort();

                            println!("[PART 2] {}", numbers[left_i] + numbers[right_i-1]);
                            break;
                        }

                        while total < numbers[i] {
                            total += numbers[right_i];
                            right_i += 1;
                        }

                        while total > numbers[i] {
                            total -= numbers[left_i];
                            left_i += 1;
                        }
                    }

                    break;
                }

                preamble_numbers.remove(&numbers[i - PREAMBLE]);
                preamble_numbers.insert(numbers[i]);
            }
        },
        Err(err) => panic!("{}", err),
    }
}

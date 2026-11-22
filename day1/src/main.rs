use std::fs;
use std::collections::HashSet;

fn main() {
    // const FILE_PATH: &str = "data/test.txt";
    const FILE_PATH: &str = "data/org.txt";

    part1(FILE_PATH);
    part2(FILE_PATH);
}

fn part1(path: &str) {
    match fs::read_to_string(path) {
        Ok(data) => {
            let mut entries: HashSet<u64> = HashSet::new();
            for n in data.split('\n') {
                if n == "" {
                    break;
                }
                entries.insert(n.parse().expect("should be a number"));
            }

            let cmp_num: u64 = 2020;

            // SOLUTION 1
            for &n1 in entries.iter() {
                if n1 > cmp_num {
                    continue;
                }

                let n2 = cmp_num - n1;

                if entries.contains(&n2) {
                    println!("[PART 1] {n1} x {n2} = {}", n1 * n2);
                    break;
                }
            }

            // SOLUTION 2
            let nums = entries
                .iter()
                .map(|n| *n)
                .collect::<Vec<u64>>();

            for index in 0..nums.len() {
                if nums[index] > 2020 {
                    continue;
                }
                if let Some(mut v) =  rec(&nums, index+1, 2020-nums[index], 2, 2) {
                    v.push(nums[index]);
                    let mut mult = 1u64;

                    print!("[PART 1] ");
                    for j in 0..v.len() {
                        mult *= v[j];
                        if j == v.len() -1 {
                            println!("{} = {}", v[j], mult);
                        } else {
                            print!("{} x ", v[j]);
                        }
                    }

                    break;
                }
            }
        },
        Err(err) => panic!("{}", err),
    }
}

fn part2(path: &str) {
    match fs::read_to_string(path) {
        Ok(data) => {
            let mut entries: HashSet<u64> = HashSet::new();
            for n in data.split('\n') {
                if n == "" {
                    break;
                }
                entries.insert(n.parse().expect("should be a number"));
            }

            let cmp_num: u64 = 2020;

            // SOLUTION 1:
            let mut remains: HashSet<u64> = HashSet::new();

            for &n1 in entries.iter() {
                if n1 > cmp_num {
                    continue;
                }
                remains.insert(cmp_num - n1);
            }

            'outer: for &remain in remains.iter() {
                for &n2 in entries.iter() {
                    if n2 > remain {
                        continue;
                    }

                    let n3 = remain - n2;
                    if entries.contains(&n3) {
                        let n1 = cmp_num - remain;
                        println!("[PART 2] {n1} x {n2} x {n3} = {}", n1 * n2 * n3);
                        break 'outer;
                    }
                }
            }

            // SOLUTION 3:
            let nums = entries
                .iter()
                .map(|n| *n)
                .collect::<Vec<u64>>();

            for index in 0..nums.len() {
                if nums[index] > 2020 {
                    continue;
                }
                if let Some(mut v) =  rec(&nums, index+1, 2020-nums[index], 2, 3) {
                    v.push(nums[index]);
                    let mut mult = 1u64;

                    print!("[PART 2] ");
                    for j in 0..v.len() {
                        mult *= v[j];
                        if j == v.len() -1 {
                            println!("{} = {}", v[j], mult);
                        } else {
                            print!("{} x ", v[j]);
                        }
                    }

                    break;
                }
            }
        },
        Err(err) => panic!("{}", err),
    }

    // NOTE:
    // SOLUTION 2:
    // implement the solution without remains HashSet (only using two loops)
    //
    // SOLUTION 3:
    // implement a solution using recursion to calculate for any depth
}

fn rec(nums: &Vec<u64>, start_index: usize, remain: u64, cur_depth: u64, depth: u64) -> Option<Vec<u64>> {
    for index in start_index..nums.len() {
        if cur_depth == depth {
            if nums[index] == remain {
                return Some(vec![nums[index]]);
            }
            continue;
        }

        if nums[index] < remain {
            if let Some(mut v) = rec(nums, index + 1, remain - nums[index], cur_depth + 1, depth) {
                v.push(nums[index]);
                return Some(v);
            }
        }
    }
    return None;
}

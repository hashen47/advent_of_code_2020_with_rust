use std::fs;
use std::collections::{HashSet, HashMap};

// const FILE_PATH: &str = "data/temp.txt";
const FILE_PATH: &str = "data/org.txt";

fn main() {
    part1();
    part2();
}

fn part1() {
    match fs::read_to_string(FILE_PATH) {
        Ok(text) => {
            let mut total_yes_answered_questions: usize  = 0;
            let mut group_anyone_yes_answered_questions: HashSet<u8> = HashSet::new();

            let lines = text.split('\n').collect::<Vec<&str>>();
            for line in lines {
                if line == "" {
                    total_yes_answered_questions += group_anyone_yes_answered_questions.len();
                    group_anyone_yes_answered_questions = HashSet::new();
                    continue;
                }

                for ch in line.bytes() {
                    group_anyone_yes_answered_questions.insert(ch);
                }
            }

            println!("[PART 1] questions total (anyone in the group answered yes): {total_yes_answered_questions}");
        },
        Err(err) => panic!("{}", err),
    }
}

fn part2() {
    match fs::read_to_string(FILE_PATH) {
        Ok(text) => {
            let mut total_yes_answered_questions: usize  = 0;
            let mut group_everyone_yes_answered_questions: HashMap<u8, u64> = HashMap::new();
            let mut group_size: u64 = 0;

            let lines = text.split('\n').collect::<Vec<&str>>();
            for line in lines {
                if line == "" {
                    for (_, &count) in &group_everyone_yes_answered_questions {
                        if count == group_size {
                            total_yes_answered_questions += 1;
                        }
                    }
                    group_everyone_yes_answered_questions = HashMap::new();
                    group_size = 0;
                    continue;
                }

                let mut member_yes_answered_questions: HashSet<u8> = HashSet::new();

                for ch in line.bytes() {
                    member_yes_answered_questions.insert(ch);
                }

                for ch in member_yes_answered_questions {
                    group_everyone_yes_answered_questions.entry(ch).and_modify(|c| *c += 1).or_insert(1);
                }
                
                group_size += 1;
            }

            println!("[PART 2] questions total (everyone in the group answered yes): {total_yes_answered_questions}");
        },
        Err(err) => panic!("{}", err),
    }
}

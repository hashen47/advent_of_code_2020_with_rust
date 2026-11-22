use std::fs;

fn main() {
    // const FILE_PATH: &str = "data/test.txt";
    const FILE_PATH: &str = "data/org.txt";

    part1(FILE_PATH);
    part2(FILE_PATH);
}

fn part1(path: &str) {
    match fs::read_to_string(path) {
        Ok(text) => {
            let mut valid_password_count: u64 = 0;

            for line in text.split('\n') {
                if line == "" {
                    continue;
                }

                // pattern
                // 1-8 n: dpwpmhknmnlglhjtrbpx

                // split the line
                let slice = line.split('-').collect::<Vec<&str>>();
                let min_count: u64 = slice[0]
                    .parse()
                    .expect("should be a number");

                let slice = slice[1].split(": ").collect::<Vec<&str>>();
                let password = slice[1];

                let slice = slice[0].split(' ').collect::<Vec<&str>>();
                let max_count = slice[0]
                    .parse()
                    .expect("should be a number");
                let character = slice[1];

                // get chracter count
                let mut char_count: u64 = 0;
                for ch in password.chars() {
                    if character == ch.to_string() {
                        char_count += 1;
                    }
                }

                // validate character count
                if char_count >= min_count && char_count <= max_count {
                    valid_password_count += 1;
                }
            }

            println!("[PART 1]: {}", valid_password_count);
        },
        Err(err) => panic!("{}", err),
    }
}

fn part2(path: &str) {
    match fs::read_to_string(path) {
        Ok(text) => {
            let mut valid_password_count: u64 = 0;

            for line in text.split('\n') {
                if line == "" {
                    continue;
                }

                // pattern
                // 1-8 n: dpwpmhknmnlglhjtrbpx

                // split the line
                let slice = line.split('-').collect::<Vec<&str>>();
                let first_index: usize = slice[0]
                    .parse()
                    .expect("should be a number");

                let slice = slice[1].split(": ").collect::<Vec<&str>>();
                let password = slice[1];

                let slice = slice[0].split(' ').collect::<Vec<&str>>();
                let second_index: usize = slice[0]
                    .parse()
                    .expect("should be a number");
                let character = slice[1];

                // get chracter count
                let mut char_count: u64 = 0;
                let chars = password.chars().collect::<Vec<char>>();

                if chars[first_index-1].to_string() == character {
                    char_count += 1;
                }
                if chars[second_index-1].to_string() == character {
                    char_count += 1;
                }

                // validate character count
                if char_count == 1 {
                    valid_password_count += 1;
                }
            }

            println!("[PART 2]: {}", valid_password_count);
        },
        Err(err) => panic!("{}", err),
    }
}

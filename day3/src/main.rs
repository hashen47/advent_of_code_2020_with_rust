use std::fs;

const TREE_SYMBOL: char = '#';

// const FILE_PATH: &str = "data/temp.txt";
const FILE_PATH: &str = "data/org.txt";

fn main() {
    println!("[PART 1] right: 3, down: 1, tree_count: {}", count_trees(3, 1));

    // right, down
    let combinations = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];

    let mut mult: u64 = 1;
    for combination in combinations {
        mult *= count_trees(combination.0, combination.1);
    }
    println!("[PART 2] mult: {mult}");
}

fn count_trees(step_right: usize, step_down: usize) -> u64 {
    match fs::read_to_string(FILE_PATH) {
        Ok(text) => {
            let mut lines = text.split('\n').collect::<Vec<&str>>();
            lines.pop();

            let mut tree_count: u64 = 0;
            let mut x: usize = 0;
            let mut y: usize = 0;
            let line_length: usize = lines[0].len();
            loop {
                if y >= lines.len() {
                    break;
                }

                if lines[y].chars().nth(x % line_length).unwrap() == TREE_SYMBOL {
                    tree_count += 1;
                }

                x += step_right;
                y += step_down;
            }

            return tree_count;
        },
        Err(err) => panic!("{}", err),
    }
}

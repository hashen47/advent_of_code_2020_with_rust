use std::fs;

// const FILE_PATH: &str = "data/temp.txt";
const FILE_PATH: &str = "data/org.txt";

fn main() {
    part1();
    part2();
}

fn part1() {
    match fs::read_to_string(FILE_PATH) {
        Ok(text) => {
            let mut highest_seat_id: u64 = 0;

            let mut lines: Vec<&str> = text.split('\n').collect();
            lines.pop();

            for &line in &lines {
                let mut min_row: u64 = 0;
                let mut max_row: u64 = 127;
                let mut row: u64 = 0;

                let chars: &[u8] = line.as_bytes();
                for (i, &ch) in chars[0..7].iter().enumerate() {
                    match ch {
                        b'F' => {
                            max_row = (min_row + max_row) / 2;
                            if i == 6 {
                                row = min_row;
                            }
                        },
                        b'B' => {
                            min_row = ((min_row + max_row) / 2) + 1;
                            if i == 6 {
                                row = max_row;
                            }
                        },
                        _ => {},
                    }
                }

                let mut min_col: u64 = 0;
                let mut max_col: u64 = 7;
                let mut col: u64 = 0;

                for (i, &ch) in chars[7..].iter().enumerate() {
                    match ch {
                        b'L' => {
                            max_col = (min_col + max_col) / 2;
                            if i == 2 {
                                col = min_col;
                            }
                        },
                        b'R' => {
                            min_col = ((min_col + max_col) / 2) + 1;
                            if i == 2 {
                                col = max_col;
                            }
                        },
                        _ => {},
                    }
                }

                let seat_id = row * 8 + col;
                if seat_id > highest_seat_id {
                    highest_seat_id = seat_id;
                }
            }

            println!("Highest Seat ID: {highest_seat_id}");
        },
        Err(err) => panic!("{}", err),
    }
}

fn part2() {
    match fs::read_to_string(FILE_PATH) {
        Ok(text) => {
            let mut seat_ids: Vec<u64> = Vec::new();

            let mut lines: Vec<&str> = text.split('\n').collect();
            lines.pop();

            for &line in &lines {
                let mut min_row: u64 = 0;
                let mut max_row: u64 = 127;
                let mut row: u64 = 0;

                let chars: &[u8] = line.as_bytes();
                for (i, &ch) in chars[0..7].iter().enumerate() {
                    match ch {
                        b'F' => {
                            max_row = (min_row + max_row) / 2;
                            if i == 6 {
                                row = min_row;
                            }
                        },
                        b'B' => {
                            min_row = ((min_row + max_row) / 2) + 1;
                            if i == 6 {
                                row = max_row;
                            }
                        },
                        _ => {},
                    }
                }

                let mut min_col: u64 = 0;
                let mut max_col: u64 = 7;
                let mut col: u64 = 0;

                for (i, &ch) in chars[7..].iter().enumerate() {
                    match ch {
                        b'L' => {
                            max_col = (min_col + max_col) / 2;
                            if i == 2 {
                                col = min_col;
                            }
                        },
                        b'R' => {
                            min_col = ((min_col + max_col) / 2) + 1;
                            if i == 2 {
                                col = max_col;
                            }
                        },
                        _ => {},
                    }
                }

                seat_ids.push(row * 8 + col);
            }

            seat_ids.sort();

            for i in 0..seat_ids.len()-1 {
                if seat_ids[i+1] - seat_ids[i] == 2 {
                    println!("Your Seat ID: {}", (seat_ids[i] + seat_ids[i+1]) / 2);
                }
            }
        },
        Err(err) => panic!("{}", err),
    }
}

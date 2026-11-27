use std::fs;

// const FILE_PATH: &str = "data/temp.txt";
const FILE_PATH: &str = "data/org.txt";

// symbols
const FLOOR: u8 = b'.';
const EMPTY: u8 = b'L';
const OCCUPIED: u8 = b'#';

#[derive(Clone)]
struct FloorPlan {
    pub seats: Vec<u8>,
    pub rows: usize,
    pub cols: usize,
}

impl FloorPlan {
    pub fn new(lines: &Vec<String>) -> FloorPlan {
        let mut seats = Vec::with_capacity(lines.len() * lines[0].len());
        for line in lines {
            for ch in line.bytes() {
                seats.push(ch);
            }
        }

        FloorPlan {
            seats,
            rows: lines.len(),
            cols: lines[0].len(),
        }
    }

    fn count_occupied_seats(&self, row: usize, col: usize) -> u64 {
        let mut occupied_seat_count: u64 = 0;

        for row_offset in -1isize..2 {
            for col_offset in -1isize..2 {
                if row_offset == 0 && col_offset == 0 {
                    continue;
                }

                let new_row = (row as isize) + row_offset;
                let new_col = (col as isize) + col_offset;

                if new_row < 0 || new_row >= self.rows as isize || new_col < 0 || new_col >= self.cols as isize {
                    continue;
                }

                match self.seats[(new_row as usize) * self.cols + (new_col as usize)] {
                    OCCUPIED => {
                        occupied_seat_count += 1;
                    },
                    _ => {},
                }
            }
        }

        occupied_seat_count
    }
    
    fn count_occupied_seats_advanced(&self, row: usize, col: usize) -> u64 {
        let mut occupied_seat_count: u64 = 0;

        let steps = [
            (-1isize, 0isize),
            (-1isize, 1isize),
            (0isize, 1isize),
            (1isize, 1isize),
            (1isize, 0isize),
            (1isize, -1isize),
            (0isize, -1isize),
            (-1isize, -1isize),
        ];

        for step in &steps {
            let mut row_offset = step.0;
            let mut col_offset = step.1;

            loop {
                let new_row = (row as isize) + row_offset;
                let new_col = (col as isize) + col_offset;

                if new_row < 0 || new_row >= self.rows as isize || new_col < 0 || new_col >= self.cols as isize {
                    break;
                }

                match self.seats[(new_row as usize) * self.cols + (new_col as usize)] {
                    OCCUPIED => {
                        occupied_seat_count += 1;
                        break;
                    },
                    EMPTY => {
                        break;
                    },
                    _ => {},
                }

                row_offset += step.0;
                col_offset += step.1;
            }
        }

        occupied_seat_count
    }

    pub fn re_arrange_seats(&mut self) {
        let mut seats = self.seats.clone();

        for row in 0..self.rows {
            for col in 0..self.cols {
                let count = self.count_occupied_seats(row, col);
                match self.seats[row * self.cols + col] {
                    EMPTY => {
                        if count == 0 {
                            seats[row * self.cols + col] = OCCUPIED;
                        }
                    },
                    OCCUPIED => {
                        if count >= 4 {
                            seats[row * self.cols + col] = EMPTY;
                        }
                    },
                    _ => {},
                }
            }
        }

        self.seats = seats;
    }

    pub fn re_arrange_seats_advanced(&mut self) {
        let mut seats = self.seats.clone();

        for row in 0..self.rows {
            for col in 0..self.cols {
                let count = self.count_occupied_seats_advanced(row, col);
                match self.seats[row * self.cols + col] {
                    EMPTY => {
                        if count == 0 {
                            seats[row * self.cols + col] = OCCUPIED;
                        }
                    },
                    OCCUPIED => {
                        if count >= 5 {
                            seats[row * self.cols + col] = EMPTY;
                        }
                    },
                    _ => {},
                }
            }
        }

        self.seats = seats;
    }
}

fn main() {
    let lines = read_lines();
    let floor_plan = FloorPlan::new(&lines);
    part1(floor_plan.clone());
    part2(floor_plan);
}

fn read_lines() -> Vec<String> {
    match fs::read_to_string(FILE_PATH) {
        Ok(text) => {
            let mut lines = text
                .split('\n')
                .map(|line| line.to_string())
                .collect::<Vec<String>>();
            lines.pop();
            return lines;
        },
        Err(err) => panic!("{}", err),
    }
}

fn part1(mut floor_plan: FloorPlan) {
    let mut seats = floor_plan.seats.clone();

    loop {
        floor_plan.re_arrange_seats();
        if seats == floor_plan.seats {
            break;
        }
        seats = floor_plan.seats.clone();
    }

    let mut occupied_seat_count: u64 = 0;
    for row in 0..floor_plan.rows {
        for col in 0..floor_plan.cols {
            if seats[row * floor_plan.cols + col] == OCCUPIED {
                occupied_seat_count += 1;
            }
        }
    }

    println!("[PART 1]: {occupied_seat_count}");
}

fn part2(mut floor_plan: FloorPlan) {
    let mut seats = floor_plan.seats.clone();

    loop {
        floor_plan.re_arrange_seats_advanced();
        if seats == floor_plan.seats {
            break;
        }
        seats = floor_plan.seats.clone();
    }

    let mut occupied_seat_count: u64 = 0;
    for row in 0..floor_plan.rows {
        for col in 0..floor_plan.cols {
            if seats[row * floor_plan.cols + col] == OCCUPIED {
                occupied_seat_count += 1;
            }
        }
    }

    println!("[PART 2]: {occupied_seat_count}");
}

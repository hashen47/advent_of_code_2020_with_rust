use std::fs;

// const FILE_PATH: &str = "data/temp.txt";
const FILE_PATH: &str = "data/org.txt";

fn main() {
    let lines = read_lines();
    part1(&lines);
    part2(&lines);
}

fn read_lines() -> Vec<String> {
    match fs::read_to_string(FILE_PATH) {
        Ok(text) => {
            let mut lines = text.split('\n')
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            lines.pop();
            return lines;
        },
        Err(err) => panic!("{}", err),
    }
}

fn part1(lines: &Vec<String>) {
    #[derive(Debug)]
    pub struct Timestamp {
        pub n: u64,
        pub total_wait_time: u64,
    }

    let n: u64 = lines[0].parse().expect("should be a number");
    let mut bus_timestamps: Vec<Timestamp> = Vec::new();

    for text in lines[1].split(',') {
        if text == "x" {
            continue
        }
        let x = text.parse().expect("should be a number");
        let remainder = n % x;
        let mut total_wait_time = n + x - remainder;
        if remainder == 0 {
            total_wait_time = n;
        }
        
        bus_timestamps.push(Timestamp {
            n: x,
            total_wait_time, 
        });
    }

    bus_timestamps.sort_by(|a, b| a.total_wait_time.cmp(&b.total_wait_time));
    println!("[PART 1] {}", bus_timestamps[0].n * (bus_timestamps[0].total_wait_time - n));
}

fn part2(lines: &Vec<String>) {
    #[derive(Debug)]
    pub struct Timestamp {
        pub bus_id: u64,
        pub depart_after: u64, // minutes
    }

    let mut bus_timestamps: Vec<Timestamp> = Vec::new();

    let mut depart_after: u64 = 0;
    for text in lines[1].split(',') {
        if text == "" {
            break;
        }

        if text == "x" {
            depart_after += 1;
            continue
        }

        let x = text.parse().expect("should be a number");
        bus_timestamps.push(Timestamp {
            bus_id: x,
            depart_after, 
        });
        depart_after += 1;
    }

    let mut x: u64 = 0;
    let mut _lcm: u64 = bus_timestamps[0].bus_id;
    for i in  0..bus_timestamps.len()-1 {
        let r1: u64 = if bus_timestamps[i].depart_after == 0 {
            0
        } else {
            bus_timestamps[i].bus_id - (bus_timestamps[i].depart_after % bus_timestamps[i].bus_id)
        };

        let r2: u64 = if bus_timestamps[i+1].depart_after == 0 {
            0
        } else {
            bus_timestamps[i+1].bus_id - (bus_timestamps[i+1].depart_after % bus_timestamps[i+1].bus_id)
        };

        loop {
            if x % bus_timestamps[i].bus_id == r1 && x % bus_timestamps[i+1].bus_id == r2 {
                _lcm = lcm(_lcm, bus_timestamps[i+1].bus_id); 
                break;
            }
            x += _lcm;
        }
    }

    println!("[PART 2] {x}");
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }
    (a / gcd(a, b)) * b
}

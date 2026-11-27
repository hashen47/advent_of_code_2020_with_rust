use std::fs;

// const FILE_PATH: &str = "data/temp.txt";
const FILE_PATH: &str = "data/org.txt";

#[derive(Debug)]
pub enum Instruction {
    N(f64),
    S(f64),
    E(f64),
    W(f64),
    L(f64),
    R(f64),
    F(f64),
}

fn main() {
    let instructions = lines_to_instructions(&read_lines());
    // println!("{instructions:#?}");
    execute_instructions_part_1(&instructions);
    execute_instructions_part_2(&instructions);
}

fn read_lines() -> Vec<String> {
    match fs::read_to_string(FILE_PATH) {
        Ok(text) => {
            let mut lines = text.split('\n')
                .map(|line| line.to_string())
                .collect::<Vec<String>>();
            lines.pop();
            return lines;
        },
        Err(err) => panic!("{}", err),
    }
}

fn lines_to_instructions(lines: &Vec<String>) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::with_capacity(lines.len());

    for line in lines {
        let bytes = line.bytes().collect::<Vec<u8>>();
        match bytes[0] {
            b'N' => {
                instructions.push(
                    Instruction::N(
                        std::str::from_utf8(&bytes[1..])
                            .unwrap()
                            .parse()
                            .expect("should be a number")
                    )
                );
            },
            b'S' => {
                instructions.push(
                    Instruction::S(
                        std::str::from_utf8(&bytes[1..])
                            .unwrap()
                            .parse()
                            .expect("should be a number")
                    )
                );
            },
            b'E' => {
                instructions.push(
                    Instruction::E(
                        std::str::from_utf8(&bytes[1..])
                            .unwrap()
                            .parse()
                            .expect("should be a number")
                    )
                );
            },
            b'W' => {
                instructions.push(
                    Instruction::W(
                        std::str::from_utf8(&bytes[1..])
                            .unwrap()
                            .parse()
                            .expect("should be a number")
                    )
                );
            },
            b'L' => {
                instructions.push(
                    Instruction::L(
                        std::str::from_utf8(&bytes[1..])
                            .unwrap()
                            .parse()
                            .expect("should be a number")
                    )
                );
            },
            b'R' => {
                instructions.push(
                    Instruction::R(
                        std::str::from_utf8(&bytes[1..])
                            .unwrap()
                            .parse()
                            .expect("should be a number")
                    )
                );
            },
            b'F' => {
                instructions.push(
                    Instruction::F(
                        std::str::from_utf8(&bytes[1..])
                            .unwrap()
                            .parse()
                            .expect("should be a number")
                    )
                );
            },
            _ => panic!("invalid instruction: {}", bytes[0]),
        }
    }

    instructions
}

fn execute_instructions_part_1(instructions: &Vec<Instruction>) {
    let mut north: f64 = 0f64;
    let mut east: f64 = 0f64;
    let mut angle: f64 = 90f64; // east

    for instruction in instructions {
        match instruction {
            Instruction::N(v) => {
                north += v;
            },
            Instruction::S(v) => {
                north -= v;
            },
            Instruction::E(v) => {
                east += v;
            },
            Instruction::W(v) => {
                east -= v;
            },
            Instruction::L(v) => {
                angle -= v;
            },
            Instruction::R(v) => {
                angle += v;
            },
            Instruction::F(v) => {
                north += angle.to_radians().cos() * v;
                east += angle.to_radians().sin() * v;
            },
        }
    }

    println!("[PART 1] {:.0}", north.abs() + east.abs());
}

fn execute_instructions_part_2(instructions: &Vec<Instruction>) {
    let mut waypoint_e: f64 = 10f64;
    let mut waypoint_n: f64 = 1f64;
    
    let mut north: f64 = 0f64;
    let mut east: f64 = 0f64;

    for instruction in instructions {
        match instruction {
            Instruction::N(v) => {
                waypoint_n += v;
            },
            Instruction::S(v) => {
                waypoint_n -= v;
            },
            Instruction::E(v) => {
                waypoint_e += v;
            },
            Instruction::W(v) => {
                waypoint_e -= v;
            },
            Instruction::L(v) => {
                let waypoint_e_copy = waypoint_e;
                let waypoint_n_copy = waypoint_n;
                waypoint_e = waypoint_e_copy * v.to_radians().cos() - waypoint_n_copy * v.to_radians().sin();
                waypoint_n = waypoint_e_copy * v.to_radians().sin() + waypoint_n_copy * v.to_radians().cos();
            },
            &Instruction::R(mut v) => {
                let waypoint_e_copy = waypoint_e;
                let waypoint_n_copy = waypoint_n;
                v *= -1f64;
                waypoint_e = waypoint_e_copy * v.to_radians().cos() - waypoint_n_copy * v.to_radians().sin();
                waypoint_n = waypoint_e_copy * v.to_radians().sin() + waypoint_n_copy * v.to_radians().cos();
            },
            Instruction::F(v) => {
                north += waypoint_n * v;
                east += waypoint_e * v;
            },
        }
    }

    println!("[PART 2] {:.0}", north.abs() + east.abs());
}

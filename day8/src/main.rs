use std::fs;
use std::collections::HashSet;

// const FILE_PATH: &str = "data/temp.txt";
const FILE_PATH: &str = "data/org.txt";

#[derive(Clone, Debug)]
enum OpCode {
    NOP(i32),
    JMP(i32),
    ACC(i32),
}

fn main() {
    part1();
    part2();
}

fn part1() {
    match fs::read_to_string(FILE_PATH) {
        Ok(text) => {
            let mut instructions: Vec<OpCode> = Vec::new();
            let mut lines = text.split('\n').collect::<Vec<&str>>();
            lines.pop();

            // parse instructions
            for line in lines {
                let parts = line.split(' ').collect::<Vec<&str>>();
                match parts[0] {
                    "nop" => {
                        let n = parts[1].parse().expect("should be a number");
                        instructions.push(OpCode::NOP(n));
                    },
                    "jmp" => {
                        let n = parts[1].parse().expect("should be a number");
                        instructions.push(OpCode::JMP(n));
                    },
                    "acc" => {
                        let n = parts[1].parse().expect("should be a number");
                        instructions.push(OpCode::ACC(n));
                    },
                    _ => panic!("invalid instruction: {}", parts[0]),
                }
            }

            // execute instructions
            let mut acc: i32 = 0;
            let mut executed_line_index: HashSet<usize> = HashSet::new();

            let mut index: usize = 0;
            loop {
                if index >= instructions.len() {
                    break;
                }
                
                let inst = &instructions[index];

                if executed_line_index.contains(&index) {
                    break;
                }
                executed_line_index.insert(index);

                match inst {
                    OpCode::NOP(_) => {
                        index += 1;
                    },
                    OpCode::JMP(val) => {
                        match i32::try_from(index) {
                            Ok(mut i) => {
                                if *val < 0 && val.abs() > i {
                                    panic!("index cannot be negative, index: {}, jmp instruction val: {}", i, val);
                                }
                                i += *val;
                                index = i.try_into().expect("index cannot be negative");
                            },
                            Err(err) => panic!("usize to i32 conversion fail {}", err),
                        }
                    },
                    OpCode::ACC(val) => {
                        acc += val;
                        index += 1;
                    },
                }
            }

            println!("[PART 1]: {acc}");
        },
        Err(err) => panic!("{}", err),
    };
}

fn part2() {
    match fs::read_to_string(FILE_PATH) {
        Ok(text) => {
            let mut instructions: Vec<OpCode> = Vec::new();
            let mut nop_or_jmp_instruction_indexes: Vec<usize> = Vec::new();
            let mut lines = text.split('\n').collect::<Vec<&str>>();
            lines.pop();

            // parse instructions
            for (i, line) in lines.iter().enumerate() {
                let parts = line.split(' ').collect::<Vec<&str>>();
                match parts[0] {
                    "nop" => {
                        let n = parts[1].parse().expect("should be a number");
                        instructions.push(OpCode::NOP(n));
                        nop_or_jmp_instruction_indexes.push(i);
                    },
                    "jmp" => {
                        let n = parts[1].parse().expect("should be a number");
                        instructions.push(OpCode::JMP(n));
                        nop_or_jmp_instruction_indexes.push(i);
                    },
                    "acc" => {
                        let n = parts[1].parse().expect("should be a number");
                        instructions.push(OpCode::ACC(n));
                    },
                    _ => panic!("invalid instruction: {}", parts[0]),
                }
            }

            // execute instructions
            for nop_or_jmp_index in nop_or_jmp_instruction_indexes {
                let mut is_execution_complete: bool = false;
                let mut acc: i32 = 0;
                let mut executed_line_index: HashSet<usize> = HashSet::new();

                let mut index: usize = 0;
                loop {
                    if index >= instructions.len() {
                        is_execution_complete = true;
                        break;
                    }

                    if executed_line_index.contains(&index) {
                        break;
                    }

                    let mut inst = instructions[index].clone();
                    executed_line_index.insert(index);

                    if index == nop_or_jmp_index {
                        match inst {
                            OpCode::NOP(val) => {
                                inst = OpCode::JMP(val);
                            },
                            OpCode::JMP(val) => {
                                inst = OpCode::NOP(val);
                            },
                            _ => {},
                        }
                    }

                    match inst {
                        OpCode::NOP(_) => {
                            index += 1;
                        },
                        OpCode::JMP(val) => {
                            match i32::try_from(index) {
                                Ok(mut i) => {
                                    if val < 0 && val.abs() > i {
                                        panic!("index cannot be negative, index: {}, jmp instruction val: {}", i, val);
                                    }
                                    i += val;
                                    index = i.try_into().expect("index cannot be negative");
                                },
                                Err(err) => panic!("usize to i32 conversion fail {}", err),
                            }
                        },
                        OpCode::ACC(val) => {
                            acc += val;
                            index += 1;
                        },
                    }
                }

                if is_execution_complete {
                    println!("[PART 2]: {acc}");
                    break;
                }
            }
        },
        Err(err) => panic!("{}", err),
    };
}

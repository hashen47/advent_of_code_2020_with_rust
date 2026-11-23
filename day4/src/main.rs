use std::fs;

// const FILE_PATH: &str = "data/temp.txt";
const FILE_PATH: &str = "data/org.txt";

fn main() {
    part1();
    part2();
}


fn part1() {
    // byr (Birth Year)
    // iyr (Issue Year)
    // eyr (Expiration Year)
    // hgt (Height)
    // hcl (Hair Color)
    // ecl (Eye Color)
    // pid (Passport ID)
    // cid (Country ID) (this field is not required)

    struct Passport<'a> {
        byr: Option<&'a str>,
        iyr: Option<&'a str>,
        eyr: Option<&'a str>,
        hgt: Option<&'a str>,
        hcl: Option<&'a str>,
        ecl: Option<&'a str>,
        pid: Option<&'a str>,
        cid: Option<&'a str>,

    }

    impl<'a> Passport<'a> {
        pub fn new() -> Passport<'a> {
            Passport {
                byr: None,
                iyr: None,
                eyr: None,
                hgt: None,
                hcl: None,
                ecl: None,
                pid: None,
                cid: None,
            }
        }

        pub fn set_fields(&mut self, text: &'a str) {
            for partition in text.split(' ') {
                let key_value = partition.split(':').collect::<Vec<&str>>();
                match key_value[0] {
                    "byr" => {
                        self.byr = Some(key_value[1]);
                    },
                    "iyr" => {
                        self.iyr = Some(key_value[1]);
                    },
                    "eyr" => {
                        self.eyr = Some(key_value[1]);
                    },
                    "hgt" => {
                        self.hgt = Some(key_value[1]);
                    },
                    "hcl" => {
                        self.hcl = Some(key_value[1]);
                    },
                    "ecl" => {
                        self.ecl = Some(key_value[1]);
                    },
                    "pid" => {
                        self.pid = Some(key_value[1]);
                    },
                    "cid" => {
                        self.cid = Some(key_value[1]);
                    },
                    _ => {},
                }
            }
        }

        pub fn validate(&self) -> bool {
            let Some(_) = self.byr else {
                return false;
            };

            let Some(_) = self.iyr else {
                return false;
            };

            let Some(_) = self.eyr else {
                return false;
            };

            let Some(_) = self.hgt else {
                return false;
            };

            let Some(_) = self.hcl else {
                return false;
            };

            let Some(_) = self.hcl else {
                return false;
            };

            let Some(_) = self.ecl else {
                return false;
            };

            let Some(_) = self.pid else {
                return false;
            };

            true
        }
    }

    match fs::read_to_string(FILE_PATH) {
        Ok(text) => {
            let lines = text.split('\n').collect::<Vec<&str>>();

            let mut valid_passport_count: u64 = 0;

            let mut passport = Passport::new();
            for line in lines {
                if line == "" {
                    if passport.validate() {
                        valid_passport_count += 1;
                    }
                    passport = Passport::new();
                } else {
                    passport.set_fields(line);
                }
            }

            println!("[PART 1] valid passport count: {}", valid_passport_count);
        },
        Err(err) => panic!("{}", err),
    }
}

fn part2() {
    // byr (Birth Year)
    // iyr (Issue Year)
    // eyr (Expiration Year)
    // hgt (Height)
    // hcl (Hair Color)
    // ecl (Eye Color)
    // pid (Passport ID)
    // cid (Country ID) (this field is not required)

    struct Passport<'a> {
        byr: Option<&'a str>,
        iyr: Option<&'a str>,
        eyr: Option<&'a str>,
        hgt: Option<&'a str>,
        hcl: Option<&'a str>,
        ecl: Option<&'a str>,
        pid: Option<&'a str>,
        cid: Option<&'a str>,

    }

    impl<'a> Passport<'a> {
        pub fn new() -> Passport<'a> {
            Passport {
                byr: None,
                iyr: None,
                eyr: None,
                hgt: None,
                hcl: None,
                ecl: None,
                pid: None,
                cid: None,
            }
        }

        pub fn set_fields(&mut self, text: &'a str) {
            for partition in text.split(' ') {
                let key_value = partition.split(':').collect::<Vec<&str>>();
                match key_value[0] {
                    "byr" => {
                        self.byr = Some(key_value[1]);
                    },
                    "iyr" => {
                        self.iyr = Some(key_value[1]);
                    },
                    "eyr" => {
                        self.eyr = Some(key_value[1]);
                    },
                    "hgt" => {
                        self.hgt = Some(key_value[1]);
                    },
                    "hcl" => {
                        self.hcl = Some(key_value[1]);
                    },
                    "ecl" => {
                        self.ecl = Some(key_value[1]);
                    },
                    "pid" => {
                        self.pid = Some(key_value[1]);
                    },
                    "cid" => {
                        self.cid = Some(key_value[1]);
                    },
                    _ => {},
                }
            }
        }

        pub fn validate(&self) -> bool {
            if let Some(byr) = self.byr {
                let byr: i32 = byr
                    .parse()
                    .expect("should be a number");
                if byr < 1920 || byr > 2002 {
                    return false;
                }
            } else {
                return false;
            }

            if let Some(iyr) = self.iyr {
                let iyr: i32 = iyr
                    .parse()
                    .expect("should be a number");
                if iyr < 2010 || iyr > 2020 {
                    return false;
                }
            } else {
                return false;
            }

            if let Some(eyr) = self.eyr {
                let eyr: i32 = eyr
                    .parse()
                    .expect("should be a number");
                if eyr < 2020 || eyr > 2030 {
                    return false;
                }
            } else {
                return false;
            };

            if let Some(hgt) = self.hgt {
                let chars = hgt.chars().collect::<Vec<char>>();
                if &chars[chars.len()-2..].iter().collect::<String>() == "cm" {
                    let hgt: Option<u32> = chars[0..chars.len()-2]
                        .into_iter()
                        .try_fold(0u32, |acc, &x| x.to_digit(10).map(|d| acc * 10 + d));
                    if let Some(hgt) = hgt {
                        if hgt < 150 || hgt > 193 {
                            return false;
                        }
                    } else {
                        return false;
                    }
                } else if &chars[chars.len()-2..].iter().collect::<String>() == "in" {
                    let hgt: Option<u32> = chars[0..chars.len()-2]
                        .into_iter()
                        .try_fold(0u32, |acc, &x| x.to_digit(10).map(|d| acc * 10 + d));
                    if let Some(hgt) = hgt {
                        if hgt < 59 || hgt > 76 {
                            return false;
                        }
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                return false;
            }

            if let Some(hcl) = self.hcl {
                let chars = hcl.chars().collect::<Vec<char>>();
                if &chars[0..1].iter().collect::<String>() == "#" {
                    if chars.len() == 7 {
                        for &ch in &chars[1..] {
                            let mut is_ok = false;
                            for valid_ch in ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'] {
                                if ch == valid_ch {
                                    is_ok = true;
                                    break;
                                }
                            }
                            if !is_ok {
                                return false;
                            }
                        }
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                return false;
            }

            if let Some(ecl) = self.ecl {
                let mut is_ok = false;
                for word in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"] {
                    if word == ecl {
                        is_ok = true;
                        break;
                    }
                }
                if !is_ok {
                    return false;
                }
            } else {
                return false;
            }

            if let Some(pid) = self.pid {
                if pid.len() != 9 {
                    return false;
                }

                for ch in pid.chars() {
                    let mut is_ok = false;
                    for valid_ch in ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'] {
                        if ch == valid_ch {
                            is_ok = true;
                            break;
                        }
                    }
                    if !is_ok {
                        return false;
                    }
                }
            } else {
                return false;
            }

            true
        }
    }

    match fs::read_to_string(FILE_PATH) {
        Ok(text) => {
            let lines = text.split('\n').collect::<Vec<&str>>();

            let mut valid_passport_count: u64 = 0;

            let mut passport = Passport::new();
            for line in lines {
                if line == "" {
                    if passport.validate() {
                        valid_passport_count += 1;
                    }
                    passport = Passport::new();
                } else {
                    passport.set_fields(line);
                }
            }

            println!("[PART 2] valid passport count: {}", valid_passport_count);
        },
        Err(err) => panic!("{}", err),
    }
}

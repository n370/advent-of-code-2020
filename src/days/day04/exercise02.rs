use std::{ fs };

pub fn main() -> Option<bool> {
    let file_contents = match fs::read_to_string(
        "./inputs/2020-12-04-aoc-01-input.txt"
    ) {
        Ok(c) => c,
        Err(e) => panic!("{:?}", e)
    };

    let (
        valid_passports, invalid_passports
    ): (Vec<Passport>, Vec<Passport>) = file_contents
        .split("\n\n")
        .map(|block| Passport::new(block))
        .partition(|passport| {
            passport.is_valid()
        });

    println!(
        "There are {} valid passports and {} invalid passports",
        valid_passports.len(), invalid_passports.len()
    );

    Some(true)
}

#[derive(Debug)]
struct Passport {
    byr: u16,
    iyr: u16,
    eyr: u16,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: u16,
}

impl Passport {
    pub fn new(raw: &str) -> Passport {
        raw
            .split_whitespace()
            .fold(Passport {
                byr: 0,
                iyr: 0,
                eyr: 0,
                hgt: String::from(""),
                hcl: String::from(""),
                ecl: String::from(""),
                pid: String::from(""),
                cid: 0,
            }, |mut passport, field| {
                let collection: Vec<_> = field.split(':').collect();
                match collection[0] {
                    "byr" => {
                        if collection[1].len() == 4 {
                            let val = collection[1].parse::<u16>().unwrap();
                            if val >= 1920 && val <= 2002 {
                                passport.byr = val
                            }
                        }
                        passport
                    },
                    "iyr" => {
                        if collection[1].len() == 4 {
                            let val = collection[1].parse::<u16>().unwrap();
                            if val >= 2010 && val <= 2020 {
                                passport.iyr = val
                            }
                        }
                        passport
                    },
                    "eyr" => {
                        if collection[1].len() == 4 {
                            let val = collection[1].parse::<u16>().unwrap();
                            if val >= 2020 && val <= 2030 {
                                passport.eyr = val
                            }
                        }
                        passport
                    },
                    "hgt" => {
                        let metric = collection[1].contains("cm");
                        let imperial = collection[1].contains("in");
                        if metric {
                            let val = match collection[1].split("cm").nth(0) {
                                Some(s) => s.parse::<u16>().unwrap(),
                                None => panic!("Not a metric height here!")
                            };
                            if val >= 150 && val <= 193 {
                                passport.hgt = collection[1].to_string();
                            }
                        } else if imperial {
                            let val = match collection[1].split("in").nth(0) {
                                Some(s) => s.parse::<u16>().unwrap(),
                                None => panic!("Not an imperial height here!")
                            };
                            if val >= 59 && val <= 76 {
                                passport.hgt = collection[1].to_string();
                            }
                        }
                        passport
                    },
                    "hcl" => {
                        if collection[1].starts_with('#') {
                            if collection[1]
                                .split('#')
                                .nth(1)
                                .unwrap()
                                .chars()
                                .filter(|c| {
                                    vec!(
                                        '0','1','2','3','4','5','6',
                                        '7','8','9','a','b','c','d',
                                        'e','f','A','B','C','D','E',
                                        'F'
                                    ).contains(c)
                                }).count() == 6 {
                                    passport.hcl = collection[1].to_string();
                                }
                        }
                        passport
                    },
                    "ecl" => {
                        if vec!("amb", "blu", "brn", "gry", "grn", "hzl", "oth")
                            .contains(&collection[1]) {
                                passport.ecl = collection[1].to_string();
                        }
                        passport
                    },
                    "pid" => {
                        passport.pid = match collection[1].parse::<u32>() {
                            Ok(_) => {
                                if collection[1].len() == 9 {
                                    collection[1].to_string()
                                } else {
                                    String::from("")
                                }
                            },
                            Err(_) => String::from("")
                        };
                        passport
                    },
                    "cid" => {
                        passport.cid = collection[1].parse::<u16>().unwrap();
                        passport
                    },
                    _ => passport
                }
            })
    }

    pub fn is_valid(&self) -> bool {
        if
            self.byr != 0
            && self.iyr != 0
            && self.eyr != 0
            && self.hgt != ""
            && self.hcl != ""
            && self.ecl != ""
            && self.pid != ""
        {
            true
        } else {
            false
        }
    }
}

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
                        passport.byr = collection[1].parse::<u16>().unwrap();
                        passport
                    },
                    "iyr" => {
                        passport.iyr = collection[1].parse::<u16>().unwrap();
                        passport
                    },
                    "eyr" => {
                        passport.eyr = collection[1].parse::<u16>().unwrap();
                        passport
                    },
                    "hgt" => {
                        passport.hgt = String::from(collection[1]);
                        passport
                    },
                    "hcl" => {
                        passport.hcl = String::from(collection[1]);
                        passport
                    },
                    "ecl" => {
                        passport.ecl = String::from(collection[1]);
                        passport
                    },
                    "pid" => {
                        passport.pid = String::from(collection[1]);
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

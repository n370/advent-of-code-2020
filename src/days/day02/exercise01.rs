use std::{ fs };

pub fn main() {
    let contents = match fs::read_to_string(
        "./inputs/2020-12-02-aoc-01-input.txt"
    ) {
        Ok(contents) => contents,
        Err(e) => panic!("{:?}", e)
    };

    let entries = contents.split('\n').filter(|e| e.len() > 0);
    let mut valid_pwds = vec![];

    for entry in entries {
        let entry_sections = entry.split_whitespace();
        let mut min = 0;
        let mut max = 0;
        let mut letter: char = ' ';
        let mut pwd: &str = "";

        for section in entry_sections {
            if section.contains('-') {
                min = match section.split('-').nth(0) {
                    Some(s) => match s.parse::<u16>() {
                        Ok(num) => num,
                        Err(e) => panic!("{:?}", e)
                    },
                    None => panic!("No min found!")
                };

                max = match section.split('-').nth(1) {
                    Some(s) => match s.parse::<u16>() {
                        Ok(num) => num,
                        Err(e) => panic!("{:?}", e)
                    },
                    None => panic!("No max found!")
                };
            } else if section.contains(':') {
                letter = match section.chars().nth(0) {
                    Some(c) => c,
                    None => panic!("No letter here!")
                };
            } else {
                pwd = section;
            }
        }

        let mut count: u16 = 0;

        for slot in pwd.chars() {
            if slot == letter {
                count += 1
            }
        }

        println!("\n----");
        println!("Entry: {}", entry);
        println!("Min: {}, Max: {}, Letter: {}, Pwd: {}", min, max, letter, pwd);
        println!("Letter count: {}", count);
        println!("----\n");

        if count <= max && count >= min {
            valid_pwds.push(pwd);
        }
    }

    println!("There are {} valid passwords", valid_pwds.len());
}

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
        let mut first_position = 0;
        let mut second_position = 0;
        let mut letter: char = ' ';
        let mut pwd: &str = "";

        for section in entry_sections {
            if section.contains('-') {
                first_position = match section.split('-').nth(0) {
                    Some(s) => match s.parse::<usize>() {
                        Ok(num) => num - 1,
                        Err(e) => panic!("{:?}", e)
                    },
                    None => panic!("No first position found!")
                };

                second_position = match section.split('-').nth(1) {
                    Some(s) => match s.parse::<usize>() {
                        Ok(num) => num - 1,
                        Err(e) => panic!("{:?}", e)
                    },
                    None => panic!("No second position found!")
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

        println!("\n----");
        println!("Entry: {}", entry);
        println!("
            First position: {}, Second position: {}, Letter: {}, Pwd: {}",
            first_position, second_position, letter, pwd
        );

        let (first_position_letter, second_position_letter) = {
            (
                match pwd.chars().nth(first_position) {
                    Some(n) => n,
                    None => panic!("No letter at first position!")
                },
                match pwd.chars().nth(second_position) {
                    Some(n) => n,
                    None => panic!("No letter at second position!")
                }
            )
        };

        if (first_position_letter == letter && second_position_letter != letter)
            || (first_position_letter != letter && second_position_letter == letter) {
                valid_pwds.push(pwd);
                println!("Valid!");
        } else {
            println!("Invalid!");
        }

        println!("----\n");
    }

    println!("There are {} valid passwords", valid_pwds.len());
}

use std::{ fs };

pub fn main() {
    let contents = match fs::read_to_string(
        "./inputs/2020-12-01-aoc-01-input.txt"
    ) {
        Ok(contents) => contents,
        Err(e) => panic!("{:?}", e)
    };

    let numbers = &contents.split_whitespace();

    for first_number_str in numbers.clone() {
        for second_number_str in numbers.clone() {
            for third_number_str in numbers.clone() {
                let (first_number, second_number, third_number) = {
                    (
                        match first_number_str.parse::<i32>() {
                            Ok(num) => num,
                            Err(e) => panic!("{:?}", e)
                        },
                        match second_number_str.parse::<i32>() {
                            Ok(num) => num,
                            Err(e) => panic!("{:?}", e)
                        },
                        match third_number_str.parse::<i32>() {
                            Ok(num) => num,
                            Err(e) => panic!("{:?}", e)
                        },
                    )
                };
                if first_number + second_number + third_number == 2020 {
                    return println!("{}", first_number * second_number * third_number);
                }
            }
        }
    }
}

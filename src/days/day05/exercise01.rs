use std::{ fs };

pub fn main() -> Option<bool> {
    let file_contents = match fs::read_to_string(
        "./inputs/2020-12-05-aoc-01-input.txt"
    ) {
        Ok(c) => c,
        Err(e) => panic!("{:?}", e)
    };

    let largest_id = file_contents
        .split('\n')
        .map(|line| {
            let result = get_row_and_column(line);
            result.0 * 8 + result.1
        })
        .max()?;

    println!("The largest id is: {}", largest_id);

    Some(true)
}

fn get_row_and_column(line: &str) -> (usize, usize) {
    let result = line
        .chars()
        .enumerate()
        .fold(((0, 127), (0, 7)), |mut result, (index, val)| {
            if index < 6 {
                let half = (result.0.1 - result.0.0) / 2;
                if val == 'F' {
                    result.0 = (result.0.0, result.0.0 + half);
                } else {
                    result.0 = (result.0.0 + half + 1, result.0.1);
                }
            } else if index == 6 {
                if val == 'F' {
                    result.0 = (result.0.0, 0);
                } else {
                    result.0 = (result.0.1, 0);
                }
            } else if index > 6 && index < 9 {
                let half = (result.1.1 - result.1.0) / 2;
                if val == 'L' {
                    result.1 = (result.1.0, result.1.0 + half);
                } else {
                    result.1 = (result.1.0 + half + 1, result.1.1);
                }
            } else {
                if val == 'L' {
                    result.1 = (result.1.0, 0);
                } else {
                    result.1 = (result.1.1, 0);
                }
            }
            result
        });
    (result.0.0, result.1.0)
}

#[test]
fn it_works() {
    assert_eq!(get_row_and_column("FBFBBFFRLR"), (44, 5));
}

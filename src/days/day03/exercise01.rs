use std::{ fs };

pub fn main() {
    let file_contents = match fs::read_to_string(
        "./inputs/2020-12-03-aoc-01-input.txt"
    ) {
        Ok(c) => c,
        Err(e) => panic!("{:?}", e)
    };

    let lines = file_contents.split_whitespace();

    let line_count = lines.clone().count();

    let first_line = match lines.clone().nth(0) {
        Some(l) => l,
        None => panic!("No line at index!")
    };

    let column_count = first_line.chars().count();

    let repeat_count = line_count / (column_count / 3);

    let tree = '#';

    let mut tree_count = 0;

    let mut line_count = 0;

    lines.map(|line| {
        (0..repeat_count).fold(line.to_owned(), |mut acc, _| {
            acc.push_str(line);
            acc.to_owned()
        })
    }).for_each(|line| {
        if line_count == 0 {
            false;
        } else {
            if match line.chars().nth(
                if line_count == 1 {
                    3
                } else {
                    3 * line_count
                }
            ) {
                Some(c) => c,
                None => panic!("No char here!")
            } == tree {
                tree_count += 1
            }
        }
        line_count += 1
    });

    println!("Line count is: {}", line_count);
    println!("Tree count is: {}", tree_count);
}

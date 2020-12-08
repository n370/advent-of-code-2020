use std::{ fs };

pub fn main() -> Option<bool> {
    let file_contents = fs::read_to_string(
        "./inputs/2020-12-07-aoc-01-input.txt"
    ).unwrap();

    let colors = recurse(
        file_contents.split('\n').collect::<Vec<&str>>(),
        vec!(String::from("shiny gold"))
    );

    println!("{:?}", colors.len() - 1);

    Some(true)
}

fn recurse(mut lines: Vec<&str>, mut colors: Vec<String>) -> Vec<String> {
    for (index, line) in lines.clone().iter().enumerate() {
        for color in colors.clone() {
            if line.contains(&color) && !line.starts_with(&color) {
                colors.push(
                    line.split(" bags contain ").nth(0).unwrap().to_string()
                );
                lines.remove(index);
                return recurse(lines, colors);
            }
        }
    }
    colors
}

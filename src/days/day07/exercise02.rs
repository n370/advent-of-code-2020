use std::{ fs };

pub fn main() -> Option<bool> {
    let file_contents = fs::read_to_string(
        "./inputs/2020-12-07-aoc-01-input.txt"
    ).unwrap();

    let bags = get_list_of_bags(
        file_contents.split('\n')
            .filter(|line| line.len() > 0)
            .collect::<Vec<&str>>()
    );

    let bag_tree = recurse(
        &bags,
        Bag {
            color: String::from("shiny gold"),
            count: 1,
            contents: vec!()
        }
    );

    println!("{:?}", bag_tree);

    let count = recurse_tree(bag_tree);

    println!("Count is: {}", count - 1);

    Some(true)
}

#[derive(Debug)]
pub struct Bag {
    color: String,
    count: usize,
    contents: Vec<Bag>
}

pub fn recurse_tree(bag_tree: Bag) -> usize {
    let mut contents: usize = 1;
    for bag in bag_tree.contents {
        contents += recurse_tree(bag);
    }
    bag_tree.count * contents
}

pub fn recurse(bags: &Vec<(String, Vec<(usize, String)>)>, mut bag_tree: Bag) -> Bag {
    for bag in bags {
        if bag.0 == bag_tree.color {
            for item in &bag.1 {
                bag_tree.contents.push(
                    recurse(
                        bags,
                        Bag {
                            color: item.1.to_owned(),
                            count: item.0,
                            contents: vec!()
                        }
                    )
                );
            }
        }
    }
    bag_tree
}

fn get_list_of_bags(lines: Vec<&str>) -> Vec<(String, Vec<(usize, String)>)> {
    lines.iter().map(|line| {
        let line_parts: Vec<&str> = line.split(" bags contain ").collect();
        let container = line_parts[0].to_string();
        let contents = if line_parts[1] == "no other bags." {
            vec!()
        } else {
            line_parts[1]
                .split(", ")
                .map(|segment| {
                    let segment_parts = segment
                        .split(' ')
                        .collect::<Vec<&str>>();
                    (
                        segment_parts[0]
                            .parse::<usize>()
                            .ok()
                            .unwrap(),
                        segment_parts[1..=2]
                            .join(" ")
                    )
                }).collect::<Vec<(usize, String)>>()
        };
        (container, contents)
    }).collect()
}

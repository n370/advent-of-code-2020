use std::{ fs };

pub fn main() -> Option<bool> {
    let file_contents = match fs::read_to_string(
        "./inputs/2020-12-06-aoc-01-input.txt"
    ) {
        Ok(c) => c,
        Err(e) => panic!("{:?}", e)
    };

    let total_sum = file_contents
        .split("\n\n")
        .map(|block| {
            block
                .split('\n')
                .fold(vec!(), |mut acc, curr| {
                    curr.chars().for_each(|c| {
                        if !acc.contains(&c) {
                            acc.push(c)
                        }
                    });
                    acc
                })
                .len()
        }).fold(0, |acc, curr| acc + curr);

    println!("The total sum is: {}", total_sum);

    Some(true)
}

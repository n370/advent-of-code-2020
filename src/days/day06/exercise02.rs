use std::{ fs };

pub fn main() -> Option<bool> {
    let file_contents = match fs::read_to_string(
        "./inputs/2020-12-06-aoc-01-input.txt"
    ) {
        Ok(c) => c,
        Err(e) => panic!("{:?}", e)
    };

    let total_sum: usize = file_contents
        .split("\n\n")
        .map(|block| {
            let mut ordered_block_answers = block
                .split('\n')
                .filter(|line| line.len() > 0)
                .map(|line| {
                    line
                        .chars()
                        .fold(vec!(), |mut a, c| {
                            a.push(c.clone());
                            a
                        })
                }).collect::<Vec<_>>();

            ordered_block_answers
                .sort_by(|a, b| a.len().cmp(&b.len()));

            let mut valid_answers = ordered_block_answers[0].clone();

            for block_answer in &ordered_block_answers {
                for valid_answer in &ordered_block_answers[0] {
                    if !block_answer.contains(&valid_answer) {
                        let index = match valid_answers.iter()
                            .position(|answer| answer == valid_answer) {
                                Some(index) => index,
                                None => continue
                            };
                        valid_answers.remove(index);
                    }
                }
            }

            valid_answers.len()
        })
        .sum();

    println!("The total sum is: {}", total_sum);

    Some(true)
}

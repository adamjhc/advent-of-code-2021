use utils;

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}

fn part_one() -> usize {
    utils::read_string("./input.txt")
        .lines()
        .fold(0, |syntax_error_score, line| {
            let mut stack = Vec::new();
            for character in line.chars() {
                match character {
                    '(' | '[' | '{' | '<' => stack.push(character),
                    ')' if stack.last().unwrap() == &'(' => {
                        stack.pop();
                    }
                    ']' if stack.last().unwrap() == &'[' => {
                        stack.pop();
                    }
                    '}' if stack.last().unwrap() == &'{' => {
                        stack.pop();
                    }
                    '>' if stack.last().unwrap() == &'<' => {
                        stack.pop();
                    }
                    _ => {
                        return syntax_error_score
                            + match character {
                                ')' => 3,
                                ']' => 57,
                                '}' => 1197,
                                '>' => 25137,
                                _ => panic!("Unexpected character"),
                            }
                    }
                }
            }

            syntax_error_score
        })
}

fn part_two() -> usize {
    let mut completion_scores = utils::read_string("./input.txt").lines().fold(
        Vec::new(),
        |mut completion_scores, line| {
            let mut stack = Vec::new();
            for character in line.chars() {
                match character {
                    '(' | '[' | '{' | '<' => stack.push(character),
                    ')' if stack.last().unwrap() == &'(' => {
                        stack.pop();
                    }
                    ']' if stack.last().unwrap() == &'[' => {
                        stack.pop();
                    }
                    '}' if stack.last().unwrap() == &'{' => {
                        stack.pop();
                    }
                    '>' if stack.last().unwrap() == &'<' => {
                        stack.pop();
                    }
                    _ => return completion_scores,
                }
            }

            let mut completion_score = 0;
            while let Some(character) = stack.pop() {
                completion_score *= 5;
                completion_score += match character {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => panic!("Unexpected character"),
                }
            }

            completion_scores.push(completion_score);
            completion_scores
        },
    );

    completion_scores.sort();
    completion_scores[completion_scores.len() / 2]
}

#[cfg(test)]
mod day_10_tests {
    use super::*;

    #[test]
    fn part_one_gives_correct_answer() {
        assert_eq!(part_one(), 392367)
    }

    #[test]
    fn part_two_gives_correct_answer() {
        assert_eq!(part_two(), 2192104158)
    }
}

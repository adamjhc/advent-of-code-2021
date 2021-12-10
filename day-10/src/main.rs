use utils;

fn main() {
    println!("{}", part_one())
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

#[cfg(test)]
mod day_10_tests {
    use super::*;

    #[test]
    fn part_one_gives_correct_answer() {
        assert_eq!(part_one(), 392367)
    }
}

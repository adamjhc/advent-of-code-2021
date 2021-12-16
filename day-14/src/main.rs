use std::collections::HashMap;

use utils;

fn main() {
    println!("{}", part_one())
}

fn part_one() -> usize {
    let (mut polymer_template, insertion_rules) = parse_input();

    for _ in 0..10 {
        let mut polymer_template_new = polymer_template.clone();
        let mut num_insertions = 0;

        let chars = polymer_template.chars().collect::<Vec<char>>();

        for i in 1..chars.len() {
            let pair: String = vec![chars[i - 1], chars[i]].iter().collect();

            if insertion_rules.contains_key(&pair) {
                let insertion = insertion_rules.get(&pair).unwrap();
                polymer_template_new.insert_str(i + num_insertions, &insertion);
                num_insertions += 1;
            }
        }

        polymer_template = polymer_template_new;
    }

    let counts = polymer_template
        .chars()
        .fold(HashMap::new(), |mut counts, character| {
            *counts.entry(character).or_insert(0) += 1;
            counts
        });

    let (high, low) = counts
        .iter()
        .fold((0, usize::MAX), |(high, low), (_, &count)| {
            if count > high {
                (count, low)
            } else if count < low {
                (high, count)
            } else {
                (high, low)
            }
        });

    high - low
}

fn parse_input() -> (String, HashMap<String, String>) {
    let input = utils::read_string("./input.txt");
    let mut lines = input.lines();

    let polymer_template = lines.next().unwrap().to_string();

    lines.next();

    let mut insertion_rules = HashMap::new();

    for line in lines {
        let split: Vec<&str> = line.split(" -> ").collect();

        insertion_rules.insert(split[0].to_string(), split[1].to_string());
    }

    (polymer_template, insertion_rules)
}

#[cfg(test)]
mod day_14_tests {
    use super::*;

    #[test]
    fn part_one_gives_correct_answer() {
        assert_eq!(part_one(), 2112)
    }
}

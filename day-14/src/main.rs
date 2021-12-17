use std::collections::HashMap;

use utils;

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
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

fn part_two() -> usize {
    let (polymer_template, insertion_rules) = parse_input();

    let pairs = polymer_template.chars().collect::<Vec<char>>();
    let mut counts_of_pairs = pairs.windows(2).fold(HashMap::new(), |mut counts, pair| {
        *counts.entry((pair[0], pair[1])).or_insert(0) += 1;
        counts
    });

    let insertion_rules: HashMap<(char, char), char> = insertion_rules
        .iter()
        .map(|(key, value)| {
            let characters: Vec<char> = key.chars().collect();

            (
                (characters[0], characters[1]),
                value.chars().next().unwrap(),
            )
        })
        .into_iter()
        .collect();

    for _ in 0..40 {
        let mut count_of_pairs_new = HashMap::new();

        for (pair, count) in counts_of_pairs {
            if let Some(&insertion) = insertion_rules.get(&pair) {
                *count_of_pairs_new.entry((pair.0, insertion)).or_insert(0) += count;
                *count_of_pairs_new.entry((insertion, pair.1)).or_insert(0) += count;
            } else {
                *count_of_pairs_new.entry(pair).or_insert(0) += count;
            }
        }

        counts_of_pairs = count_of_pairs_new;
    }

    // this will count characters in overlapping pairs twice
    let counts = counts_of_pairs
        .iter()
        .fold(HashMap::new(), |mut counts, (characters, count)| {
            *counts.entry(characters.0).or_insert(0) += count;
            *counts.entry(characters.1).or_insert(0) += count;
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

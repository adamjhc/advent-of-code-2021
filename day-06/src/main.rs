use utils;

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}

fn part_one() -> usize {
    let mut laternfish_ages = parse_input();

    for _ in 0..80 {
        let mut new_laternfish = 0;

        laternfish_ages.iter_mut().for_each(|age| {
            if *age == 0 {
                new_laternfish += 1;
                *age = 6;
            } else {
                *age -= 1;
            }
        });

        for _ in 0..new_laternfish {
            laternfish_ages.push(8);
        }
    }

    laternfish_ages.len()
}

fn part_two() -> usize {
    let laternfish_ages = parse_input();

    let mut num_ages = vec![0; 9];

    laternfish_ages.iter().for_each(|&age| num_ages[age] += 1);

    for _ in 0..256 {
        let mut new_ages = vec![0; 9];

        for age in 0..9 {
            if age == 0 {
                new_ages[6] += num_ages[0];
                new_ages[8] += num_ages[0];
            } else {
                new_ages[age - 1] += num_ages[age];
            }
        }

        num_ages = new_ages;
    }

    num_ages.iter().sum()
}

fn parse_input() -> Vec<usize> {
    utils::read_string("./input.txt")
        .split(",")
        .filter_map(|num| num.parse().ok())
        .collect()
}

#[cfg(test)]
mod day_06_tests {
    use super::*;

    #[test]
    fn part_one_gives_correct_answer() {
        assert_eq!(part_one(), 371379)
    }

    #[test]
    fn part_two_gives_correct_answer() {
        assert_eq!(part_two(), 1674303997472)
    }
}

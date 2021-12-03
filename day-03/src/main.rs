use utils;

fn main() {
    println!("{}", part_one())
}

fn part_one() -> usize {
    let input = utils::read_string("./input.txt");
    let half_length = input.lines().count() as u32 / 2;

    let most_common = input.lines().fold(vec![0; 12], |mut counts, binary| {
        for i in 0..counts.len() {
            counts[i] += binary.chars().nth(i).unwrap().to_digit(10).unwrap();
        }

        counts
    });

    let gamma_rate = most_common.iter().fold(0, |mut gamma_rate, &b| {
        gamma_rate <<= 1;

        if b > half_length {
            gamma_rate += 1;
        }

        gamma_rate
    });

    let epsilon_rate = gamma_rate ^ 0b1111_1111_1111;

    gamma_rate * epsilon_rate
}

#[cfg(test)]
mod day_03_tests {
    use super::*;

    #[test]
    fn part_one_gives_correct_answer() {
        assert_eq!(part_one(), 2640986)
    }
}

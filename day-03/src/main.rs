use utils;

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
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

fn part_two() -> u32 {
    let input = utils::read_string("./input.txt");
    let half_length = input.lines().count() as u32 / 2;

    let binary_digits: Vec<Vec<u32>> = input
        .lines()
        .map(|binary_string| {
            binary_string
                .chars()
                .map(|binary_digit| binary_digit.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let mut ogr_considerations = binary_digits.clone();
    let mut index = 0;
    loop {
        let sum = ogr_considerations
            .iter()
            .fold(0, |sum, binary| sum + binary[index]);

        let most_common = if sum >= half_length { 1 } else { 0 };

        ogr_considerations = ogr_considerations
            .iter()
            .filter(|&binary| binary[index] == most_common)
            .map(Vec::to_owned)
            .collect();

        if ogr_considerations.len() == 1 {
            break;
        }

        index += 1;
    }

    let mut csr_considerations = binary_digits.clone();
    let mut index = 0;
    loop {
        let sum = csr_considerations
            .iter()
            .fold(0, |sum, binary| sum + binary[index]);

        let most_common = if sum >= half_length { 0 } else { 1 };

        csr_considerations = csr_considerations
            .iter()
            .filter(|&binary| binary[index] == most_common)
            .map(Vec::to_owned)
            .collect();

        if csr_considerations.len() == 1 {
            break;
        }

        index += 1;
    }

    let ogr: u32 = ogr_considerations[0]
        .iter()
        .fold(0, |mut binary, binary_digit| {
            binary <<= 1;
            binary + binary_digit
        });

    let csr: u32 = csr_considerations[0]
        .iter()
        .fold(0, |mut binary, binary_digit| {
            binary <<= 1;
            binary + binary_digit
        });

    ogr * csr
}

#[cfg(test)]
mod day_03_tests {
    use super::*;

    #[test]
    fn part_one_gives_correct_answer() {
        assert_eq!(part_one(), 2640986)
    }
}

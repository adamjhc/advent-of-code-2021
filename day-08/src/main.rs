use utils;

fn main() {
    println!("{}", part_one())
}

fn part_one() -> usize {
    let input = parse_input();

    for (mut signals, output) in input {
        signals.sort_by(|a, b| a.len().cmp(&b.len()));
    }

    0
}

fn parse_input() -> Vec<(Vec<String>, Vec<String>)> {
    utils::read_string("./input.txt")
        .lines()
        .map(|line| {
            let input: Vec<Vec<String>> = line
                .split(" | ")
                .map(|input| input.split(" ").map(|str| str.to_owned()).collect())
                .collect();

            (input[0].clone(), input[1].clone())
        })
        .collect()
}

use utils;

fn main() {
    println!("{}", part_1());
    println!("{}", part_2());
}

fn part_1() -> u32 {
    let lines = utils::read_lines("./input.txt");

    let mut increases = 0u32;
    let mut previous_depth = u32::MAX;
    for line in lines {
        if let Ok(depth) = line {
            let depth_int = depth.parse().unwrap();
            if depth_int > previous_depth {
                increases += 1;
            }

            previous_depth = depth_int;
        }
    }

    increases
}

fn part_2() -> u32 {
    let depths: Vec<u32> = utils::read_lines("./input.txt")
        .map(|depth_str| depth_str.unwrap().parse().unwrap())
        .collect();

    let mut increases = 0u32;
    let mut previous_depth = u32::MAX;
    for i in 2..depths.len() {
        let depth_sliding_window = depths[i] + depths[i - 1] + depths[i - 2];
        if depth_sliding_window > previous_depth {
            increases += 1;
        }

        previous_depth = depth_sliding_window;
    }

    increases
}

#[cfg(test)]
mod day_01_tests {
    use super::*;

    #[test]
    fn part_1_gives_correct_answer() {
        assert_eq!(part_1(), 1233)
    }

    #[test]
    fn part_2_gives_correct_answer() {
        assert_eq!(part_2(), 1275)
    }
}

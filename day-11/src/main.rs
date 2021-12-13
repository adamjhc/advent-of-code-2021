use utils;

fn main() {
    println!("{}", part_one())
}

fn part_one() -> usize {
    let mut energy_levels = parse_input();

    let mut total_flashes = 0;

    for _ in 0..100 {
        energy_levels
            .iter_mut()
            .for_each(|row| row.iter_mut().for_each(|(energy, _)| *energy += 1));

        loop {
            let mut flashes = 0;

            let max_row = energy_levels.len();
            for row in 0..max_row {
                let max_col = energy_levels[row].len();
                for col in 0..max_col {
                    let (energy, has_flashed) = energy_levels[row][col];

                    if energy > 9 && !has_flashed {
                        let adjacent_locations = get_adjacent_locations(row, col, max_row, max_col);

                        adjacent_locations
                            .iter()
                            .for_each(|&(row, col)| energy_levels[row][col].0 += 1);

                        energy_levels[row][col] = (energy, true);
                        flashes += 1;
                    }
                }
            }

            total_flashes += flashes;

            if flashes == 0 {
                break;
            }
        }

        energy_levels.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|(energy, has_flashed)| {
                if *has_flashed {
                    *energy = 0;
                    *has_flashed = false;
                }
            })
        });
    }

    total_flashes
}

fn parse_input() -> Vec<Vec<(u32, bool)>> {
    utils::read_string("./input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|energy| (energy.to_digit(10).unwrap(), false))
                .collect()
        })
        .collect()
}

fn get_adjacent_locations(
    row: usize,
    col: usize,
    max_row: usize,
    max_col: usize,
) -> Vec<(usize, usize)> {
    let mut adjacent_locations = Vec::new();

    if row > 0 {
        adjacent_locations.push((row - 1, col));
    }
    if row < max_row - 1 {
        adjacent_locations.push((row + 1, col));
    }
    if col > 0 {
        adjacent_locations.push((row, col - 1));
    }
    if col < max_col - 1 {
        adjacent_locations.push((row, col + 1));
    }
    if row > 0 && col > 0 {
        adjacent_locations.push((row - 1, col - 1));
    }
    if row > 0 && col < max_col - 1 {
        adjacent_locations.push((row - 1, col + 1));
    }
    if row < max_row - 1 && col > 0 {
        adjacent_locations.push((row + 1, col - 1));
    }
    if row < max_row - 1 && col < max_col - 1 {
        adjacent_locations.push((row + 1, col + 1));
    }

    adjacent_locations
}

#[cfg(test)]
mod day_11_tests {
    use super::*;

    #[test]
    fn part_one_gives_correct_answer() {
        assert_eq!(part_one(), 1713)
    }
}

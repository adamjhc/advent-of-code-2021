use std::collections::HashMap;

use utils;

#[derive(Clone)]
struct Line {
    from: Point,
    to: Point,
    segments: Vec<Point>,
}

impl Line {
    fn new(from: Point, to: Point) -> Line {
        let mut segments = Vec::new();

        let min_x = from.x.min(to.x);
        let min_y = from.y.min(to.y);
        let max_x = from.x.max(to.x);
        let max_y = from.y.max(to.y);

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                segments.push(Point { x, y })
            }
        }

        Line { from, to, segments }
    }

    fn is_orthogonal(&self) -> bool {
        self.from.y == self.to.y || self.from.x == self.to.x
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

fn main() {
    println!("{}", part_one())
}

fn part_one() -> usize {
    let lines = parse_input();

    let orthogonal_lines: Vec<&Line> = lines.iter().filter(|&line| line.is_orthogonal()).collect();

    let mut points: HashMap<Point, usize> = HashMap::new();
    orthogonal_lines.iter().for_each(|&line| {
        line.segments.iter().for_each(|&point| {
            if points.contains_key(&point) {
                *points.get_mut(&point).unwrap() += 1;
            } else {
                points.insert(point, 1);
            }
        })
    });

    points.iter().filter(|(_, &count)| count >= 2).count()
}

fn parse_input() -> Vec<Line> {
    utils::read_lines("./input.txt")
        .map(Result::unwrap)
        .map(|line| {
            let points_split = line.split(" -> ");
            let points: Vec<Point> = points_split
                .map(|point_input| {
                    let numbers: Vec<usize> = point_input
                        .split(",")
                        .map(|num| num.parse().unwrap())
                        .collect();
                    Point {
                        x: numbers[0],
                        y: numbers[1],
                    }
                })
                .collect();

            Line::new(points[0], points[1])
        })
        .collect()
}

#[cfg(test)]
mod day_05_tests {
    use super::*;

    #[test]
    fn part_one_gives_correct_answer() {
        assert_eq!(part_one(), 8350)
    }
}

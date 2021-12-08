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

        let range_x: Vec<usize> = if from.x < to.x {
            (from.x..=to.x).collect()
        } else {
            (to.x..=from.x).rev().collect()
        };

        let range_y: Vec<usize> = if from.y < to.y {
            (from.y..=to.y).collect()
        } else {
            (to.y..=from.y).rev().collect()
        };

        if from.y == to.y || from.x == to.x {
            for x in range_x {
                for y in &range_y {
                    segments.push(Point { x, y: *y })
                }
            }
        } else {
            for (&x, y) in range_x.iter().zip(range_y) {
                segments.push(Point { x, y })
            }
        }

        Line { from, to, segments }
    }

    fn is_orthogonal(&self) -> bool {
        self.from.y == self.to.y || self.from.x == self.to.x
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
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

fn part_two() -> usize {
    let lines = parse_input();

    let mut points: HashMap<Point, usize> = HashMap::new();
    lines.iter().for_each(|line| {
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

    #[test]
    fn part_two_gives_correct_answer() {
        assert_eq!(part_two(), 19374)
    }

    #[test]
    fn line_segments_horizontal_to_right() {
        let line = Line::new(Point { x: 0, y: 3 }, Point { x: 5, y: 3 });

        assert_eq!(
            line.segments,
            vec![
                Point { x: 0, y: 3 },
                Point { x: 1, y: 3 },
                Point { x: 2, y: 3 },
                Point { x: 3, y: 3 },
                Point { x: 4, y: 3 },
                Point { x: 5, y: 3 },
            ]
        )
    }

    #[test]
    fn line_segments_horizontal_to_left() {
        let line = Line::new(Point { x: 5, y: 3 }, Point { x: 0, y: 3 });

        assert_eq!(
            line.segments,
            vec![
                Point { x: 5, y: 3 },
                Point { x: 4, y: 3 },
                Point { x: 3, y: 3 },
                Point { x: 2, y: 3 },
                Point { x: 1, y: 3 },
                Point { x: 0, y: 3 },
            ]
        )
    }

    #[test]
    fn line_segments_vertical_up() {
        let line = Line::new(Point { x: 3, y: 0 }, Point { x: 3, y: 5 });

        assert_eq!(
            line.segments,
            vec![
                Point { x: 3, y: 0 },
                Point { x: 3, y: 1 },
                Point { x: 3, y: 2 },
                Point { x: 3, y: 3 },
                Point { x: 3, y: 4 },
                Point { x: 3, y: 5 },
            ]
        )
    }

    #[test]
    fn line_segments_vertical_down() {
        let line = Line::new(Point { x: 3, y: 5 }, Point { x: 3, y: 0 });

        assert_eq!(
            line.segments,
            vec![
                Point { x: 3, y: 5 },
                Point { x: 3, y: 4 },
                Point { x: 3, y: 3 },
                Point { x: 3, y: 2 },
                Point { x: 3, y: 1 },
                Point { x: 3, y: 0 },
            ]
        )
    }

    #[test]
    fn line_segments_diagonal_up_right() {
        let line = Line::new(Point { x: 0, y: 3 }, Point { x: 3, y: 6 });

        assert_eq!(
            line.segments,
            vec![
                Point { x: 0, y: 3 },
                Point { x: 1, y: 4 },
                Point { x: 2, y: 5 },
                Point { x: 3, y: 6 },
            ]
        )
    }

    #[test]
    fn line_segments_diagonal_down_right() {
        let line = Line::new(Point { x: 0, y: 6 }, Point { x: 3, y: 3 });

        assert_eq!(
            line.segments,
            vec![
                Point { x: 0, y: 6 },
                Point { x: 1, y: 5 },
                Point { x: 2, y: 4 },
                Point { x: 3, y: 3 },
            ]
        )
    }

    #[test]
    fn line_segments_diagonal_up_left() {
        let line = Line::new(Point { x: 3, y: 3 }, Point { x: 0, y: 6 });

        assert_eq!(
            line.segments,
            vec![
                Point { x: 3, y: 3 },
                Point { x: 2, y: 4 },
                Point { x: 1, y: 5 },
                Point { x: 0, y: 6 },
            ]
        )
    }

    #[test]
    fn line_segments_diagonal_down_left() {
        let line = Line::new(Point { x: 3, y: 6 }, Point { x: 0, y: 3 });

        assert_eq!(
            line.segments,
            vec![
                Point { x: 3, y: 6 },
                Point { x: 2, y: 5 },
                Point { x: 1, y: 4 },
                Point { x: 0, y: 3 },
            ]
        )
    }
}

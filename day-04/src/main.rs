use utils;

#[derive(PartialEq, Debug)]
struct BingoBoard {
    board: Vec<Vec<BoardValue>>,
}

impl BingoBoard {
    fn new() -> BingoBoard {
        BingoBoard { board: Vec::new() }
    }

    fn fill_row(&mut self, values: Vec<usize>) {
        self.board
            .push(values.iter().map(|&value| BoardValue::new(value)).collect())
    }

    fn mark_number(&mut self, number: usize) {
        self.board.iter_mut().for_each(|board_row| {
            board_row.iter_mut().for_each(|board_value| {
                if board_value.value == number {
                    board_value.set_marked()
                }
            })
        })
    }

    fn is_winner(&self) -> bool {
        if self
            .board
            .iter()
            .any(|row| row.iter().all(BoardValue::is_marked))
        {
            return true;
        }

        for col in 0..self.board[0].len() {
            if (0..self.board.len()).all(|row| self.board[row][col].is_marked()) {
                return true;
            }
        }

        false
    }

    fn sum_unmarked_numbers(&self) -> usize {
        self.board.iter().fold(0, |sum, row| {
            sum + row.iter().fold(0, |sum_row, board_value| {
                if board_value.is_marked() {
                    sum_row
                } else {
                    sum_row + board_value.value
                }
            })
        })
    }
}

#[derive(PartialEq, Debug)]
struct BoardValue {
    value: usize,
    marked: bool,
}

impl BoardValue {
    fn new(value: usize) -> BoardValue {
        BoardValue {
            value,
            marked: false,
        }
    }

    fn set_marked(&mut self) {
        self.marked = true;
    }

    fn is_marked(&self) -> bool {
        self.marked
    }
}

fn main() {
    println!("{}", part_one());
    println!("{}", part_two());
}

fn part_one() -> usize {
    let (numbers, mut bingo_boards) = parse_input();

    for number in numbers {
        bingo_boards
            .iter_mut()
            .for_each(|bingo_board| bingo_board.mark_number(number));

        for bingo_board in &bingo_boards {
            if bingo_board.is_winner() {
                return bingo_board.sum_unmarked_numbers() * number;
            }
        }
    }

    0
}

fn part_two() -> usize {
    let (numbers, mut bingo_boards) = parse_input();

    let mut last_winning_score = 0usize;
    for number in numbers {
        bingo_boards
            .iter_mut()
            .for_each(|bingo_board| bingo_board.mark_number(number));

        let mut num_removed = 0;
        for i in 0..bingo_boards.len() {
            let index = i - num_removed;
            if bingo_boards[index].is_winner() {
                last_winning_score = bingo_boards[index].sum_unmarked_numbers() * number;
                bingo_boards.swap_remove(index);
                num_removed += 1;
            }
        }
    }

    last_winning_score
}

fn parse_input() -> (Vec<usize>, Vec<BingoBoard>) {
    let input = utils::read_string("./input.txt");
    let mut sections = input.split("\n\n");

    let numbers: Vec<usize> = sections
        .next()
        .unwrap()
        .split(",")
        .filter_map(|n| n.parse().ok())
        .collect();

    let mut bingo_boards: Vec<BingoBoard> = Vec::new();
    while let Some(board_input) = sections.next() {
        let mut board: BingoBoard = BingoBoard::new();
        board_input.split("\n").for_each(|row| {
            board.fill_row(row.split(" ").filter_map(|n| n.parse().ok()).collect())
        });

        bingo_boards.push(board);
    }

    (numbers, bingo_boards)
}

#[cfg(test)]
mod day_04_tests {
    use super::*;

    #[test]
    fn part_one_gives_correct_answer() {
        assert_eq!(part_one(), 34506)
    }

    #[test]
    fn part_two_gives_correct_answer() {
        assert_eq!(part_two(), 7686)
    }

    #[test]
    fn bingo_board_fills_row() {
        let mut bingo_board = BingoBoard::new();

        bingo_board.fill_row(vec![1, 2, 3, 4, 5]);

        assert_eq!(
            bingo_board,
            BingoBoard {
                board: vec![vec![
                    BoardValue {
                        value: 1,
                        marked: false
                    },
                    BoardValue {
                        value: 2,
                        marked: false
                    },
                    BoardValue {
                        value: 3,
                        marked: false
                    },
                    BoardValue {
                        value: 4,
                        marked: false
                    },
                    BoardValue {
                        value: 5,
                        marked: false
                    }
                ]]
            }
        )
    }

    #[test]
    fn bingo_board_marks_number() {
        let mut bingo_board = BingoBoard::new();
        bingo_board.fill_row(vec![1, 2, 3, 4, 5]);
        bingo_board.mark_number(4);

        assert_eq!(
            bingo_board,
            BingoBoard {
                board: vec![vec![
                    BoardValue {
                        value: 1,
                        marked: false
                    },
                    BoardValue {
                        value: 2,
                        marked: false
                    },
                    BoardValue {
                        value: 3,
                        marked: false
                    },
                    BoardValue {
                        value: 4,
                        marked: true
                    },
                    BoardValue {
                        value: 5,
                        marked: false
                    }
                ]]
            }
        )
    }

    #[test]
    fn bingo_board_marks_numbers() {
        let mut bingo_board = BingoBoard::new();
        bingo_board.fill_row(vec![1, 2, 3, 4, 5]);
        bingo_board.fill_row(vec![5, 4, 3, 2, 1]);
        bingo_board.mark_number(4);

        assert_eq!(
            bingo_board,
            BingoBoard {
                board: vec![
                    vec![
                        BoardValue {
                            value: 1,
                            marked: false
                        },
                        BoardValue {
                            value: 2,
                            marked: false
                        },
                        BoardValue {
                            value: 3,
                            marked: false
                        },
                        BoardValue {
                            value: 4,
                            marked: true
                        },
                        BoardValue {
                            value: 5,
                            marked: false
                        }
                    ],
                    vec![
                        BoardValue {
                            value: 5,
                            marked: false
                        },
                        BoardValue {
                            value: 4,
                            marked: true
                        },
                        BoardValue {
                            value: 3,
                            marked: false
                        },
                        BoardValue {
                            value: 2,
                            marked: false
                        },
                        BoardValue {
                            value: 1,
                            marked: false
                        }
                    ]
                ]
            }
        )
    }

    #[test]
    fn bingo_board_is_winner_column() {
        let mut bingo_board = BingoBoard::new();
        bingo_board.fill_row(vec![1, 3, 3, 4, 5]);
        bingo_board.fill_row(vec![2, 3, 4, 4, 5]);
        bingo_board.fill_row(vec![3, 4, 5, 4, 5]);
        bingo_board.fill_row(vec![4, 5, 1, 4, 5]);
        bingo_board.fill_row(vec![5, 1, 2, 4, 5]);

        bingo_board.mark_number(1);
        bingo_board.mark_number(2);
        bingo_board.mark_number(3);
        bingo_board.mark_number(4);
        bingo_board.mark_number(5);

        assert!(bingo_board.is_winner())
    }

    #[test]
    fn bingo_board_is_winner_row() {
        let mut bingo_board = BingoBoard::new();
        bingo_board.fill_row(vec![1, 2, 3, 4, 5]);

        bingo_board.mark_number(1);
        bingo_board.mark_number(2);
        bingo_board.mark_number(3);
        bingo_board.mark_number(4);
        bingo_board.mark_number(5);

        assert!(bingo_board.is_winner())
    }

    #[test]
    fn bingo_board_is_not_winner() {
        let mut bingo_board = BingoBoard::new();
        bingo_board.fill_row(vec![1, 2, 3, 4, 5]);

        assert!(!bingo_board.is_winner())
    }

    #[test]
    fn bingo_board_sum_marked_numbers() {
        let mut bingo_board = BingoBoard::new();
        bingo_board.fill_row(vec![1, 2, 3, 4, 5]);
        bingo_board.fill_row(vec![5, 4, 3, 2, 1]);
        bingo_board.fill_row(vec![1, 1, 1, 1, 1]);

        bingo_board.mark_number(1);

        assert_eq!(bingo_board.sum_unmarked_numbers(), 28)
    }

    #[test]
    fn board_value_is_set_as_marked() {
        let mut board_value = BoardValue::new(2);

        board_value.set_marked();

        assert_eq!(
            board_value,
            BoardValue {
                value: 2,
                marked: true
            }
        );
    }
}

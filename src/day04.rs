use itertools::Itertools;
use std::fs;

const N_POSITIONS: usize = 25;

pub fn main() {
    println!("04a: {}", part_1("./input/day04/test.txt"));
    println!("04b: {}", part_2("./input/day04/actual.txt"));
}

#[derive(Debug, Copy, Clone)]
struct Position {
    scratched: bool,
    value: usize,
}

impl From<usize> for Position {
    fn from(item: usize) -> Self {
        Position {
            scratched: false,
            value: item,
        }
    }
}

#[derive(Copy, Clone)]
struct Board {
    board: [Position; N_POSITIONS],
    has_won: bool,
}

impl Board {
    fn from_input(input: String) -> Board {
        let board: [Position; N_POSITIONS] = input
            .split(" ")
            .flat_map(|el| el.parse::<usize>())
            .map(Position::from)
            .collect::<Vec<Position>>()
            .try_into()
            .unwrap();

        Board {
            board,
            has_won: false,
        }
    }

    fn scratch(&mut self, position: usize) {
        match Board::get_position(self, position) {
            Some(pos) => {
                pos.scratched = true;
                self.has_won = self.has_won();
            }
            None => (),
        }
    }

    fn get_position<'a>(&'a mut self, position: usize) -> Option<&'a mut Position> {
        self.board.iter_mut().filter(|p| p.value == position).next()
    }

    fn has_won(&self) -> bool {
        let columns = (0..5).into_iter().any(|col| Board::has_column(&self, col));
        let rows = (0..5).into_iter().any(|col| Board::has_row(&self, col));

        columns || rows
    }

    fn has_column(&self, idx: usize) -> bool {
        (0..5)
            .into_iter()
            .all(|col| self.board[col + idx * 5].scratched)
    }

    fn has_row(&self, idx: usize) -> bool {
        (0..5)
            .into_iter()
            .all(|row| self.board[row * 5 + idx].scratched)
    }

    fn sum_unchecked(self) -> usize {
        self.board
            .into_iter()
            .filter(|pos| !pos.scratched)
            .map(|pos| pos.value)
            .sum()
    }
}

fn play_bingo(draws: Vec<usize>, mut boards: Vec<Board>) -> impl Iterator<Item = usize> {
    let res = draws.into_iter().flat_map(move |draw| -> Vec<usize> {
        boards
            .iter_mut()
            .filter_map(|board| -> Option<usize> {
                if board.has_won {
                    None
                } else {
                    board.scratch(draw);
                    if board.has_won {
                        let sum = board.sum_unchecked();
                        Some(draw * sum)
                    } else {
                        None
                    }
                }
            })
            .collect()
    });

    res
}

fn part_1(path: &str) -> usize {
    let (draws, boards) = get_inputs(path);
    play_bingo(draws, boards).next().unwrap()
}

fn part_2(path: &str) -> usize {
    let (draws, boards) = get_inputs(path);
    play_bingo(draws, boards).last().unwrap()
}

fn get_inputs(path: &str) -> (Vec<usize>, Vec<Board>) {
    let file = fs::read_to_string(path).expect(&*format!("Could not find {}", path));
    let draws: Vec<usize> = file.lines().take(1).collect::<Vec<&str>>()[0]
        .to_owned()
        .split(",")
        .filter_map(|el| el.parse().ok())
        .collect();

    let boards: Vec<Board> = file
        .lines()
        .skip(2)
        .collect::<Vec<&str>>()
        .chunks(6)
        .map(|chunk| Board::from_input(chunk.iter().join(" ")))
        .collect();

    (draws, boards)
}

#[cfg(test)]
mod tests {
    use crate::day04::*;

    #[test]
    fn testcase_a() {
        assert_eq!(part_1("./input/day04/test.txt"), 4512)
    }

    #[test]
    fn actual_a() {
        assert_eq!(part_1("./input/day04/actual.txt"), 64084)
    }

    #[test]
    fn actual_b() {
        assert_eq!(part_2("./input/day04/actual.txt"), 12833)
    }

    #[test]
    fn testcase_b() {
        assert_eq!(part_2("./input/day04/test.txt"), 1924)
    }
}

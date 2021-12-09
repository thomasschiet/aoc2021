use itertools::Itertools;
use std::fs;

pub fn main() {
    println!("08a: {}", part_1("./input/day08/actual.txt"));
    println!("08b: {}", part_2("./input/day08/actual.txt"));
}

#[derive(Clone)]
struct Display {
    input: Vec<String>,
    output: Vec<String>,
}

impl From<&str> for Display {
    fn from(line: &str) -> Self {
        let s = line.to_string();
        let (input, output) = s.split_once(" | ").unwrap();
        Display {
            input: input
                .to_string()
                .split(" ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
            output: output
                .to_string()
                .split(" ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        }
    }
}

fn part_1(path: &str) -> usize {
    let file = fs::read_to_string(path).expect(&*format!("Could not find {}", path));
    let displays: Vec<Display> = file.lines().map(Display::from).collect();
    displays
        .iter()
        .flat_map(|display| {
            display.output.iter().filter(|word| {
                word.len() == 2 || word.len() == 4 || word.len() == 3 || word.len() == 7
            })
        })
        .count()
}

fn part_2(path: &str) -> usize {
    let file = fs::read_to_string(path).expect(&*format!("Could not find {}", path));
    let displays: Vec<Display> = file.lines().map(Display::from).collect();
    displays
        .into_iter()
        .map(|display| {
            let permutation = get_permutation(&display.input);
            let output_interpreted: String = display
                .output
                .iter()
                .map(|number| permute(number, &permutation))
                .map(interpret)
                .map(|o| format!("{}", o.unwrap()))
                .collect();

            output_interpreted.parse::<usize>().unwrap()
        })
        .sum()
}

fn get_permutation(input: &Vec<String>) -> Vec<char> {
    let base = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    let len = base.len();

    base.into_iter()
        .permutations(len)
        .find(|permutation| {
            input
                .iter()
                .map(|number| permute(number, &permutation))
                .map(interpret)
                .all(|x| x.is_some())
        })
        .unwrap()
}

fn permute(number: &String, permutation: &Vec<char>) -> String {
    let mut permuted: Vec<char> = number
        .chars()
        .map(|c| match c {
            'a' => permutation[0],
            'b' => permutation[1],
            'c' => permutation[2],
            'd' => permutation[3],
            'e' => permutation[4],
            'f' => permutation[5],
            _ => permutation[6],
        })
        .collect();

    permuted.sort_by(|a, b| a.cmp(b));
    String::from_iter(permuted)
}

fn interpret(input: String) -> Option<u8> {
    match input.as_str() {
        "abcefg" => Some(0),
        "cf" => Some(1),
        "acdeg" => Some(2),
        "acdfg" => Some(3),
        "bcdf" => Some(4),
        "abdfg" => Some(5),
        "abdefg" => Some(6),
        "acf" => Some(7),
        "abcdefg" => Some(8),
        "abcdfg" => Some(9),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::day08::*;

    #[test]
    fn testcase_a() {
        assert_eq!(part_1("./input/day08/test.txt"), 26)
    }

    #[test]
    fn actual_a() {
        assert_eq!(part_1("./input/day08/actual.txt"), 412);
    }

    #[test]
    fn actual_b() {
        assert_eq!(part_2("./input/day08/actual.txt"), 978171);
    }

    #[test]
    fn testcase_b() {
        assert_eq!(part_2("./input/day08/test.txt"), 61229)
    }
}

use std::fs;

enum Instruction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

pub fn main() {
    let input = parse("./input/day02/actual.txt");

    println!("02a: {}", to_answer_a(sail(&input)));
    println!("02b: {}", to_answer_b(sail_with_aim(&input)));
}

fn sail(instructions: &Vec<Instruction>) -> (i32, i32) {
    instructions.iter().fold(
        (0, 0),
        |(horizontal, depth), instruction| match instruction {
            Instruction::Forward(x) => (horizontal + x, depth),
            Instruction::Down(x) => (horizontal, depth + x),
            Instruction::Up(x) => (horizontal, depth - x),
        },
    )
}

fn sail_with_aim(instructions: &Vec<Instruction>) -> (i32, i32, i32) {
    instructions.iter().fold(
        (0, 0, 0),
        |(horizontal, depth, aim), instruction| match instruction {
            Instruction::Down(x) => (horizontal, depth, aim + x),
            Instruction::Up(x) => (horizontal, depth, aim - x),
            Instruction::Forward(x) => (horizontal + x, depth + aim * x, aim),
        },
    )
}

fn to_answer_a(position: (i32, i32)) -> i32 {
    position.0 * position.1
}

fn to_answer_b(position: (i32, i32, i32)) -> i32 {
    position.0 * position.1
}

fn parse(path: &str) -> Vec<Instruction> {
    fs::read_to_string(path)
        .expect(&*format!("Could not find {}", path))
        .lines()
        .map(to_instruction)
        .collect()
}

fn to_instruction(line: &str) -> Instruction {
    let mut iter = line.split_whitespace();

    let instruction = iter.next().unwrap();
    let val = iter.next().unwrap().parse::<i32>().unwrap();

    match instruction {
        "forward" => Instruction::Forward(val),
        "down" => Instruction::Down(val),
        "up" => Instruction::Up(val),
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::day02::*;

    #[test]
    fn testcase_a() {
        let input = parse("./input/day02/test.txt");
        let end_pos = sail(&input);
        assert_eq!(to_answer_a(end_pos), 150);
    }

    #[test]
    fn testcase_b() {
        let input = parse("./input/day02/test.txt");
        let end_pos = sail_with_aim(&input);
        assert_eq!(to_answer_b(end_pos), 900);
    }

    #[test]
    fn actual_a() {
        let input = parse("./input/day02/actual.txt");
        let end_pos = sail(&input);
        assert_eq!(to_answer_a(end_pos), 1804520);
    }

    #[test]
    fn actual_b() {
        let input = parse("./input/day02/actual.txt");
        let end_pos = sail_with_aim(&input);
        assert_eq!(to_answer_b(end_pos), 1971095320);
    }
}
